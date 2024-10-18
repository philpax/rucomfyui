use std::path::Path;

use anyhow::{Context, Result};
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use rucomfyui::object_info::ObjectType;

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

/// Converts the given name to a [`syn::Ident`], including converting to either `snake_case` or `PascalCase`.
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

    syn::parse_str::<syn::Ident>(&name).with_context(|| format!("Error parsing {name}"))
}

/// A helper function to get the output struct ident for the given [`ObjectType`].
pub fn object_type_out_struct_ident(ty: &ObjectType) -> syn::Ident {
    if let ObjectType::Other(ty) = ty {
        panic!("Unexpected other type: {ty:?}");
    }
    name_to_ident(&format!("{ty:?}Out"), true).unwrap()
}
