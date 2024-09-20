use std::path::Path;

use anyhow::Result;
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use rucomfyui::ObjectType;

/// Writes the given [`TokenStream`] to the path with [`prettyplease`] formatting.
pub fn write_tokenstream(path: &Path, output: TokenStream) -> Result<()> {
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

pub fn name_to_ident(name: &str, pascal_case: bool) -> Result<syn::Ident> {
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

pub fn object_type_struct_ident(ty: &ObjectType) -> syn::Ident {
    name_to_ident(&format!("{ty:?}Out"), true).unwrap()
}
