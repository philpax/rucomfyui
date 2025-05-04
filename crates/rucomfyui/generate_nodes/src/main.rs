use std::path::Path;

use anyhow::{Context, Result};
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;

mod util;

use rucomfyui::object_info::{
    CategoryTree, CategoryTreeNode, Object, ObjectInputMetaTyped, ObjectType,
};

#[tokio::main]
async fn main() {
    run().await.unwrap();
}

async fn run() -> Result<()> {
    let our_directory = Path::new(file!()).parent().unwrap().parent().unwrap();
    let rucomfyui_directory = our_directory.parent().unwrap();

    let object_info = load_or_get_object_info(our_directory).await?;
    let category_tree = rucomfyui::object_info::categorize(object_info.iter());

    let out_dir = rucomfyui_directory.join("src").join("nodes");
    let _ = std::fs::remove_dir_all(&out_dir);
    std::fs::create_dir_all(&out_dir)?;

    // Write all.
    write_category_tree_root(&category_tree, &out_dir).context("root")?;
    util::write_tokenstream(&out_dir.join("types.rs"), type_module_definitions()?)?;
    util::write_tokenstream(&out_dir.join("all.rs"), all_nodes(&category_tree, &[])?)?;

    Ok(())
}

async fn load_or_get_object_info(
    our_directory: &Path,
) -> Result<Vec<rucomfyui::object_info::Object>> {
    let path = our_directory.join("object_info.json");
    match std::fs::read_to_string(&path) {
        Ok(existing) => Ok(serde_json::from_str(&existing)?),
        Err(_) => {
            println!("Loading object info from ComfyUI...");
            let url = std::env::args().nth(1).expect("ComfyUI URL not provided");
            let client = rucomfyui::Client::new(url);

            let mut object_info: Vec<Object> = client
                .object_info()
                .await?
                .values()
                .filter(|n| {
                    !(n.python_module.starts_with("custom_nodes") || n.category.starts_with("_"))
                })
                .cloned()
                .collect();
            object_info.sort_by(|a, b| a.name.cmp(&b.name));
            // Scrub all array type values to improve determinism of results.
            for object in &mut object_info {
                for input in object.input.required.values_mut().chain(
                    object
                        .input
                        .optional
                        .iter_mut()
                        .flat_map(|v| v.values_mut()),
                ) {
                    if let Some(array) = input.as_input_type_mut().as_array_mut() {
                        array.clear();
                    }
                }
            }
            std::fs::write(path, serde_json::to_string_pretty(&object_info)?)?;
            Ok(object_info)
        }
    }
}

fn all_nodes(tree: &CategoryTree, ctx: &[syn::Ident]) -> Result<TokenStream> {
    fn generate_node_stmts(
        tree: &CategoryTree,
        ctx: &[syn::Ident],
        statements: &mut Vec<TokenStream>,
    ) -> Result<()> {
        for node in tree.values() {
            match node {
                CategoryTreeNode::Category(name, tree) => {
                    let name = util::name_to_ident(name, util::NameToIdentCase::Snake)?;
                    generate_node_stmts(
                        tree,
                        &ctx.iter().cloned().chain([name]).collect::<Vec<_>>(),
                        statements,
                    )?;
                }
                CategoryTreeNode::Object(object) => {
                    let name = util::name_to_ident(&object.name, util::NameToIdentCase::Pascal)?;
                    statements.push(quote! {
                        pub use crate :: nodes :: #(#ctx::)* #name;
                    });
                }
            }
        }

        Ok(())
    }

    let mut nodes = vec![];
    generate_node_stmts(tree, ctx, &mut nodes)?;
    Ok(quote! {
        //! Helper module to import all nodes at once.
        #(#nodes)*
    })
}

