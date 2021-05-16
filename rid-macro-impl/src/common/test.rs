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

pub fn dump_tokens(tokens: &TokenStream) {
    eprintln!("----------\n\n{}\n\n----------", tokens);
}

pub fn dump_code(code: &str) {
    eprintln!("----------\n\n{}\n\n----------", code);
}

pub fn normalize_code(code: &str) -> String {
    code.lines()
        .filter_map(|x| {
            let x = x.trim();
            if x.is_empty() {
                None
            } else {
                Some(x)
            }
        })
        .collect::<Vec<&str>>()
        .join("\n")
}

pub fn normalize_code_vec(code: &str) -> Vec<&str> {
    code.lines()
        .filter_map(|x| {
            let x = x.trim();
            if x.is_empty() {
                None
            } else {
                Some(x)
            }
        })
        .collect::<Vec<&str>>()
}
