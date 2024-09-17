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
                let name = name_to_ident(&format!("{ty:?}"), true)?;
                let doc = format!("**{ty:?}**");
                Ok(quote! {
                    #[doc = #doc]
                    pub trait #name {}
                })
            })
            .collect::<Result<Vec<_>>>()?;

        (
            "Root module".to_string(),
            quote! {
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
    let name = name_to_ident(&node.name, true)?;
    let mut doc = format!("**{}**", node.display_name);
    if !node.description.is_empty() {
        doc.push_str("\n\n");
        doc.push_str(&node.description);
    }

    struct ProcessedInput<'a> {
        name: &'a str,
        tooltip: Option<&'a str>,
        ty: NodeType,
        generic_name: syn::Ident,
    }

    let required_inputs = node
        .input_order
        .required
        .iter()
        .flat_map(|name| {
            let input = node.input.required.get(name)?;
            Some(ProcessedInput {
                name: name.as_str(),
                tooltip: input.tooltip(),
                ty: input.as_type()?,
                generic_name: name_to_ident(name, true).ok()?,
            })
        })
        .collect::<Vec<_>>();
    let optional_inputs = node
        .input_order
        .optional
        .as_ref()
        .map(|o| {
            o.iter()
                .flat_map(|name| {
                    let input = node.input.optional.as_ref()?.get(name)?;
                    Some(ProcessedInput {
                        name: name.as_str(),
                        tooltip: input.tooltip(),
                        ty: input.as_type()?,
                        generic_name: name_to_ident(name, true).ok()?,
                    })
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let input_generics = required_inputs
        .iter()
        .chain(optional_inputs.iter())
        .map(|input| {
            let generic_name = &input.generic_name;
            let ty = name_to_ident(&format!("{:?}", input.ty), false)?;
            Ok(quote! {
                #generic_name: crate :: nodes :: #ty
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let fields = required_inputs
        .iter()
        .chain(optional_inputs.iter())
        .map(|input| {
            let name = name_to_ident(input.name, false)?;
            let generic_name = &input.generic_name;
            let doc = input.tooltip.unwrap_or("No documentation.");
            Ok(quote! {
                #[doc = #doc]
                pub #name: #generic_name
            })
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(quote! {
        #[doc = #doc]
        pub struct #name < #(#input_generics),* > {
            #(#fields),*
        }
    })
}

fn name_to_ident(name: &str, pascal_case: bool) -> Result<syn::Ident> {
    let mut name = name.replace(".", "_");
    if name.starts_with(char::is_numeric) {
        name = format!("n{name}");
    }

    if pascal_case {
        name = name.to_case(Case::UpperCamel);
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
}
