use proc_macro2::TokenStream;
pub fn remove_doc_comments(tokens: TokenStream) -> TokenStream {
    let code = tokens.to_string();
    let lines = code.split("\"]");
    let without_docs: Vec<&str> = lines
        .into_iter()
        .filter(|x| !x.contains("# [doc ="))
        .collect();
    without_docs.join("\n").parse().unwrap()
}
