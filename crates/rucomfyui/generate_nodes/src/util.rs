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
    let mut name = remove_emoji(name)
        .replace(".", "_")
        .replace("|", "_")
        .replace(" ", "_");
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

fn remove_emoji(s: &str) -> String {
    s.chars()
        .filter(|c| {
            let code = *c as u32;
            // Filter out common emoji ranges
            !(
                (0x1F600..=0x1F64F).contains(&code) || // Emoticons
                (0x1F300..=0x1F5FF).contains(&code) || // Misc Symbols and Pictographs
                (0x1F680..=0x1F6FF).contains(&code) || // Transport and Map
                (0x1F700..=0x1F77F).contains(&code) || // Alchemical
                (0x1F780..=0x1F7FF).contains(&code) || // Geometric Shapes
                (0x1F800..=0x1F8FF).contains(&code) || // Supplemental Arrows-C
                (0x1F900..=0x1F9FF).contains(&code) || // Supplemental Symbols and Pictographs
                (0x1FA00..=0x1FA6F).contains(&code) || // Chess Symbols
                (0x1FA70..=0x1FAFF).contains(&code) || // Symbols and Pictographs Extended-A
                (0x2600..=0x26FF).contains(&code)   || // Misc symbols
                (0x2700..=0x27BF).contains(&code)   || // Dingbats
                (0xFE00..=0xFE0F).contains(&code)   || // Variation Selectors
                (0x1F1E6..=0x1F1FF).contains(&code) || // Regional Indicators (flags)
                (0x1F191..=0x1F251).contains(&code) || // Enclosed chars
                (0x1F004..=0x1F0CF).contains(&code) || // Mahjong, playing cards
                (0x1F170..=0x1F189).contains(&code) || // Enclosed alphanumerics
                (0x2300..=0x23FF).contains(&code)   || // Miscellaneous Technical
                (0x2B50..=0x2B55).contains(&code)
                // Stars and other symbols
            )
        })
        .collect()
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
