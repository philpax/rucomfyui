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

#[derive(Debug, Clone, Copy)]
/// The case to convert the name to.
pub enum NameToIdentCase {
    /// Preserve the original case.
    Preserve,
    /// Convert to PascalCase.
    Pascal,
    /// Convert to snake_case.
    Snake,
}

/// Converts the given name to a [`syn::Ident`], including converting to either `snake_case` or `PascalCase`.
pub fn name_to_ident(name: &str, case: NameToIdentCase) -> Result<syn::Ident> {
    let mut name = name.replace(".", "_");
    if name.starts_with(char::is_numeric) {
        name = format!("n{name}");
    }

    match case {
        NameToIdentCase::Preserve => {}
        NameToIdentCase::Pascal => {
            name = name.to_case(Case::UpperCamel);
        }
        NameToIdentCase::Snake => {
            name = name.to_case(Case::Snake);
        }
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
    name_to_ident(&format!("{ty:?}Out"), NameToIdentCase::Pascal).unwrap()
}

/// Ensures the description is doc safe by applying the following transformations:
/// - Replace `[` with `\[`
/// - Replace `]` with `\]`
/// - Replace `\n` with `\n\n`
pub fn ensure_string_is_doc_safe(description: &str) -> String {
    description
        .replace("[", "\\[")
        .replace("]", "\\]")
        .replace("\n", "\n\n")
}
