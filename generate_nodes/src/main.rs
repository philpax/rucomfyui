use std::path::Path;

use anyhow::{Context, Result};
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;

use rucomfyui::{CategoryTree, CategoryTreeNode, Object, ObjectInput, ObjectType};

#[tokio::main]
async fn main() {
    run().await.unwrap();
}

async fn run() -> Result<()> {
    let url = std::env::args().nth(1).expect("ComfyUI URL not provided");

    let client = rucomfyui::Client::new(url);
    let object_info = client.object_info().await?;

    let category_tree =
        rucomfyui::categorize_objects(object_info.values().filter(|n| {
            !(n.python_module.starts_with("custom_nodes") || n.category.starts_with("_"))
        }));

    let out_dir = Path::new("src/nodes");
    let _ = std::fs::remove_dir_all(out_dir);
    std::fs::create_dir_all(out_dir)?;

    // Write all categories and nodes.
    write_category_tree(("", &category_tree), out_dir).context("root")?;

    // Write type-definitions module.
    write_tokenstream_with_formatting(&out_dir.join("types.rs"), type_module_definitions()?)?;

    // Write all-nodes module.
    let all_nodes = build_list_of_all_nodes(&category_tree, &[])?;
    write_tokenstream_with_formatting(
        &out_dir.join("all.rs"),
        quote! {
            //! Helper module to import all nodes at once.
            #(#all_nodes)*
        },
    )?;

    Ok(())
}

fn build_list_of_all_nodes(tree: &CategoryTree, ctx: &[syn::Ident]) -> Result<Vec<TokenStream>> {
    let mut output = vec![];

    for node in tree.values() {
        match node {
            CategoryTreeNode::Category(name, tree) => {
                let name = name_to_ident(name, false)?;
                output.extend(build_list_of_all_nodes(
                    tree,
                    &ctx.iter().cloned().chain([name]).collect::<Vec<_>>(),
                )?);
            }
            CategoryTreeNode::Object(object) => {
                let name = name_to_ident(&object.name, true)?;
                output.push(quote! {
                    pub use crate :: nodes :: #(#ctx::)* #name;
                });
            }
        }
    }

    Ok(output)
}

fn write_category_tree((name, tree): (&str, &CategoryTree), directory: &Path) -> Result<()> {
    std::fs::create_dir(directory).ok();

    let (doc, extra) = if name.is_empty() {
        (
            "Typed node definitions for ComfyUI that provide a type-safe abstraction over the API."
                .to_string(),
            root_module_definitions()?,
        )
    } else {
        (format!("`{name}` definitions/categories."), quote! {})
    };

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
            CategoryTreeNode::Object(object) => {
                nodes.push(write_node(object)?);
            }
        }
    }

    let output = quote! {
        #![doc = #doc]
        #![allow(unused_imports)]

        use crate::WorkflowNodeId;

        #(#modules)*
        #(#nodes)*
        #extra
    };
    let path = directory.join("mod.rs");
    write_tokenstream_with_formatting(&path, output)?;
    Ok(())
}

fn root_module_definitions() -> Result<TokenStream> {
    Ok(quote! {
        pub mod all;
        pub mod types;

        use crate::WorkflowInput;

        /// Implemented for all typed nodes; provides the node's output and metadata.
        pub trait TypedNode {
            /// The type of the node's output.
            type Output;
            /// Returns the node's output.
            fn output(&self, node_id: WorkflowNodeId) -> Self::Output;

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

        /// Converts a value to a workflow input.
        pub trait ToWorkflowInput {
            /// Converts the value to a workflow input.
            fn to_workflow_input(&self) -> WorkflowInput;
        }

        impl ToWorkflowInput for std::string::String {
            fn to_workflow_input(&self) -> WorkflowInput {
                WorkflowInput::String(self.clone())
            }
        }
        impl types :: String for std::string::String {}
        impl<'a> ToWorkflowInput for &'a str {
            fn to_workflow_input(&self) -> WorkflowInput {
                WorkflowInput::String(self.to_string())
            }
        }
        impl<'a> types :: String for &'a str {}

        impl ToWorkflowInput for f32 {
            fn to_workflow_input(&self) -> WorkflowInput {
                WorkflowInput::F32(*self)
            }
        }
        impl types :: Float for f32 {}

        impl ToWorkflowInput for i32 {
            fn to_workflow_input(&self) -> WorkflowInput {
                WorkflowInput::I32(*self)
            }
        }
        impl types :: Int for i32 {}

        impl ToWorkflowInput for bool {
            fn to_workflow_input(&self) -> WorkflowInput {
                WorkflowInput::Boolean(*self)
            }
        }
        impl types :: Boolean for bool {}
    })
}

fn type_module_definitions() -> Result<TokenStream> {
    let types = ObjectType::ALL
        .iter()
        .map(|ty| {
            let recased_ty = format!("{ty:?}").to_case(Case::ScreamingSnake);
            let doc = format!("A value of ComfyUI type `{recased_ty}`.");
            let name = name_to_ident(&format!("{ty:?}"), true)?;

            let output_doc = format!("A node output of type [`{ty:?}`].");
            let output_name = object_type_struct_ident(&ty);
            Ok(quote! {
                #[doc = #doc]
                pub trait #name : ToWorkflowInput {}
                impl ToWorkflowInput for Box<dyn #name> {
                    fn to_workflow_input(&self) -> WorkflowInput {
                        self.as_ref().to_workflow_input()
                    }
                }
                impl #name for Box<dyn #name> {}

                #[doc = #output_doc]
                #[derive(Clone, Copy)]
                pub struct #output_name {
                    /// The ID of the node that produced the output.
                    pub node_id: WorkflowNodeId,
                    /// The node's output slot.
                    pub node_slot: u32,
                }
                impl ToWorkflowInput for #output_name {
                    fn to_workflow_input(&self) -> WorkflowInput {
                        WorkflowInput::Slot(self.node_id.to_string(), self.node_slot)
                    }
                }
                impl #name for #output_name {}
            })
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(quote! {
        //! Definitions for all ComfyUI types.

        use crate::{nodes::ToWorkflowInput, WorkflowInput, WorkflowNodeId};

        #(#types)*
    })
}

