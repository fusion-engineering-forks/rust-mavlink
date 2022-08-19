use std::path::PathBuf;

use proc_macro2::TokenStream;
use quote::quote;

pub fn to_module_name<P: Into<PathBuf>>(file_name: P) -> String {
    file_name
        .into()
        .file_stem() // remove extension
        .unwrap()
        .to_string_lossy() // convert to string
        .to_lowercase() // all lowercase
        .replace(|c: char| !c.is_alphanumeric(), "_") // remove non alphanum
}

/// Removes the trailing zeroes in the payload
///
/// # Note:
///
/// There must always be at least one remaining byte even if it is a
/// zero byte.
pub fn q_remove_trailing_zeroes(buf: proc_macro2::Ident) -> TokenStream {
    quote!{
        while let Some(&0) = #buf.last() {
            if #buf.len() <= 1 { break; }
            #buf.pop();
        }
    }
}
