use std::path::PathBuf;

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
pub fn q_remove_trailing_zeroes(buf: quote::Ident) -> quote::Tokens {
    quote::quote! {
        while let Some(&0) = #buf.last() {
            if #buf.len() <= 1 { break; }
            #buf.pop();
        }
    }
}