fn type_module_definitions() -> Result<TokenStream> {
    let types = ObjectType::ALL
        .iter()
        .map(|ty| {
            let recased_ty = format!("{ty:?}").to_case(Case::ScreamingSnake);
            let doc = format!("A value of ComfyUI type `{recased_ty}`.");
            let name = util::name_to_ident(&format!("{ty:?}"), util::NameToIdentCase::Pascal)?;

            let output_doc = format!("A node output of type [`{ty:?}`].");
            let output_name = util::object_type_out_struct_ident(ty);
            Ok(quote! {
                #[doc = #doc]
                pub trait #name : Clone + Into<WorkflowInput> {}

                #[doc = #output_doc]
                #[derive(Clone, Copy)]
                pub struct #output_name(pub UntypedOut);
                impl Out for #output_name {
                    fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
                        Self(UntypedOut::from_dynamic(node_id, node_slot))
                    }
                    fn into_input(self) -> WorkflowInput {
                        self.0.into_input()
                    }
                }
                impl #name for #output_name {}
            })
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(quote! {
        //! Definitions for all ComfyUI types.
        use crate::workflow::{WorkflowInput, WorkflowNodeId};

        /// Implemented for all `*Out` types.
        pub trait Out: Sized {
            /// Create an output from a dynamic node. Use carefully.
            fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self;
            /// Convert the output to a dynamic input.
            fn into_input(self) -> WorkflowInput;
        }
        impl<T: Out> From<T> for WorkflowInput {
            fn from(value: T) -> Self {
                value.into_input()
            }
        }

        /// A generic output with no specific type. Can be used when you
        /// don't care about the output type.
        ///
        /// Wrapped by the type-safe output types.
        #[derive(Clone, Copy)]
        pub struct UntypedOut {
            /// The ID of the node that produced the output.
            pub node_id: WorkflowNodeId,
            /// The node's output slot.
            pub node_slot: u32,
        }
        impl Out for UntypedOut {
            fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
                Self { node_id, node_slot }
            }
            fn into_input(self) -> WorkflowInput {
                self.node_id.to_input_with_slot(self.node_slot)
            }
        }

        #(#types)*

        impl String for std::string::String {}
        impl String for &str {}
        impl Float for f32 {}
        impl Float for f64 {}
        impl Int for u32 {}
        impl Int for i32 {}
        impl Int for i64 {}
        impl Int for u64 {}
        impl Boolean for bool {}
    })
}

fn write_category_tree_root(root: &CategoryTree, directory: &Path) -> Result<()> {
    let mut modules = vec![];

    for (key, node) in root {
        match node {
            CategoryTreeNode::Category(name, tree) => {
                let key = util::name_to_ident(key, util::NameToIdentCase::Snake)?;
                write_category_tree((name, tree), &directory.join(key.to_string()))
                    .context(name.clone())?;
                modules.push(quote! {
                    pub mod #key;
                });
            }
            CategoryTreeNode::Object(_) => unreachable!("There should be no top-level nodes."),
        }
    }

    let output = quote! {
        //! Typed node definitions for ComfyUI that provide a type-safe abstraction over the API.
        #(#modules)*
        pub mod all;
        pub mod types;

        use std::collections::HashMap;

        use crate::workflow::{WorkflowNodeId, WorkflowInput};

        /// Implemented for all typed nodes; provides the node's output and metadata.
        pub trait TypedNode: Clone {
            /// The type of the node's output.
            type Output;
            /// Returns the node's output.
            fn output(&self, node_id: WorkflowNodeId) -> Self::Output;

            /// Returns the inputs for this node after conversion to [`WorkflowInput`].
            fn inputs(&self) -> HashMap<String, WorkflowInput>;

            /// The name of the node.
            const NAME: &'static str;
            /// The display name of the node.
            const DISPLAY_NAME: &'static str;
            /// The description of the node.
            const DESCRIPTION: &'static str;
            /// The category of the node.
            const CATEGORY: &'static str;
        }

        /// Implemented for all output nodes (i.e. nodes at which a workflow terminates).
        pub trait TypedOutputNode {}
    };
    let path = directory.join("mod.rs");
    util::write_tokenstream(&path, output)?;
    Ok(())
}

