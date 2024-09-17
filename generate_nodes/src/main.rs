use std::{
    collections::{BTreeMap, HashMap},
    path::Path,
};

use anyhow::{Context, Result};
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    run().await.unwrap();
}

async fn run() -> Result<()> {
    let url = std::env::args().nth(1).expect("ComfyUI URL not provided");
    let object_info = reqwest::get(format!("{url}/object_info")).await?;
    let object_info: HashMap<String, Node> = object_info.json().await?;

    let category_tree =
        build_category_tree(object_info.values().filter(|n| {
            !(n.python_module.starts_with("custom_nodes") || n.category.starts_with("_"))
        }));
    write_category_tree(("", &category_tree), Path::new("src/nodes")).context("root")?;
    Ok(())
}

fn write_category_tree((name, tree): (&str, &CategoryTree), directory: &Path) -> Result<()> {
    std::fs::create_dir(directory).ok();

    let mut modules = vec![];
    let mut nodes = vec![];

    for (key, node) in tree {
        match node {
            CategoryTreeNode::Category(name, tree) => {
                let key = name_to_ident(key, false)?;
                write_category_tree((name, tree), &directory.join(key.to_string()))
                    .context(name.clone())?;
                modules.push(quote! {
                    pub mod #key;
                });
            }
            CategoryTreeNode::Node(node) => {
                nodes.push(write_node(node)?);
            }
        }
    }

    let (doc, extra) = if name.is_empty() {
        let types = NodeType::ALL
            .iter()
            .map(|ty| {
                let doc = format!("**{ty:?}**");
                let name = name_to_ident(&format!("{ty:?}"), true)?;

                let output_doc = format!("A node output of type [`{ty:?}`].");
                let output_name = name_to_ident(&format!("{ty:?}Out"), true)?;
                Ok(quote! {
                    #[doc = #doc]
                    pub trait #name : ToTypedValue {}

                    #[doc = #output_doc]
                    pub struct #output_name(pub u32);
                    impl ToTypedValue for #output_name {
                        fn to_typed_value(self) -> TypedValue {
                            TypedValue::Slot(self.0)
                        }
                    }
                    impl #name for #output_name {}
                })
            })
            .collect::<Result<Vec<_>>>()?;

        (
            "Root module".to_string(),
            quote! {
                /// Implemented for all typed nodes. Provides the node's output and metadata.
                pub trait TypedNode {
                    /// The type of the node's output.
                    type Output;
                    /// Returns the node's output.
                    fn output(&self) -> Self::Output;

                    /// The name of the node.
                    const NAME: &'static str;
                    /// The display name of the node.
                    const DISPLAY_NAME: &'static str;
                    /// The description of the node.
                    const DESCRIPTION: &'static str;
                    /// The category of the node.
                    const CATEGORY: &'static str;
                }

                /// A typed value. Used to determine the value for a given input.
                #[derive(Debug, Clone, PartialEq)]
                pub enum TypedValue {
                    /// A string value.
                    String(std::string::String),
                    /// A f32 value.
                    F32(f32),
                    /// A i32 value.
                    I32(i32),
                    /// A boolean value.
                    Boolean(bool),
                    /// A slot value. The inner value is the index of the output slot.
                    Slot(u32),
                }

                /// Converts a value to a typed value for use in a workflow.
                pub trait ToTypedValue {
                    /// Converts the value to a typed value.
                    fn to_typed_value(self) -> TypedValue;
                }

                impl ToTypedValue for std::string::String {
                    fn to_typed_value(self) -> TypedValue {
                        TypedValue::String(self)
                    }
                }
                impl String for std::string::String {}

                impl ToTypedValue for f32 {
                    fn to_typed_value(self) -> TypedValue {
                        TypedValue::F32(self)
                    }
                }
                impl Float for f32 {}

                impl ToTypedValue for i32 {
                    fn to_typed_value(self) -> TypedValue {
                        TypedValue::I32(self)
                    }
                }
                impl Int for i32 {}

                impl ToTypedValue for bool {
                    fn to_typed_value(self) -> TypedValue {
                        TypedValue::Boolean(self)
                    }
                }
                impl Boolean for bool {}

                #(#types)*
            },
        )
    } else {
        (name.to_string(), quote! {})
    };

    let output = quote! {
        #![doc = #doc]
        #(#modules)*
        #(#nodes)*
        #extra
    };
    let output = match syn::parse2::<syn::File>(output.clone()) {
        Ok(file) => prettyplease::unparse(&file),
        Err(e) => {
            println!("Error parsing output for {name}: {e}");
            output.to_string()
        }
    };
    std::fs::write(directory.join("mod.rs"), output)?;
    Ok(())
}

