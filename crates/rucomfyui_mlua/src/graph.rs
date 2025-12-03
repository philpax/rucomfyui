//! Graph builder for ComfyUI workflows.

use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
};

use mlua::prelude::*;
use rucomfyui::{
    object_info::{Object, ObjectInfo, ObjectType},
    workflow::{Workflow, WorkflowInput, WorkflowMeta, WorkflowNode, WorkflowNodeId},
};

use crate::{
    conversion::lua_to_workflow_input,
    error::Error,
    node_output::{NodeOutput, NodeOutputValue, NodeOutputs},
};

/// A workflow graph builder.
///
/// This is the main type for building ComfyUI workflows in Lua.
/// Node types are exposed as methods via the `__index` metamethod.
pub struct Graph {
    /// The workflow being built.
    workflow: RefCell<Workflow>,
    /// The next node ID to assign.
    next_id: RefCell<u32>,
    /// Object info for node definitions.
    object_info: ObjectInfo,
}

impl Graph {
    /// Create a new graph from object info.
    #[allow(clippy::new_ret_no_self)]
    pub fn new(lua: &Lua, object_info_table: LuaTable) -> LuaResult<LuaAnyUserData> {
        let object_info = parse_object_info(lua, object_info_table)?;

        lua.create_userdata(Self {
            workflow: RefCell::new(Workflow::default()),
            next_id: RefCell::new(0),
            object_info,
        })
    }

    /// Add a node to the graph.
    fn add_node(&self, node: WorkflowNode) -> WorkflowNodeId {
        let mut next_id = self.next_id.borrow_mut();
        *next_id += 1;
        let id = WorkflowNodeId(*next_id);
        self.workflow.borrow_mut().0.insert(id, node);
        id
    }

    /// Get the object definition for a node type.
    fn get_object(&self, name: &str) -> Option<&Object> {
        self.object_info.get(name)
    }

    /// Convert the graph to a workflow.
    pub fn to_workflow(&self) -> Workflow {
        self.workflow.borrow().clone()
    }
}

impl LuaUserData for Graph {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        // Get the underlying workflow as a Lua-compatible table
        methods.add_method("_get_workflow", |lua, this, ()| {
            let workflow = this.to_workflow();
            lua.to_value(&workflow)
        });

        // The main magic: __index returns a node constructor for any node type
        methods.add_meta_method(LuaMetaMethod::Index, |lua, this, key: String| {
            // Check if it's a known node type
            if let Some(object) = this.get_object(&key) {
                // Return a constructor function for this node type
                let object = object.clone();
                let constructor = lua.create_function(move |lua, args: LuaMultiValue| {
                    create_node(lua, &object, args)
                })?;
                Ok(LuaValue::Function(constructor))
            } else {
                // Unknown node type - return nil (let Lua handle the error)
                Ok(LuaValue::Nil)
            }
        });
    }
}

/// Create a node from Lua arguments.
fn create_node(lua: &Lua, object: &Object, args: LuaMultiValue) -> LuaResult<LuaValue> {
    // Get the graph from the first argument (self)
    let mut args_iter = args.into_iter();
    let graph_ud = args_iter
        .next()
        .ok_or_else(|| LuaError::external("Expected graph as first argument"))?;

    let graph = match &graph_ud {
        LuaValue::UserData(ud) => ud.borrow::<Graph>()?,
        _ => return Err(LuaError::external("Expected graph as first argument")),
    };

    // Parse the remaining arguments into inputs
    let inputs = parse_node_inputs(lua, object, args_iter.collect())?;

    // Create the workflow node
    let node = WorkflowNode {
        inputs,
        class_type: object.name.clone(),
        meta: Some(WorkflowMeta::new(object.display_name())),
    };

    // Add to graph and create output
    let node_id = graph.add_node(node);
    let output = create_node_output(object, node_id);

    output.into_lua(lua)
}

/// Parse Lua arguments into workflow inputs.
fn parse_node_inputs(
    _lua: &Lua,
    object: &Object,
    args: Vec<LuaValue>,
) -> LuaResult<HashMap<String, WorkflowInput>> {
    let mut inputs = HashMap::new();

    // Collect input names in order
    let input_names: Vec<&str> = object.all_inputs().map(|(name, _, _)| name).collect();

    if args.is_empty() {
        // No arguments - check if all inputs are optional
        for (name, _input, required) in object.all_inputs() {
            if required {
                return Err(Error::MissingInput {
                    node: object.name.clone(),
                    input: name.to_string(),
                }
                .into());
            }
        }
        return Ok(inputs);
    }

    // Check if first argument is a table (named arguments)
    if let Some(LuaValue::Table(table)) = args.first() {
        // Named arguments mode
        for (name, _input, required) in object.all_inputs() {
            let value: LuaValue = table.get(name)?;
            if value.is_nil() {
                if required {
                    return Err(Error::MissingInput {
                        node: object.name.clone(),
                        input: name.to_string(),
                    }
                    .into());
                }
            } else {
                inputs.insert(name.to_string(), lua_to_workflow_input(value)?);
            }
        }
    } else {
        // Positional arguments mode
        for (i, value) in args.into_iter().enumerate() {
            if i >= input_names.len() {
                break; // Extra arguments ignored
            }
            let name = input_names[i];
            inputs.insert(name.to_string(), lua_to_workflow_input(value)?);
        }

        // Check for missing required inputs
        for (name, _input, required) in object.all_inputs() {
            if required && !inputs.contains_key(name) {
                return Err(Error::MissingInput {
                    node: object.name.clone(),
                    input: name.to_string(),
                }
                .into());
            }
        }
    }

    Ok(inputs)
}

