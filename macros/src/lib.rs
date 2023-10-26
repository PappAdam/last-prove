use proc_macro::TokenStream;
use std::{fs, path::Path};

#[proc_macro]
pub fn load_consts(item: TokenStream) -> TokenStream {
    let file_path = item.to_string().replace('"', "");

    let file_content = fs::read_to_string(&file_path).expect("Failed to read file content");

    let mut result = String::new();
    for s in file_content.split('\n') {
        result += "pub const ";
        result += s;
        result += ";";
    }

    result.parse().unwrap()
}
