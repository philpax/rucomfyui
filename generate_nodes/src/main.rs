use std::{
    collections::{BTreeMap, HashMap},
    path::Path,
};

use proc_macro2::TokenStream;
use quote::quote;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    let url = std::env::args().nth(1).expect("ComfyUI URL not provided");
    let object_info = reqwest::get(format!("{url}/object_info")).await.unwrap();
    let object_info: HashMap<String, Node> = object_info.json().await.unwrap();

    let category_tree =
        build_category_tree(object_info.values().filter(|n| {
            !(n.python_module.starts_with("custom_nodes") || n.category.starts_with("_"))
        }));
    write_category_tree(("", &category_tree), Path::new("src/nodes"));
}

fn write_category_tree((name, tree): (&str, &CategoryTree), directory: &Path) {
    std::fs::create_dir(directory).ok();

    let mut modules = vec![];
    let mut nodes = vec![];

    for (key, node) in tree {
        match node {
            CategoryTreeNode::Category(name, tree) => {
                let key = name_to_ident(key, false);
                write_category_tree((name, tree), &directory.join(key.to_string()));
                modules.push(quote! {
                    pub mod #key;
                });
            }
            CategoryTreeNode::Node(node) => {
                nodes.push(write_node(node));
            }
        }
    }
    let doc = if name.is_empty() {
        "Root module".to_string()
    } else {
        name.to_string()
    };
    let file = syn::parse2::<syn::File>(quote! {
        #![doc = #doc]
        #(#modules)*
        #(#nodes)*
    })
    .unwrap();
    std::fs::write(directory.join("mod.rs"), prettyplease::unparse(&file)).unwrap();
}

fn write_node(node: &Node) -> TokenStream {
    let name = name_to_ident(&node.name, true);
    let mut doc = format!("**{}**", node.display_name);
    if !node.description.is_empty() {
        doc.push_str("\n\n");
        doc.push_str(&node.description);
    }
    quote! {
        #[doc = #doc]
        pub struct #name {
        }
    }
}

fn name_to_ident(name: &str, pascal_case: bool) -> syn::Ident {
    let mut name = name.replace("_", "");
    if name.starts_with(char::is_numeric) {
        name = format!("n{name}");
    }

    if pascal_case {
        if let Some(first) = name.chars().next() {
            name.replace_range(0..1, &first.to_uppercase().to_string());
        }
    }

    quote::format_ident!("{name}")
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum InputType {
    Array(Vec<String>),
    Typed(NodeType),
}

#[derive(Debug, Serialize, Deserialize)]
struct InputBundle<T> {
    required: Option<T>,
    optional: Option<T>,
}

#[derive(Debug, Serialize, Deserialize)]
struct InputMeta {
    tooltip: Option<String>,
    #[serde(flatten)]
    rest: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
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