/// Create the appropriate output type for a node.
fn create_node_output(object: &Object, node_id: WorkflowNodeId) -> NodeOutputValue {
    if object.output.len() <= 1 {
        // Single output or output node
        NodeOutputValue::Single(NodeOutput::new(node_id, 0))
    } else {
        // Multiple outputs - create named outputs
        let mut outputs = HashMap::new();
        for (i, name) in object.output_name.iter().enumerate() {
            // Convert output name to snake_case for Lua field access
            let lua_name = to_snake_case(name);
            outputs.insert(lua_name, i as u32);
        }
        NodeOutputValue::Multiple(NodeOutputs::new(node_id, outputs))
    }
}

/// Convert a string to snake_case.
fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c);
        }
    }
    result
}

/// Parse a Lua table into ObjectInfo.
fn parse_object_info(_lua: &Lua, table: LuaTable) -> LuaResult<ObjectInfo> {
    // The table should be the result of client:get_object_info()
    // which is already a properly structured ObjectInfo
    let mut object_info = BTreeMap::new();

    for pair in table.pairs::<String, LuaTable>() {
        let (name, obj_table) = pair?;
        let object = parse_object(&name, obj_table)?;
        object_info.insert(name, object);
    }

    Ok(object_info)
}

/// Parse a single object definition from Lua.
fn parse_object(name: &str, table: LuaTable) -> LuaResult<Object> {
    use rucomfyui::object_info::*;

    let display_name: Option<String> = table.get("display_name")?;
    let description: String = table.get::<String>("description").unwrap_or_default();
    let python_module: String = table.get::<String>("python_module").unwrap_or_default();
    let category: String = table.get::<String>("category").unwrap_or_default();
    let output_node: bool = table.get("output_node").unwrap_or(false);

    // Parse outputs
    let output = parse_output_types(table.get::<LuaTable>("output")?)?;
    let output_is_list = parse_bool_array(table.get::<LuaTable>("output_is_list")?)?;
    let output_name = parse_string_array(table.get::<LuaTable>("output_name")?)?;
    let output_tooltips = table
        .get::<LuaTable>("output_tooltips")
        .ok()
        .map(|t| parse_optional_string_array(t).unwrap_or_default())
        .unwrap_or_default();

    // Parse inputs
    let input_table: LuaTable = table.get("input")?;
    let input_order_table: LuaTable = table.get("input_order")?;

    let required_inputs = parse_inputs(input_table.get::<LuaTable>("required")?)?;
    let optional_inputs = input_table
        .get::<LuaTable>("optional")
        .ok()
        .map(parse_inputs)
        .transpose()?;

    let required_order = parse_string_array(input_order_table.get::<LuaTable>("required")?)?;
    let optional_order = input_order_table
        .get::<LuaTable>("optional")
        .ok()
        .map(parse_string_array)
        .transpose()?;

    Ok(Object {
        name: name.to_string(),
        display_name,
        description,
        python_module,
        category,
        input: ObjectInputBundle {
            required: required_inputs,
            optional: optional_inputs,
        },
        input_order: ObjectInputBundle {
            required: required_order,
            optional: optional_order,
        },
        output,
        output_is_list,
        output_name,
        output_node,
        output_tooltips,
    })
}

fn parse_output_types(table: LuaTable) -> LuaResult<Vec<ObjectType>> {
    let mut result = Vec::new();
    for value in table.sequence_values::<String>() {
        let type_str = value?;
        let object_type = parse_object_type(&type_str);
        result.push(object_type);
    }
    Ok(result)
}

