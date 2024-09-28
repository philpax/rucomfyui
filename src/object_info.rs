//! Information about the objects (nodes, inputs, outputs, etc.) in ComfyUI.

use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

use crate::{Client, Result};

/// Object info for a ComfyUI instance, where the keys are the object names.
pub type ObjectInfo = HashMap<String, Object>;

impl Client {
    /// Get the object info for this ComfyUI instance, where the keys are the object names.
    pub async fn object_info(&self) -> Result<ObjectInfo> {
        Ok(self
            .client
            .get(format!("{}/object_info", self.api_base))
            .send()
            .await?
            .json()
            .await?)
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
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
    pub input: ObjectInputBundle<HashMap<String, ObjectInput>>,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
/// Type of an input.
pub enum ObjectInputType {
    /// Array of strings. This is likely to show values exclusive to your installation;
    /// [`Self::as_type`] will return `Some(ObjectType::String)` for these to ensure that
    /// downstream code can still pass values in. If this is not desired, you can check
    /// for the array type and handle it appropriately.
    Array(Vec<String>),
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
}

#[derive(Debug, Serialize, Deserialize, Clone)]
/// Bundle of required and optional inputs.
pub struct ObjectInputBundle<T> {
    /// Required inputs.
    pub required: T,
    /// Optional inputs.
    pub optional: Option<T>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
/// Metadata for an input.
pub struct ObjectInputMeta {
    /// Tooltip for the input.
    pub tooltip: Option<String>,
    /// Rest of the input metadata.
    #[serde(flatten)]
    pub rest: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// Type of an object.
#[allow(missing_docs)]
pub enum ObjectType {
    Audio,
    Boolean,
    ClipVisionOutput,
    ClipVision,
    Clip,
    Conditioning,
    ControlNet,
    Float,
    Gligen,
    Guider,
    Image,
    InpaintModel,
    InpaintPatch,
    Int,
    Latent,
    Mask,
    Model,
    Noise,
    Photomaker,
    Sampler,
    String,
    Sigmas,
    StyleModel,
    UpscaleModel,
    Vae,
    Webcam,
    #[serde(untagged)]
    Other(String),
}
impl ObjectType {
    /// All object types excluding [`ObjectType::Other`].
    ///
    /// This should include all standard ComfyUI types.
    pub const ALL: &[ObjectType] = &[
        Self::Audio,
        Self::Boolean,
        Self::ClipVisionOutput,
        Self::ClipVision,
        Self::Clip,
        Self::Conditioning,
        Self::ControlNet,
        Self::Float,
        Self::Gligen,
        Self::Guider,
        Self::Image,
        Self::InpaintModel,
        Self::InpaintPatch,
        Self::Int,
        Self::Latent,
        Self::Mask,
        Self::Model,
        Self::Noise,
        Self::Photomaker,
        Self::Sampler,
        Self::String,
        Self::Sigmas,
        Self::StyleModel,
        Self::UpscaleModel,
        Self::Vae,
        Self::Webcam,
    ];
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
impl<'a> std::fmt::Debug for CategoryTreeNode<'a> {
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
