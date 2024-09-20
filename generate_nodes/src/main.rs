use std::path::Path;

use anyhow::{Context, Result};
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;

mod util;

use rucomfyui::object_info::{CategoryTree, CategoryTreeNode, Object, ObjectInput, ObjectType};

#[tokio::main]
async fn main() {
    run().await.unwrap();
}

async fn run() -> Result<()> {
    let url = std::env::args().nth(1).expect("ComfyUI URL not provided");

    let client = rucomfyui::Client::new(url);
    let object_info = client.object_info().await?;

    let category_tree =
        rucomfyui::object_info::categorize(object_info.values().filter(|n| {
            !(n.python_module.starts_with("custom_nodes") || n.category.starts_with("_"))
        }));

    let out_dir = Path::new("src/nodes");
    let _ = std::fs::remove_dir_all(out_dir);
    std::fs::create_dir_all(out_dir)?;

    // Write all.
    write_category_tree_root(&category_tree, out_dir).context("root")?;
    util::write_tokenstream(&out_dir.join("types.rs"), type_module_definitions()?)?;
    util::write_tokenstream(&out_dir.join("all.rs"), all_nodes(&category_tree, &[])?)?;

    Ok(())
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
                    let name = util::name_to_ident(name, false)?;
                    generate_node_stmts(
                        tree,
                        &ctx.iter().cloned().chain([name]).collect::<Vec<_>>(),
                        statements,
                    )?;
                }
                CategoryTreeNode::Object(object) => {
                    let name = util::name_to_ident(&object.name, true)?;
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
            let name = util::name_to_ident(&format!("{ty:?}"), true)?;

            let output_doc = format!("A node output of type [`{ty:?}`].");
            let output_name = util::object_type_struct_ident(&ty);
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
                impl #output_name {
                    /// Create an output from a dynamic node. Use carefully.
                    pub fn from_dynamic(node_id: WorkflowNodeId, node_slot: u32) -> Self {
                        Self { node_id, node_slot }
                    }
                }
                impl ToWorkflowInput for #output_name {
                    fn to_workflow_input(&self) -> WorkflowInput {
                        self.node_id.to_input_with_slot(self.node_slot)
                    }
                }
                impl #name for #output_name {}
            })
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(quote! {
        //! Definitions for all ComfyUI types.
        use crate::{nodes::ToWorkflowInput, workflow::{WorkflowInput, WorkflowNodeId}};

        #(#types)*
    })
}

fn write_category_tree_root(root: &CategoryTree, directory: &Path) -> Result<()> {
    let mut modules = vec![];

    for (key, node) in root {
        match node {
            CategoryTreeNode::Category(name, tree) => {
                let key = util::name_to_ident(key, false)?;
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
        pub trait TypedNode {
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
                let key = util::name_to_ident(key, false)?;
                write_category_tree((name, tree), &directory.join(key.to_string()))
                    .context(name.clone())?;
                modules.push(quote! {
                    pub mod #key;
                });
            }
            CategoryTreeNode::Object(object) => {
                let struct_name = util::name_to_ident(&object.name, true)?;
                let node_output_struct_name = (!object.output_node && object.output.len() > 1)
                    .then(|| util::name_to_ident(&format!("{}Output", object.name), true))
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

        use crate::workflow::{WorkflowNodeId, WorkflowInput};

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
        &struct_name,
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
    tooltip: Option<&'a str>,
    ty: ObjectType,
    generic_name: syn::Ident,
    generic_ty: syn::Ident,
    optional: bool,
}
fn node_processed_inputs(node: &Object) -> Result<Vec<ProcessedInput>> {
    fn process_input<'a>(
        name: &'a str,
        input: &'a ObjectInput,
        optional: bool,
    ) -> Option<ProcessedInput<'a>> {
        let ty = input.as_type()?;
        Some(ProcessedInput {
            original_name: name,
            name: util::name_to_ident(name, false).ok()?,
            tooltip: input.tooltip(),
            ty: ty.clone(),
            generic_name: util::name_to_ident(name, true).ok()?,
            generic_ty: util::name_to_ident(&format!("{:?}", ty), true).ok()?,
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

    Ok(required_inputs
        .into_iter()
        .chain(optional_inputs.into_iter())
        .collect::<Vec<_>>())
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
                let output_struct_name = util::object_type_struct_ident(&input.ty);
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
        let doc = input.tooltip.unwrap_or("No documentation.");
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
                let name = util::name_to_ident(output.name, false)?;
                let ty = util::object_type_struct_ident(&output.ty);
                let i = i as u32;
                Ok(quote! {
                    #name: crate :: nodes :: types :: #ty { node_id, node_slot: #i }
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
    } else {
        if node.output_node {
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
            let ty = util::object_type_struct_ident(&node.output[0]);
            quote! {
                type Output = crate :: nodes :: types :: #ty;
                fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
                    Self::Output { node_id, node_slot: 0u32 }
                }
            }
        } else {
            panic!("`node_output_struct_name` missing, but not a handled case: {node:?}");
        }
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
                            output.insert(#original_name.to_string(), v.to_workflow_input());
                        }
                    }
                } else {
                    quote! {
                        output.insert(#original_name.to_string(), self.#name.to_workflow_input());
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
            let name = util::name_to_ident(output.name, false)?;
            let ty = util::object_type_struct_ident(&output.ty);
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