fn parse_object_type(s: &str) -> ObjectType {
    // Try to match known types
    match s {
        "ACCESSORIES_OPTIONS" => ObjectType::AccessoriesOptions,
        "AUDIO" => ObjectType::Audio,
        "AUDIO_ENCODER" => ObjectType::AudioEncoder,
        "AUDIO_ENCODER_OUTPUT" => ObjectType::AudioEncoderOutput,
        "BOOLEAN" => ObjectType::Boolean,
        "CAMERA_CONTROL" => ObjectType::CameraControl,
        "CLIP" => ObjectType::Clip,
        "CLIP_VISION" => ObjectType::ClipVision,
        "CLIP_VISION_OUTPUT" => ObjectType::ClipVisionOutput,
        "CONDITIONING" => ObjectType::Conditioning,
        "CONTROL_NET" => ObjectType::ControlNet,
        "FLOAT" => ObjectType::Float,
        "GEMINI_INPUT_FILES" => ObjectType::GeminiInputFiles,
        "GLIGEN" => ObjectType::Gligen,
        "GUIDER" => ObjectType::Guider,
        "HOOK_KEYFRAMES" => ObjectType::HookKeyframes,
        "HOOKS" => ObjectType::Hooks,
        "IMAGE" => ObjectType::Image,
        "INPAINT_MODEL" => ObjectType::InpaintModel,
        "INPAINT_PATCH" => ObjectType::InpaintPatch,
        "INT" => ObjectType::Int,
        "LATENT" => ObjectType::Latent,
        "LATENT_OPERATION" => ObjectType::LatentOperation,
        "LOAD3D_CAMERA" => ObjectType::Load3DCamera,
        "LORA_MODEL" => ObjectType::LoraModel,
        "LOSS_MAP" => ObjectType::LossMap,
        "LUMA_CONCEPTS" => ObjectType::LumaConcepts,
        "LUMA_REF" => ObjectType::LumaRef,
        "MASK" => ObjectType::Mask,
        "MESH" => ObjectType::Mesh,
        "MODEL" => ObjectType::Model,
        "MODEL_PATCH" => ObjectType::ModelPatch,
        "NOISE" => ObjectType::Noise,
        "OPENAI_CHAT_CONFIG" => ObjectType::OpenAiChatConfig,
        "OPENAI_INPUT_FILES" => ObjectType::OpenAiInputFiles,
        "PHOTOMAKER" => ObjectType::Photomaker,
        "PIXVERSE_TEMPLATE" => ObjectType::PixverseTemplate,
        "RECRAFT_COLOR" => ObjectType::RecraftColor,
        "RECRAFT_CONTROLS" => ObjectType::RecraftControls,
        "RECRAFT_V3_STYLE" => ObjectType::RecraftV3Style,
        "SAMPLER" => ObjectType::Sampler,
        "SIGMAS" => ObjectType::Sigmas,
        "STRING" => ObjectType::String,
        "STYLE_MODEL" => ObjectType::StyleModel,
        "SVG" => ObjectType::Svg,
        "TIMESTEPS_RANGE" => ObjectType::TimestepsRange,
        "UPSCALE_MODEL" => ObjectType::UpscaleModel,
        "VAE" => ObjectType::Vae,
        "VIDEO" => ObjectType::Video,
        "VOXEL" => ObjectType::Voxel,
        "WAN_CAMERA_EMBEDDING" => ObjectType::WanCameraEmbedding,
        "WEBCAM" => ObjectType::Webcam,
        "*" => ObjectType::Wildcard,
        other => ObjectType::Other(other.to_string()),
    }
}

fn parse_bool_array(table: LuaTable) -> LuaResult<Vec<bool>> {
    table.sequence_values::<bool>().collect()
}

fn parse_string_array(table: LuaTable) -> LuaResult<Vec<String>> {
    table.sequence_values::<String>().collect()
}

fn parse_optional_string_array(table: LuaTable) -> LuaResult<Vec<Option<String>>> {
    let mut result = Vec::new();
    for value in table.sequence_values::<LuaValue>() {
        let v = value?;
        match v {
            LuaValue::Nil => result.push(None),
            LuaValue::String(s) => result.push(Some(s.to_str()?.to_string())),
            _ => result.push(None),
        }
    }
    Ok(result)
}

fn parse_inputs(table: LuaTable) -> LuaResult<BTreeMap<String, rucomfyui::object_info::ObjectInput>>
{
    use rucomfyui::object_info::*;

    let mut result = BTreeMap::new();

    for pair in table.pairs::<String, LuaValue>() {
        let (name, value) = pair?;

        // Each input is an array: [type, meta?]
        let input = match value {
            LuaValue::Table(arr) => {
                let type_value: LuaValue = arr.get(1)?;
                let meta_value: Option<LuaTable> = arr.get(2).ok();

                let input_type = parse_input_type(type_value)?;
                let input_meta = meta_value
                    .map(parse_input_meta)
                    .transpose()?
                    .unwrap_or(ObjectInputMeta {
                        tooltip: None,
                        typed: None,
                    });

                ObjectInput::InputWithMeta(input_type, input_meta)
            }
            _ => continue, // Skip malformed inputs
        };

        result.insert(name, input);
    }

    Ok(result)
}