fn write_category_tree((name, tree): (&str, &CategoryTree), directory: &Path) -> Result<()> {
    std::fs::create_dir(directory).ok();

    let doc = format!("`{name}` definitions/categories.");

    let mut modules = vec![];
    let mut nodes = vec![];
    let mut node_outputs = vec![];

    for (key, node) in tree {
        match node {
            CategoryTreeNode::Category(name, tree) => {
                let key = util::name_to_ident(key, util::NameToIdentCase::Snake)?;
                write_category_tree((name, tree), &directory.join(key.to_string()))
                    .context(name.clone())?;
                modules.push(quote! {
                    pub mod #key;
                });
            }
            CategoryTreeNode::Object(object) => {
                let struct_name = util::name_to_ident(&object.name, util::NameToIdentCase::Pascal)?;
                let node_output_struct_name = (!object.output_node && object.output.len() > 1)
                    .then(|| {
                        util::name_to_ident(
                            &format!("{}Output", object.name),
                            util::NameToIdentCase::Pascal,
                        )
                    })
                    .transpose()?;

                nodes.push(write_node(
                    object,
                    &struct_name,
                    node_output_struct_name.as_ref(),
                )?);

                if let Some(node_output_struct_name) = &node_output_struct_name {
                    node_outputs.push(write_node_output_struct(
                        object,
                        &struct_name,
                        node_output_struct_name,
                    )?);
                }
            }
        }
    }

    let out_section = (!node_outputs.is_empty()).then(|| {
        quote! {
            /// Output types for nodes.
            pub mod out {
                #(#node_outputs)*
            }
        }
    });

    let output = quote! {
        #![doc = #doc]
        #![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]

        use std::collections::HashMap;

        use crate::{workflow::{WorkflowNodeId, WorkflowInput}, nodes::types::Out};

        #(#modules)*

        #out_section

        #(#nodes)*
    };
    let path = directory.join("mod.rs");
    util::write_tokenstream(&path, output)?;
    Ok(())
}

fn write_node(
    node: &Object,
    struct_name: &syn::Ident,
    node_output_struct_name: Option<&syn::Ident>,
) -> Result<TokenStream> {
    let processed_inputs = node_processed_inputs(node)?;

    let node_struct = write_node_struct(node, struct_name, &processed_inputs);
    let trait_impl = write_node_trait_impl(
        node,
        struct_name,
        &processed_inputs,
        node_output_struct_name,
    )?;

    Ok(quote! {
        #node_struct
        #trait_impl
    })
}

struct ProcessedInput<'a> {
    original_name: &'a str,
    name: syn::Ident,
    meta_typed: Option<&'a ObjectInputMetaTyped>,
    tooltip: Option<&'a str>,
    ty: ObjectType,
    generic_name: syn::Ident,
    generic_ty: syn::Ident,
    optional: bool,
}
fn node_processed_inputs(node: &Object) -> Result<Vec<ProcessedInput>> {
    Ok(node
        .all_inputs()
        .flat_map(|(name, input, required)| {
            let ty = input.as_type()?;
            let meta_typed = input.as_meta_typed();
            Some(ProcessedInput {
                original_name: name,
                name: util::name_to_ident(name, util::NameToIdentCase::Snake).ok()?,
                meta_typed,
                tooltip: input.tooltip(),
                ty: ty.clone(),
                generic_name: util::name_to_ident(
                    &format!("{name}Param"),
                    util::NameToIdentCase::Pascal,
                )
                .ok()?,
                generic_ty: util::name_to_ident(&format!("{ty:?}"), util::NameToIdentCase::Pascal)
                    .ok()?,
                optional: !required,
            })
        })
        .collect())
}

