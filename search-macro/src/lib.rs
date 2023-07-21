

use std::path::PathBuf;

use proc_macro::TokenStream;
use quote::quote;
use syn::LitStr;

#[proc_macro]
pub fn load_search_index(input: TokenStream) -> TokenStream {
    match syn::parse::<LitStr>(input) {
        Ok(input) => generate_search_index(input).into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Transforms the book to use enum routes instead of paths
fn generate_search_index(id: LitStr) -> TokenStream {
    let target_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    let name = id.value();
    let index_path = PathBuf::from(format!("{}/dioxus_search/index_{}.bin", target_dir, name));
    println!("index_path: {:?}", index_path);
    let index_str = index_path.to_str().unwrap();
    if index_path.exists() {
        quote!{
            {
                const INDEX_BYTES: &[u8] = include_bytes!(#index_str);
    
                dioxus_search::once_cell::sync::Lazy::new(|| {
                    let (bytes, _) = dioxus_search::yazi::decompress(INDEX_BYTES, dioxus_search::yazi::Format::Zlib).unwrap();
                    dioxus_search::SearchIndex::from_bytes(#name, bytes)
                })
            }
        }.into()
    }
    else {
        quote!{
            {
                dioxus_search::once_cell::sync::Lazy::new(|| {
                    eprintln!("Index not found: {:?}, have you built the index yet?", #index_str);
                    dioxus_search::SearchIndex::default()
                })
            }
        }.into()
    }

}