fn parse_input_type(value: LuaValue) -> LuaResult<rucomfyui::object_info::ObjectInputType> {
    use rucomfyui::object_info::*;

    match value {
        LuaValue::String(s) => {
            let type_str = s.to_str()?;
            Ok(ObjectInputType::Typed(parse_object_type(&type_str)))
        }
        LuaValue::Table(arr) => {
            // Array of string options
            let mut options = Vec::new();
            for v in arr.sequence_values::<LuaValue>() {
                match v? {
                    LuaValue::String(s) => {
                        options.push(ObjectInputTypeArrayValue::String(s.to_str()?.to_string()));
                    }
                    LuaValue::Table(t) => {
                        // ContentImage format
                        let content: String = t.get("content").unwrap_or_default();
                        let image: Option<String> = t.get("image").ok();
                        options.push(ObjectInputTypeArrayValue::ContentImage { content, image });
                    }
                    _ => {}
                }
            }
            Ok(ObjectInputType::Array(options))
        }
        _ => Err(LuaError::external("Invalid input type")),
    }
}

fn parse_input_meta(table: LuaTable) -> LuaResult<rucomfyui::object_info::ObjectInputMeta> {
    use rucomfyui::object_info::*;

    let tooltip: Option<String> = table.get("tooltip").ok();

    // Try to determine typed metadata
    let typed = if let Ok(default) = table.get::<bool>("default") {
        Some(ObjectInputMetaTyped::Boolean(ObjectInputMetaTypedBoolean {
            default,
        }))
    } else if table.contains_key("image_upload")? {
        Some(ObjectInputMetaTyped::Image(ObjectInputMetaTypedImage {
            image_upload: table.get("image_upload").unwrap_or(false),
        }))
    } else if table.contains_key("audio_upload")? {
        Some(ObjectInputMetaTyped::Audio(ObjectInputMetaTypedAudio {
            audio_upload: table.get("audio_upload").unwrap_or(false),
        }))
    } else if table.contains_key("multiline")? || table.contains_key("dynamicPrompts")? {
        Some(ObjectInputMetaTyped::String(ObjectInputMetaTypedString {
            dynamic_prompts: table.get("dynamicPrompts").ok(),
            multiline: table.get("multiline").ok(),
            default: table.get("default").ok(),
        }))
    } else if table.contains_key("min")? || table.contains_key("max")? {
        Some(ObjectInputMetaTyped::Number(ObjectInputMetaTypedNumber {
            default: parse_number_value(table.get::<LuaValue>("default")?)?,
            display: table.get("display").ok(),
            max: parse_number_value(table.get::<LuaValue>("max")?)?,
            min: parse_number_value(table.get::<LuaValue>("min")?)?,
            round: table
                .get::<LuaValue>("round")
                .ok()
                .map(parse_round_value)
                .transpose()?,
            step: table
                .get::<LuaValue>("step")
                .ok()
                .map(parse_number_value)
                .transpose()?,
        }))
    } else {
        None
    };

    Ok(ObjectInputMeta { tooltip, typed })
}

fn parse_number_value(
    value: LuaValue,
) -> LuaResult<rucomfyui::object_info::ObjectInputMetaTypedNumberValue> {
    use rucomfyui::object_info::ObjectInputMetaTypedNumberValue;

    match value {
        LuaValue::Integer(i) => {
            if i >= 0 {
                Ok(ObjectInputMetaTypedNumberValue::U64(i as u64))
            } else {
                Ok(ObjectInputMetaTypedNumberValue::I64(i))
            }
        }
        LuaValue::Number(n) => {
            if n.fract() == 0.0 {
                if n >= 0.0 {
                    Ok(ObjectInputMetaTypedNumberValue::U64(n as u64))
                } else {
                    Ok(ObjectInputMetaTypedNumberValue::I64(n as i64))
                }
            } else {
                Ok(ObjectInputMetaTypedNumberValue::F64(n))
            }
        }
        _ => Ok(ObjectInputMetaTypedNumberValue::I64(0)),
    }
}

fn parse_round_value(
    value: LuaValue,
) -> LuaResult<rucomfyui::object_info::ObjectInputMetaTypedRoundValue> {
    use rucomfyui::object_info::ObjectInputMetaTypedRoundValue;

    match value {
        LuaValue::Boolean(b) => Ok(ObjectInputMetaTypedRoundValue::Bool(b)),
        LuaValue::Integer(i) => Ok(ObjectInputMetaTypedRoundValue::Number(i as f64)),
        LuaValue::Number(n) => Ok(ObjectInputMetaTypedRoundValue::Number(n)),
        _ => Ok(ObjectInputMetaTypedRoundValue::Bool(false)),
    }
}
