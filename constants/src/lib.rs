use std::{fs, path::Path};

use proc_macro::TokenStream;
#[proc_macro]
pub fn make_answer(item: TokenStream) -> TokenStream {
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