fn write_node_struct(
    node: &Object,
    struct_name: &syn::Ident,
    processed_inputs: &[ProcessedInput<'_>],
) -> TokenStream {
    let input_generics = processed_inputs
        .iter()
        .map(|input| {
            let generic_name = &input.generic_name;
            let ty = &input.generic_ty;
            let default = if input.optional {
                let output_struct_name = util::object_type_out_struct_ident(&input.ty);
                quote! { = crate :: nodes :: types :: #output_struct_name }
            } else {
                quote! {}
            };
            quote! {
                #generic_name : crate :: nodes :: types :: #ty #default
            }
        })
        .collect::<Vec<_>>();

    let input_generics_no_defaults = processed_inputs.iter().map(|input| {
        let generic_name = &input.generic_name;
        let ty = &input.generic_ty;
        quote! {
            #generic_name : crate :: nodes :: types :: #ty
        }
    });

    let input_generics_instantiation = processed_inputs.iter().map(|input| {
        let generic_name = &input.generic_name;
        quote! {
            #generic_name
        }
    });

    let fields = processed_inputs.iter().map(|input| {
        let name = &input.name;
        let generic_name = &input.generic_name;
        let mut doc = input.tooltip.unwrap_or("No documentation.").to_string();
        if let Some(metadata) = input.meta_typed {
            let mut metadata_items = vec![];
            match metadata {
                ObjectInputMetaTyped::Image(v) => {
                    metadata_items.push(("Image upload", Some(v.image_upload.to_string())));
                }
                ObjectInputMetaTyped::Audio(v) => {
                    metadata_items.push(("Audio upload", Some(v.audio_upload.to_string())));
                }
                ObjectInputMetaTyped::Boolean(v) => {
                    metadata_items.push(("Default", Some(v.default.to_string())));
                }
                ObjectInputMetaTyped::String(v) => {
                    metadata_items
                        .push(("Dynamic prompts", v.dynamic_prompts.map(|v| v.to_string())));
                    metadata_items.push(("Multiline", v.multiline.map(|v| v.to_string())));
                    metadata_items.push(("Default", v.default.as_ref().map(|v| v.to_string())));
                }
                ObjectInputMetaTyped::Number(v) => {
                    metadata_items.push(("Default", Some(v.default.to_string())));
                    metadata_items.push(("Display", v.display.as_ref().map(|v| v.to_string())));
                    metadata_items.push(("Max", Some(v.max.to_string())));
                    metadata_items.push(("Min", Some(v.min.to_string())));
                    metadata_items.push(("Round", v.round.map(|v| v.to_string())));
                    metadata_items.push(("Step", v.step.map(|v| v.to_string())));
                }
            }
            let metadata_items = metadata_items
                .into_iter()
                .filter_map(|(k, v)| v.map(|v| (k, v)))
                .collect::<Vec<_>>();
            if !metadata_items.is_empty() {
                doc.push_str("\n\n**Metadata**:\n");
                for (key, value) in metadata_items {
                    doc.push_str(&format!("  - {}: {}\n", key, value));
                }
            }
        }
        let ty = if input.optional {
            quote! {
                Option<#generic_name>
            }
        } else {
            quote! { #generic_name }
        };
        quote! {
            #[doc = #doc]
            pub #name: #ty
        }
    });

    let doc = format!(
        "**{}**: {}",
        node.display_name,
        if node.description.is_empty() {
            "No description."
        } else {
            &node.description
        }
    );

    let new_arguments = processed_inputs.iter().map(|input| {
        let name = &input.name;
        let generic_name = &input.generic_name;
        let ty = if input.optional {
            quote! {
                Option<#generic_name>
            }
        } else {
            quote! { #generic_name }
        };
        quote! {
            #name: #ty
        }
    });

    let self_fields = processed_inputs.iter().map(|input| {
        let name = &input.name;
        quote! {
            #name
        }
    });

    quote! {
        #[doc = #doc]
        #[derive(Clone)]
        pub struct #struct_name < #(#input_generics),* > {
            #(#fields),*
        }
        impl < #(#input_generics_no_defaults),* > #struct_name < #(#input_generics_instantiation),* > {
            /// Create a new node.
            pub fn new( #(#new_arguments),* ) -> Self {
                Self {
                    #(#self_fields),*
                }
            }
        }
    }
}

fn write_node_trait_impl(
    node: &Object,
    inputs_name: &syn::Ident,
    processed_inputs: &[ProcessedInput<'_>],
    node_output_struct_name: Option<&syn::Ident>,
) -> Result<TokenStream> {
    let node_name = node.name.as_str();
    let display_name = node.display_name.as_str();
    let description = node.description.as_str();
    let category = node.category.as_str();

    let generic_list = processed_inputs
        .iter()
        .map(|input| {
            let generic_name = &input.generic_name;
            let generic_ty = &input.generic_ty;
            quote! {
                #generic_name: crate :: nodes :: types :: #generic_ty
            }
        })
        .collect::<Vec<_>>();

    let generic_instantiation_list = processed_inputs
        .iter()
        .map(|input| {
            let generic_name = &input.generic_name;
            quote! {
                #generic_name
            }
        })
        .collect::<Vec<_>>();

    let output_section = if let Some(node_output_struct_name) = node_output_struct_name {
        let fields = node
            .processed_output()
            .enumerate()
            .map(|(i, output)| {
                let name = util::name_to_ident(output.name, util::NameToIdentCase::Snake)?;
                let ty = util::object_type_out_struct_ident(&output.ty);
                let i = i as u32;
                Ok(quote! {
                    #name: crate :: nodes :: types :: #ty :: from_dynamic(node_id, #i)
                })
            })
            .collect::<Result<Vec<_>>>()?;

        quote! {
            type Output = out :: #node_output_struct_name;
            fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
                Self::Output {
                    #(#fields),*
                }
            }
        }
    } else if node.output_node {
        // Output nodes terminate the workflow and do not produce any
        // output, so we return just the node ID.
        quote! {
            type Output = WorkflowNodeId;
            fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
                node_id
            }
        }
    } else if node.output.len() == 1 {
        // If it's just a single output, use that type directly
        let ty = util::object_type_out_struct_ident(&node.output[0]);
        quote! {
            type Output = crate :: nodes :: types :: #ty;
            fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
                Self::Output::from_dynamic(node_id, 0)
            }
        }
    } else {
        panic!("`node_output_struct_name` missing, but not a handled case: {node:?}");
    };

    let output_node_impl = node.output_node.then(|| {
        quote! {
            impl
                < #(#generic_list),* >
                crate :: nodes :: TypedOutputNode
                for
                #inputs_name < #(#generic_instantiation_list),* >
            {}
        }
    });

    let input_section = {
        if processed_inputs.is_empty() {
            quote! {
                fn inputs(&self) -> HashMap<String, WorkflowInput> {
                    HashMap::default()
                }
            }
        } else {
            let workflow_inputs = processed_inputs.iter().map(|input| {
                let original_name = input.original_name;
                let name = &input.name;
                if input.optional {
                    quote! {
                        if let Some(v) = &self.#name {
                            output.insert(#original_name.to_string(), v.clone().into());
                        }
                    }
                } else {
                    quote! {
                        output.insert(#original_name.to_string(), self.#name.clone().into());
                    }
                }
            });

            quote! {
                fn inputs(&self) -> HashMap<String, WorkflowInput> {
                    let mut output = HashMap::default();
                    #(#workflow_inputs)*
                    output
                }
            }
        }
    };

    Ok(quote! {
        impl
            < #(#generic_list),* >
            crate :: nodes :: TypedNode
            for
            #inputs_name
            < #(#generic_instantiation_list),* >
        {
            #output_section
            #input_section

            const NAME: &'static str = #node_name;
            const DISPLAY_NAME: &'static str = #display_name;
            const DESCRIPTION: &'static str = #description;
            const CATEGORY: &'static str = #category;
        }
        #output_node_impl
    })
}

fn write_node_output_struct(
    node: &Object,
    node_struct_name: &syn::Ident,
    node_output_struct_name: &syn::Ident,
) -> Result<TokenStream> {
    let doc = format!("Output for [`{node_struct_name}`](super::{node_struct_name}).");
    let fields = node
        .processed_output()
        .map(|output| {
            let name = util::name_to_ident(output.name, util::NameToIdentCase::Snake)?;
            let ty = util::object_type_out_struct_ident(&output.ty);
            let doc = output.tooltip.unwrap_or("No documentation.");
            Ok(quote! {
                #[doc = #doc]
                pub #name: crate :: nodes :: types :: #ty
            })
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(quote! {
        #[doc = #doc]
        #[derive(Clone)]
        pub struct #node_output_struct_name {
            #(#fields),*
        }
    })
}
