use crate::function_header::{FunctionArg, FunctionHeader};

pub fn parse_function_header(line: &str) -> Option<FunctionHeader> {
    if !line.starts_with('*') && !line.starts_with('/') && line.contains('(') {
        let parts: Vec<&str> = line.splitn(2, '(').collect();
        let fn_name =
            parts[0].split(' ').last().unwrap().trim_start_matches('*');

        let args_str: &str = parts[1].splitn(2, ')').next().unwrap();
        let args = args_str
            .split(',')
            .flat_map(|x| parse_arg(x.trim()))
            .collect();

        Some(FunctionHeader {
            name: fn_name.to_string(),
            args,
        })
    } else {
        None
    }
}

fn parse_arg(arg: &str) -> Option<FunctionArg> {
    if arg == "void" {
        None
    } else if arg.contains('*') || arg.contains("PointerMut_") {
        Some(FunctionArg::Pointer)
    } else if arg.starts_with("struct") {
        let parts: Vec<&str> = arg.split(' ').collect();
        let struct_name = parts[1];
        Some(FunctionArg::Struct(struct_name.to_string()))
    } else {
        Some(FunctionArg::Int)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_function_headers() {
        let res = parse_function_header(
            "const char *rid_model_debug(struct Model *ptr);",
        );
        assert_eq!(
            res,
            Some(FunctionHeader {
                name: "rid_model_debug".to_string(),
                args: vec![FunctionArg::Pointer]
            })
        );

        let res = parse_function_header("void rid_cstring_free(char *ptr);");
        assert_eq!(
            res,
            Some(FunctionHeader {
                name: "rid_cstring_free".to_string(),
                args: vec![FunctionArg::Pointer]
            })
        );
        let res = parse_function_header(
            "const struct Todo *rid_vec_Todo_get(struct Vec_Todo *ptr, uintptr_t idx);"
        );
        assert_eq!(
            res,
            Some(FunctionHeader {
                name: "rid_vec_Todo_get".to_string(),
                args: vec![FunctionArg::Pointer, FunctionArg::Int]
            })
        );

        let res = parse_function_header(
            "Pointer_Todo rid_get_item_Pointer_Todo(struct RidVec_Pointer_Todo vec, uintptr_t idx);"
        );
        assert_eq!(
            res,
            Some(FunctionHeader {
                name: "rid_get_item_Pointer_Todo".to_string(),
                args: vec![
                    FunctionArg::Struct("RidVec_Pointer_Todo".to_string()),
                    FunctionArg::Int
                ]
            })
        );

        let res = parse_function_header(
            "void rid_msg_AddTodo(struct Model *ptr, char *arg0);",
        );
        assert_eq!(
            res,
            Some(FunctionHeader {
                name: "rid_msg_AddTodo".to_string(),
                args: vec![FunctionArg::Pointer, FunctionArg::Pointer]
            })
        );

        let res =
            parse_function_header("void rid_free_Model(PointerMut_Model ptr);");
        assert_eq!(
            res,
            Some(FunctionHeader {
                name: "rid_free_Model".to_string(),
                args: vec![FunctionArg::Pointer]
            })
        );

        let res = parse_function_header("PointerMut_Model initModel(void);");

        assert_eq!(
            res,
            Some(FunctionHeader {
                name: "initModel".to_string(),
                args: vec![]
            })
        );
    }

    #[test]
    fn not_function_headers() {
        for s in vec![
            "typedef struct Model *PointerMut_Model;",
            "/**",
            "* func dummyCalls_rid_model_debug_rid_model_debug_pretty() {",
            "uintptr_t length;",
            "#include \"stdint.h\"",
        ] {
            assert_eq!(parse_function_header(s), None)
        }
    }
}
