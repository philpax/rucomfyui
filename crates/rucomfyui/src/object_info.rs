//! Information about the objects (nodes, inputs, outputs, etc.) in ComfyUI.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{Client, Result};

/// Object info for a ComfyUI instance, where the keys are the object names.
pub type ObjectInfo = BTreeMap<String, Object>;

impl Client {
    /// Get the object info for this ComfyUI instance, where the keys are the object names.
    pub async fn object_info(&self) -> Result<ObjectInfo> {
        self.get("object_info").await
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
/// Object info for a node.
pub struct Object {
    /// The name of the object.
    pub name: String,
    /// The display name of the object.
    pub display_name: String,
    /// The description of the object. Often empty.
    pub description: String,
    /// The Python module that introduced this object.
    pub python_module: String,
    /// The category of the object.
    pub category: String,

    /// Inputs to the object.
    ///
    /// Note that this is not guaranteed to be in sorted order due to the nature of JSON objects.
    /// Use [`Self::required_inputs`], [`Self::optional_inputs`], or [`Self::all_inputs`] to get
    /// the inputs in the order they should be provided.
    pub input: ObjectInputBundle<BTreeMap<String, ObjectInput>>,
    /// Order of inputs.
    pub input_order: ObjectInputBundle<Vec<String>>,

    /// Outputs from the object.
    pub output: Vec<ObjectType>,
    /// Whether the output is a list.
    pub output_is_list: Vec<bool>,
    /// The name of the output.
    pub output_name: Vec<String>,
    /// Whether this node is an output node (i.e. terminates a workflow).
    pub output_node: bool,
    /// Tooltips for the outputs.
    #[serde(default)]
    pub output_tooltips: Vec<String>,
}
impl Object {
    /// Groups together the various outputs from the object info into an iterator of processed outputs.
    pub fn processed_output(&self) -> impl Iterator<Item = ObjectProcessedOutput<'_>> + '_ {
        self.output
            .iter()
            .zip(self.output_is_list.iter())
            .zip(self.output_name.iter())
            .enumerate()
            .map(|(idx, ((ty, is_list), name))| ObjectProcessedOutput {
                ty: ty.clone(),
                is_list: *is_list,
                name: name.as_str(),
                tooltip: self.output_tooltips.get(idx).map(|s| s.as_str()),
            })
    }
    /// Required inputs for the object, returned in the order they should be provided.
    pub fn required_inputs(&self) -> impl Iterator<Item = (&str, &ObjectInput)> {
        self.input_order
            .required
            .iter()
            .map(|name| (name.as_str(), self.input.required.get(name).unwrap()))
    }
    /// Optional inputs for the object, returned in the order they should be provided.
    pub fn optional_inputs(&self) -> impl Iterator<Item = (&str, &ObjectInput)> {
        self.input_order.optional.iter().flat_map(move |names| {
            names.iter().map(move |name| {
                (
                    name.as_str(),
                    self.input.optional.as_ref().unwrap().get(name).unwrap(),
                )
            })
        })
    }
    /// All inputs for the object, returned in the order they should be provided, where the third value is
    /// true when the input is required.
    pub fn all_inputs(&self) -> impl Iterator<Item = (&str, &ObjectInput, bool)> {
        self.required_inputs()
            .map(|(name, input)| (name, input, true))
            .chain(
                self.optional_inputs()
                    .map(|(name, input)| (name, input, false)),
            )
    }
}

#[derive(Debug, Serialize, Deserialize)]
/// Processed output from an object. Built by combining the individual lists
/// of outputs from the object info.
pub struct ObjectProcessedOutput<'a> {
    /// The type of the output.
    pub ty: ObjectType,
    /// Whether the output is a list.
    pub is_list: bool,
    /// The name of the output.
    pub name: &'a str,
    /// Tooltip for the output.
    pub tooltip: Option<&'a str>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
/// Input to an object.
pub enum ObjectInput {
    /// Input with metadata.
    InputWithMeta(ObjectInputType, ObjectInputMeta),
    /// Input without metadata.
    Input((ObjectInputType,)),
}
impl ObjectInput {
    /// The [`ObjectInputType`] of the input.
    pub fn as_input_type(&self) -> &ObjectInputType {
        match self {
            Self::InputWithMeta(ty, _) => ty,
            Self::Input(ty) => &ty.0,
        }
    }
    /// Get a mutable reference to the [`ObjectInputType`] of the input.
    pub fn as_input_type_mut(&mut self) -> &mut ObjectInputType {
        match self {
            Self::InputWithMeta(ty, _) => ty,
            Self::Input(ty) => &mut ty.0,
        }
    }
    /// The [`ObjectInputMetaTyped`] of the input, if available.
    pub fn as_meta_typed(&self) -> Option<&ObjectInputMetaTyped> {
        match self {
            Self::InputWithMeta(_, meta) => meta.typed.as_ref(),
            _ => None,
        }
    }
    /// The type of the input.
    pub fn as_type(&self) -> Option<&ObjectType> {
        self.as_input_type().as_type()
    }
    /// Tooltip for the input, if available.
    pub fn tooltip(&self) -> Option<&str> {
        match self {
            Self::InputWithMeta(_, meta) => meta.tooltip.as_deref(),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
/// Type of an input.
pub enum ObjectInputType {
    /// Array of strings. This is likely to show values exclusive to your installation;
    /// [`Self::as_type`] will return `Some(ObjectType::String)` for these to ensure that
    /// downstream code can still pass values in. If this is not desired, you can check
    /// for the array type and handle it appropriately.
    Array(Vec<ObjectInputTypeArrayValue>),
    /// Typed input.
    Typed(ObjectType),
}
impl ObjectInputType {
    /// The type of the input (i.e. its [`ObjectType`]).
    ///
    /// This will return `Some(ObjectType::String)` for array types, so downstream
    /// code can still pass values in. If you need to differentiate between array
    /// types and other types, you can check for the array type and handle it
    /// appropriately.
    pub fn as_type(&self) -> Option<&ObjectType> {
        match self {
            Self::Typed(v) => Some(v),
            // HACK: I'm not sure if this is really what we want in the long run,
            // but we treat array types as strings so they can be specified in
            // the workflow
            Self::Array(_) => Some(&ObjectType::String),
        }
    }
    /// Get the array value, if this is an array type.
    pub fn as_array_mut(&mut self) -> Option<&mut Vec<ObjectInputTypeArrayValue>> {
        match self {
            Self::Array(v) => Some(v),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
/// Value of an array input.
pub enum ObjectInputTypeArrayValue {
    /// String value. This is the standard type for array values.
    String(String),
    /// Content with an optional image.
    ///
    /// Not sure what this means; it appears in the `ckpt_name` input for `CheckpointLoader|pysssss`.
    ContentImage {
        /// Content.
        content: String,
        /// Optional image.
        image: Option<String>,
    },
}
impl From<ObjectInputTypeArrayValue> for String {
    fn from(value: ObjectInputTypeArrayValue) -> Self {
        match value {
            ObjectInputTypeArrayValue::String(v) => v,
            ObjectInputTypeArrayValue::ContentImage { content, .. } => content,
        }
    }
}
impl ObjectInputTypeArrayValue {
    /// Get the content as a borrowed string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::String(v) => v.as_str(),
            Self::ContentImage { content, .. } => content.as_str(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
/// Bundle of required and optional inputs.
pub struct ObjectInputBundle<T> {
    /// Required inputs.
    pub required: T,
    /// Optional inputs.
    pub optional: Option<T>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
/// Metadata for an input.
pub struct ObjectInputMeta {
    /// Tooltip for the input.
    pub tooltip: Option<String>,
    /// Optional typed metadata.
    #[serde(flatten)]
    pub typed: Option<ObjectInputMetaTyped>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
/// Typed metadata for an input.
#[serde(untagged)]
pub enum ObjectInputMetaTyped {
    /// Metadata for an image input.
    Image(ObjectInputMetaTypedImage),
    /// Metadata for an audio input.
    Audio(ObjectInputMetaTypedAudio),
    /// Metadata for a boolean input.
    Boolean(ObjectInputMetaTypedBoolean),
    /// Metadata for a string input.
    String(ObjectInputMetaTypedString),
    /// Metadata for a number input.
    Number(ObjectInputMetaTypedNumber),
}
impl ObjectInputMetaTyped {
    /// Get the [`ObjectInputMetaTypedImage`] if this is an image input.
    pub fn as_image(&self) -> Option<&ObjectInputMetaTypedImage> {
        match self {
            Self::Image(v) => Some(v),
            _ => None,
        }
    }
    /// Get the [`ObjectInputMetaTypedAudio`] if this is an audio input.
    pub fn as_audio(&self) -> Option<&ObjectInputMetaTypedAudio> {
        match self {
            Self::Audio(v) => Some(v),
            _ => None,
        }
    }
    /// Get the [`ObjectInputMetaTypedBoolean`] if this is a boolean input.
    pub fn as_boolean(&self) -> Option<&ObjectInputMetaTypedBoolean> {
        match self {
            Self::Boolean(v) => Some(v),
            _ => None,
        }
    }
    /// Get the [`ObjectInputMetaTypedString`] if this is a string input.
    pub fn as_string(&self) -> Option<&ObjectInputMetaTypedString> {
        match self {
            Self::String(v) => Some(v),
            _ => None,
        }
    }
    /// Get the [`ObjectInputMetaTypedNumber`] if this is a number input.
    pub fn as_number(&self) -> Option<&ObjectInputMetaTypedNumber> {
        match self {
            Self::Number(v) => Some(v),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
/// Metadata for an image input.
pub struct ObjectInputMetaTypedImage {
    /// Whether the input should allow image uploads.
    pub image_upload: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
/// Metadata for an audio input.
pub struct ObjectInputMetaTypedAudio {
    /// Whether the input should allow audio uploads.
    pub audio_upload: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
/// Metadata for a boolean input.
pub struct ObjectInputMetaTypedBoolean {
    /// Default value.
    pub default: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
/// Metadata for a string input.
pub struct ObjectInputMetaTypedString {
    /// Whether the input should have dynamic prompts.
    #[serde(rename = "dynamicPrompts", skip_serializing_if = "Option::is_none")]
    pub dynamic_prompts: Option<bool>,
    /// Whether the input should be multiline.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multiline: Option<bool>,
    /// Default value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
/// Metadata for a number input.
pub struct ObjectInputMetaTypedNumber {
    /// Default value.
    pub default: ObjectInputMetaTypedNumberValue,
    /// How the number should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    /// Maximum value.
    pub max: ObjectInputMetaTypedNumberValue,
    /// Minimum value.
    pub min: ObjectInputMetaTypedNumberValue,
    /// What to round the number to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub round: Option<ObjectInputMetaTypedRoundValue>,
    /// Step value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<ObjectInputMetaTypedNumberValue>,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
/// A number in typed metadata.
///
/// This is necessary as ComfyUI may return integers for float types, so we need to preserve whatever type it originally was.
#[serde(untagged)]
pub enum ObjectInputMetaTypedNumberValue {
    /// Signed integer metadata.
    I64(i64),
    /// Unsigned integer metadata.
    U64(u64),
    /// Float metadata.
    F64(f64),
}
impl From<i64> for ObjectInputMetaTypedNumberValue {
    fn from(v: i64) -> Self {
        Self::I64(v)
    }
}
impl From<u64> for ObjectInputMetaTypedNumberValue {
    fn from(v: u64) -> Self {
        Self::U64(v)
    }
}
impl From<f64> for ObjectInputMetaTypedNumberValue {
    fn from(v: f64) -> Self {
        Self::F64(v)
    }
}
impl From<ObjectInputMetaTypedNumberValue> for i64 {
    fn from(v: ObjectInputMetaTypedNumberValue) -> Self {
        match v {
            ObjectInputMetaTypedNumberValue::I64(v) => v,
            ObjectInputMetaTypedNumberValue::U64(v) => v as i64,
            ObjectInputMetaTypedNumberValue::F64(v) => v as i64,
        }
    }
}
impl From<ObjectInputMetaTypedNumberValue> for u64 {
    fn from(v: ObjectInputMetaTypedNumberValue) -> Self {
        match v {
            ObjectInputMetaTypedNumberValue::I64(v) => v as u64,
            ObjectInputMetaTypedNumberValue::U64(v) => v,
            ObjectInputMetaTypedNumberValue::F64(v) => v as u64,
        }
    }
}
impl From<ObjectInputMetaTypedNumberValue> for f64 {
    fn from(v: ObjectInputMetaTypedNumberValue) -> Self {
        match v {
            ObjectInputMetaTypedNumberValue::I64(v) => v as f64,
            ObjectInputMetaTypedNumberValue::U64(v) => v as f64,
            ObjectInputMetaTypedNumberValue::F64(v) => v,
        }
    }
}
impl std::fmt::Display for ObjectInputMetaTypedNumberValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::I64(v) => v.fmt(f),
            Self::U64(v) => v.fmt(f),
            Self::F64(v) => v.fmt(f),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
#[serde(untagged)]
/// How to round a number in typed metadata.
///
/// This is necessary as ComfyUI can return either a boolean for whether to round to the nearest integer or a number of decimal places to round to.
pub enum ObjectInputMetaTypedRoundValue {
    /// Whether to round the number to the nearest integer.
    Bool(bool),
    /// Number of decimal places to round to.
    Number(f64),
}
impl std::fmt::Display for ObjectInputMetaTypedRoundValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool(v) => v.fmt(f),
            Self::Number(v) => v.fmt(f),
        }
    }
}

macro_rules! define_object_type {
    ($(($ot:ident, $rename:expr)),*) => {
        #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
        /// Type of an object.
        #[allow(missing_docs)]
        pub enum ObjectType {
            $(
                #[serde(rename = $rename)]
                $ot,
            )*
            #[serde(untagged)]
            Other(String),
        }
        impl ObjectType {
            /// All object types excluding [`ObjectType::Other`].
            ///
            /// This should include all standard ComfyUI types.
            pub const ALL: &[ObjectType] = &[
                $(Self::$ot,)*
            ];
        }
    }
}
define_object_type! {
    (Audio, "AUDIO"),
    (Boolean, "BOOLEAN"),
    (ClipVisionOutput, "CLIP_VISION_OUTPUT"),
    (ClipVision, "CLIP_VISION"),
    (Clip, "CLIP"),
    (Conditioning, "CONDITIONING"),
    (ControlNet, "CONTROL_NET"),
    (Float, "FLOAT"),
    (Gligen, "GLIGEN"),
    (Guider, "GUIDER"),
    (Hooks, "HOOKS"),
    (HookKeyframes, "HOOK_KEYFRAMES"),
    (Image, "IMAGE"),
    (InpaintModel, "INPAINT_MODEL"),
    (InpaintPatch, "INPAINT_PATCH"),
    (Int, "INT"),
    (Latent, "LATENT"),
    (LatentOperation, "LATENT_OPERATION"),
    (Load3DCamera, "LOAD3D_CAMERA"),
    (Mask, "MASK"),
    (Mesh, "MESH"),
    (Model, "MODEL"),
    (Noise, "NOISE"),
    (Photomaker, "PHOTOMAKER"),
    (Sampler, "SAMPLER"),
    (String, "STRING"),
    (Sigmas, "SIGMAS"),
    (StyleModel, "STYLE_MODEL"),
    (TimestepsRange, "TIMESTEPS_RANGE"),
    (UpscaleModel, "UPSCALE_MODEL"),
    (Vae, "VAE"),
    (Video, "VIDEO"),
    (Voxel, "VOXEL"),
    (Webcam, "WEBCAM")
}

/// A tree of objects based on their categories.
pub type CategoryTree<'a> = BTreeMap<String, CategoryTreeNode<'a>>;

/// A node in the category tree.
#[derive(Clone)]
pub enum CategoryTreeNode<'a> {
    /// A category in the tree.
    Category(String, CategoryTree<'a>),
    /// An object in the tree.
    Object(&'a Object),
}
impl std::fmt::Debug for CategoryTreeNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Category(_name, arg0) => f.debug_tuple("Category").field(arg0).finish(),
            Self::Object(arg0) => f.debug_tuple("Object").field(&arg0.display_name).finish(),
        }
    }
}

/// Builds a tree of objects based on their categories.
///
/// Recommended use is with a values iterator over [`Client::object_info`] with whatever filtering
/// is appropriate for your usecase.
pub fn categorize<'a>(objects: impl Iterator<Item = &'a Object>) -> CategoryTree<'a> {
    let mut tree = CategoryTree::new();
    for object in objects {
        let categories: Vec<&str> = object.category.split('/').collect();
        insert_object(&mut tree, &categories, object);
    }

    fn insert_object<'a>(tree: &mut CategoryTree<'a>, categories: &[&str], object: &'a Object) {
        if categories.is_empty() {
            tree.entry(object.name.to_string())
                .or_insert(CategoryTreeNode::Object(object));

            return;
        }

        let current_category = categories[0].to_string();
        let CategoryTreeNode::Category(_, subtree) = tree
            .entry(current_category.clone())
            .or_insert_with(|| CategoryTreeNode::Category(current_category, BTreeMap::new()))
        else {
            return;
        };
        insert_object(subtree, &categories[1..], object);
    }

    tree
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_example_object() {
        let example = serde_json::json!({
            "name": "EmptyLatentAudio",
            "display_name": "EmptyLatentAudio",
            "description": "",
            "python_module": "comfy_extras.nodes_audio",
            "category": "latent/audio",
            "input": {
              "required": {
                "batch_size": [
                  "INT",
                  {
                    "tooltip": "The number of latent images in the batch.",
                    "default": 1,
                    "max": 4096,
                    "min": 1
                  }
                ],
                "seconds": [
                  "FLOAT",
                  {
                    "tooltip": null,
                    "default": 47.6,
                    "max": 1000.0,
                    "min": 1.0,
                    "step": 0.1
                  }
                ]
              },
              "optional": null
            },
            "input_order": {
              "required": [
                "seconds",
                "batch_size"
              ],
              "optional": null
            },
            "output": [
              "LATENT"
            ],
            "output_is_list": [
              false
            ],
            "output_name": [
              "LATENT"
            ],
            "output_node": false,
            "output_tooltips": []
        });

        let target_object = Object {
            name: "EmptyLatentAudio".into(),
            display_name: "EmptyLatentAudio".into(),
            description: "".into(),
            python_module: "comfy_extras.nodes_audio".into(),
            category: "latent/audio".into(),
            input: ObjectInputBundle {
                required: BTreeMap::from_iter([
                    (
                        "batch_size".into(),
                        ObjectInput::InputWithMeta(
                            ObjectInputType::Typed(ObjectType::Int),
                            ObjectInputMeta {
                                tooltip: Some("The number of latent images in the batch.".into()),
                                typed: Some(ObjectInputMetaTyped::Number(
                                    ObjectInputMetaTypedNumber {
                                        default: 1i64.into(),
                                        display: None,
                                        max: 4096i64.into(),
                                        min: 1i64.into(),
                                        round: None,
                                        step: None,
                                    },
                                )),
                            },
                        ),
                    ),
                    (
                        "seconds".into(),
                        ObjectInput::InputWithMeta(
                            ObjectInputType::Typed(ObjectType::Float),
                            ObjectInputMeta {
                                tooltip: None,
                                typed: Some(ObjectInputMetaTyped::Number(
                                    ObjectInputMetaTypedNumber {
                                        default: 47.6.into(),
                                        display: None,
                                        max: 1000.0.into(),
                                        min: 1.0.into(),
                                        round: None,
                                        step: Some(0.1.into()),
                                    },
                                )),
                            },
                        ),
                    ),
                ]),
                optional: None,
            },
            input_order: ObjectInputBundle {
                required: vec!["seconds".into(), "batch_size".into()],
                optional: None,
            },
            output: vec![ObjectType::Latent],
            output_is_list: vec![false],
            output_name: vec!["LATENT".into()],
            output_node: false,
            output_tooltips: vec![],
        };

        let object: Object = serde_json::from_value(example).unwrap();
        assert_eq!(object, target_object);
    }
}