fn write_tokenstream_with_formatting(path: &Path, output: TokenStream) -> Result<()> {
    let output = match syn::parse2::<syn::File>(output.clone()) {
        Ok(file) => prettyplease::unparse(&file),
        Err(e) => {
            println!("Error parsing output for {path:?}: {e}");
            output.to_string()
        }
    };
    std::fs::write(path, output)?;
    Ok(())
}

fn write_node(node: &Object) -> Result<TokenStream> {
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
    ty: ObjectType,
    generic_name: syn::Ident,
    generic_ty: syn::Ident,
    optional: bool,
}

fn write_node_inputs(node: &Object) -> Result<(syn::Ident, Vec<ProcessedInput>, TokenStream)> {
    fn process_input<'a>(
        name: &'a str,
        input: &'a ObjectInput,
        optional: bool,
    ) -> Option<ProcessedInput<'a>> {
        let ty = input.as_type()?;
        Some(ProcessedInput {
            name,
            tooltip: input.tooltip(),
            ty: ty.clone(),
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
                let output_struct_name = object_type_struct_ident(&input.ty);
                quote! { = crate :: nodes :: types :: #output_struct_name }
            } else {
                quote! {}
            };
            Ok(quote! {
                #generic_name: crate :: nodes :: types :: #ty #default
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
    node: &Object,
    inputs_name: &syn::Ident,
    processed_inputs: Vec<ProcessedInput<'_>>,
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

    if node.output_node {
        // Output nodes terminate the workflow and do not produce any
        // output, so we return an empty tuple.
        return Ok(quote! {
            impl < #(#generic_list),* > crate :: nodes :: TypedNode for #inputs_name < #(#generic_instantiation_list),* >  {
                type Output = ();
                fn output(&self, _node_id: WorkflowNodeId) -> Self::Output {}

                const NAME: &'static str = #node_name;
                const DISPLAY_NAME: &'static str = #display_name;
                const DESCRIPTION: &'static str = #description;
                const CATEGORY: &'static str = #category;
            }
            impl < #(#generic_list),* > crate :: nodes :: TypedOutputNode for #inputs_name < #(#generic_instantiation_list),* > {}
        });
    }

    let outputs = node.processed_output().collect::<Vec<_>>();

    let name = name_to_ident(&format!("{}Output", node.name), true)?;
    let doc = format!("Output for [`{}`].", inputs_name);

    let fields = outputs
        .iter()
        .map(|output| {
            let name = name_to_ident(output.name, false)?;
            let ty = object_type_struct_ident(&output.ty);
            let doc = output.tooltip.unwrap_or("No documentation.");
            Ok(quote! {
                #[doc = #doc]
                pub #name: crate :: nodes :: types :: #ty
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let typed_node = {
        let outputs_name = name.clone();

        let fields = outputs
            .iter()
            .enumerate()
            .map(|(i, output)| {
                let name = name_to_ident(output.name, false)?;
                let ty = object_type_struct_ident(&output.ty);
                let i = i as u32;
                Ok(quote! {
                    #name: crate :: nodes :: types :: #ty { node_id, node_slot: #i }
                })
            })
            .collect::<Result<Vec<_>>>()?;

        quote! {
            impl < #(#generic_list),* > crate :: nodes :: TypedNode for #inputs_name < #(#generic_instantiation_list),* >  {
                type Output = #outputs_name;
                fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
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
        #[derive(Clone)]
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

    if name == "type" {
        name = "type_".to_string();
    }

    std::panic::catch_unwind(|| quote::format_ident!("{name}"))
        .map_err(|e| anyhow::anyhow!("Error parsing {name}: {:?}", e.downcast_ref::<&str>()))
}

fn object_type_struct_ident(ty: &ObjectType) -> syn::Ident {
    name_to_ident(&format!("{ty:?}Out"), true).unwrap()
}
