use syn::{parse_file, Item, File};
use std::fs;
use std::env;

fn remove_all_attributes(input_file_path: &str, handwritten_module: &str, output_file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let source_code = fs::read_to_string(input_file_path)?;
    let mut syntax_tree: File = parse_file(&source_code)?;

    syntax_tree.items.retain(|item|
        match item {
            Item::Use(_) => false,
            _ => true
        }
    );

    let use_abstractions: Item = syn::parse_quote! {
        use crate::abstractions::simd::*;
    };

    let use_types: Item = syn::parse_quote! {
        use super::types::*;
    };

    let use_handwritten: Item = syn::parse_quote! {
        use super::avx_handwritten::*;
    };

    syntax_tree.items.insert(0, use_handwritten);
    syntax_tree.items.insert(0, use_types);
    syntax_tree.items.insert(0, use_abstractions);

    // Clear attributes from the file's top-level items
    for item in &mut syntax_tree.items {
        match item {
            Item::Fn(item_fn) => {
                item_fn.attrs.retain(|attr| attr.path().is_ident("doc"));
            },
            Item::Struct(item_struct) => {
                item_struct.attrs.clear();
                for field in &mut item_struct.fields {
                    field.attrs.retain(|attr| attr.path().is_ident("doc"));
                }
            },
            Item::Enum(item_enum) => {
                item_enum.attrs.clear();
                for variant in &mut item_enum.variants {
                    variant.attrs.retain(|attr| attr.path().is_ident("doc"));
                }
            },
            // Add more cases for other Item types (e.g., Item::Mod, Item::Impl, etc.)
            _ => {
                // For other item types, if they have an 'attrs' field, clear it.
                // This requires more specific matching or a helper trait.
            }
        }
    }

//    let output_tokens = quote! { #syntax_tree };
    let formatted_string = prettyplease::unparse(&syntax_tree);

    fs::write(output_file_path, formatted_string)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    remove_all_attributes(&args[1], &args[2], &args[3])
}