fn write_node(node: &Node) -> Result<TokenStream> {
    let (inputs_name, processed_inputs, inputs) = write_node_inputs(node)?;
    let outputs = write_node_outputs(node, &inputs_name, processed_inputs)?;

    Ok(quote! {
        #inputs
        #outputs
    })
}

struct ProcessedInput<'a> {
    name: &'a str,
    tooltip: Option<&'a str>,
    ty: NodeType,
    generic_name: syn::Ident,
    generic_ty: syn::Ident,
    optional: bool,
}

fn write_node_inputs(node: &Node) -> Result<(syn::Ident, Vec<ProcessedInput>, TokenStream)> {
    fn process_input<'a>(
        name: &'a str,
        input: &'a Input,
        optional: bool,
    ) -> Option<ProcessedInput<'a>> {
        let ty = input.as_type()?;
        Some(ProcessedInput {
            name,
            tooltip: input.tooltip(),
            ty,
            generic_name: name_to_ident(name, true).ok()?,
            generic_ty: name_to_ident(&format!("{:?}", ty), true).ok()?,
            optional,
        })
    }

    let required_inputs = node
        .input_order
        .required
        .iter()
        .flat_map(|name| process_input(name, node.input.required.get(name)?, false))
        .collect::<Vec<_>>();
    let optional_inputs = node
        .input_order
        .optional
        .as_ref()
        .map(|o| {
            o.iter()
                .flat_map(|name| {
                    process_input(name, node.input.optional.as_ref()?.get(name)?, true)
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let processed_inputs = required_inputs
        .into_iter()
        .chain(optional_inputs.into_iter())
        .collect::<Vec<_>>();

    let input_generics = processed_inputs
        .iter()
        .map(|input| {
            let generic_name = &input.generic_name;
            let ty = &input.generic_ty;
            let default = if input.optional {
                let output_struct_name = input.ty.output_struct_ident();
                quote! { = crate :: nodes :: #output_struct_name }
            } else {
                quote! {}
            };
            Ok(quote! {
                #generic_name: crate :: nodes :: #ty #default
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let fields = processed_inputs
        .iter()
        .map(|input| {
            let name = name_to_ident(input.name, false)?;
            let generic_name = &input.generic_name;
            let doc = input.tooltip.unwrap_or("No documentation.");
            let ty = if input.optional {
                quote! {
                    Option<#generic_name>
                }
            } else {
                quote! { #generic_name }
            };
            Ok(quote! {
                #[doc = #doc]
                pub #name: #ty
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let name = name_to_ident(&node.name, true)?;
    let mut doc = format!("**{}**", node.display_name);
    if !node.description.is_empty() {
        doc.push_str("\n\n");
        doc.push_str(&node.description);
    }

    Ok((
        name.clone(),
        processed_inputs,
        quote! {
            #[doc = #doc]
            pub struct #name < #(#input_generics),* > {
                #(#fields),*
            }
        },
    ))
}

fn write_node_outputs(
    node: &Node,
    inputs_name: &syn::Ident,
    processed_inputs: Vec<ProcessedInput<'_>>,
) -> Result<TokenStream> {
    struct ProcessedOutput<'a> {
        name: &'a str,
        ty: NodeType,
        tooltip: Option<&'a str>,
    }

    let outputs = node
        .output
        .iter()
        .zip(node.output_name.iter())
        .enumerate()
        .map(|(i, (ty, name))| ProcessedOutput {
            name: name.as_str(),
            ty: *ty,
            tooltip: node.output_tooltips.get(i).map(|s| s.as_str()),
        })
        .collect::<Vec<_>>();

    let name = name_to_ident(&format!("{}Output", node.name), true)?;
    let doc = format!("Output for [`{}`].", inputs_name);

    let fields = outputs
        .iter()
        .map(|output| {
            let name = name_to_ident(output.name, false)?;
            let ty = output.ty.output_struct_ident();
            let doc = output.tooltip.unwrap_or("No documentation.");
            Ok(quote! {
                #[doc = #doc]
                pub #name: crate :: nodes :: #ty
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let typed_node = {
        let outputs_name = name.clone();
        let node_name = node.name.as_str();
        let display_name = node.display_name.as_str();
        let description = node.description.as_str();
        let category = node.category.as_str();

        let fields = outputs
            .iter()
            .enumerate()
            .map(|(i, output)| {
                let name = name_to_ident(output.name, false)?;
                let ty = output.ty.output_struct_ident();
                let i = i as u32;
                Ok(quote! {
                    #name: crate :: nodes :: #ty (#i)
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let generic_list = processed_inputs.iter().map(|input| {
            let generic_name = &input.generic_name;
            let generic_ty = &input.generic_ty;
            quote! {
                #generic_name: crate :: nodes :: #generic_ty
            }
        });

        let generic_instantiation_list = processed_inputs.iter().map(|input| {
            let generic_name = &input.generic_name;
            quote! {
                #generic_name
            }
        });

        quote! {
            impl < #(#generic_list),* > crate :: nodes :: TypedNode for #inputs_name < #(#generic_instantiation_list),* >  {
                type Output = #outputs_name;
                fn output(&self) -> Self::Output {
                    Self::Output {
                        #(#fields),*
                    }
                }

                const NAME: &'static str = #node_name;
                const DISPLAY_NAME: &'static str = #display_name;
                const DESCRIPTION: &'static str = #description;
                const CATEGORY: &'static str = #category;
            }
        }
    };

    Ok(quote! {
        #[doc = #doc]
        pub struct #name {
            #(#fields),*
        }
        #typed_node
    })
}

fn name_to_ident(name: &str, pascal_case: bool) -> Result<syn::Ident> {
    let mut name = name.replace(".", "_");
    if name.starts_with(char::is_numeric) {
        name = format!("n{name}");
    }

    if pascal_case {
        name = name.to_case(Case::UpperCamel);
    } else {
        name = name.to_case(Case::Snake);
    }

    std::panic::catch_unwind(|| quote::format_ident!("{name}"))
        .map_err(|e| anyhow::anyhow!("Error parsing {name}: {:?}", e.downcast_ref::<&str>()))
}

#[derive(Debug, Serialize, Deserialize)]
struct Node {
    name: String,
    display_name: String,
    description: String,
    python_module: String,
    category: String,

    input: InputBundle<HashMap<String, Input>>,
    input_order: InputBundle<Vec<String>>,

    output: Vec<NodeType>,
    output_is_list: Vec<bool>,
    output_name: Vec<String>,
    output_node: bool,
    #[serde(default)]
    output_tooltips: Vec<String>,
}

enum CategoryTreeNode<'a> {
    Category(String, CategoryTree<'a>),
    Node(&'a Node),
}

impl<'a> std::fmt::Debug for CategoryTreeNode<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Category(_name, arg0) => f.debug_tuple("Category").field(arg0).finish(),
            Self::Node(arg0) => f.debug_tuple("Node").field(&arg0.display_name).finish(),
        }
    }
}

type CategoryTree<'a> = BTreeMap<String, CategoryTreeNode<'a>>;
fn build_category_tree<'a>(nodes: impl Iterator<Item = &'a Node>) -> CategoryTree<'a> {
    let mut tree = CategoryTree::new();
    for node in nodes {
        let categories: Vec<&str> = node.category.split('/').collect();
        insert_node(&mut tree, &categories, node);
    }
    tree
}
fn insert_node<'a>(tree: &mut CategoryTree<'a>, categories: &[&str], node: &'a Node) {
    if categories.is_empty() {
        tree.entry(node.name.to_string())
            .or_insert(CategoryTreeNode::Node(node));
    } else {
        let current_category = categories[0].to_string();
        let subtree = tree
            .entry(current_category.clone())
            .or_insert_with(|| CategoryTreeNode::Category(current_category, BTreeMap::new()));
        if let CategoryTreeNode::Category(_, subtree) = subtree {
            insert_node(subtree, &categories[1..], node);
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum Input {
    InputWithMeta(InputType, InputMeta),
    Input((InputType,)),
}
impl Input {
    fn as_type(&self) -> Option<NodeType> {
        match self {
            Self::InputWithMeta(ty, _) => ty.as_type(),
            Self::Input(ty) => ty.0.as_type(),
        }
    }
    fn tooltip(&self) -> Option<&str> {
        match self {
            Self::InputWithMeta(_, meta) => meta.tooltip.as_deref(),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum InputType {
    Array(Vec<String>),
    Typed(NodeType),
}
impl InputType {
    fn as_type(&self) -> Option<NodeType> {
        match self {
            Self::Typed(v) => Some(*v),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct InputBundle<T> {
    required: T,
    optional: Option<T>,
}

#[derive(Debug, Serialize, Deserialize)]
struct InputMeta {
    tooltip: Option<String>,
    #[serde(flatten)]
    rest: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum NodeType {
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
}
impl NodeType {
    const ALL: &[NodeType] = &[
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

    fn output_struct_ident(&self) -> syn::Ident {
        name_to_ident(&format!("{:?}Out", self), true).unwrap()
    }
}
