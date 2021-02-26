pub(crate) fn dart_extensions(binding: &str) -> Result<String, String> {
    let mut extension_sections: Vec<Vec<String>> = vec![];
    let mut inside_extension = false;
    let mut current_extension = vec![];

    for line in binding.lines() {
        if !inside_extension {
            if line.trim_start().starts_with("* ```dart") {
                inside_extension = true;
            }
            continue;
        }
        if line.trim_start().starts_with("* ```") {
            extension_sections.push(current_extension);
            current_extension = vec![];
            inside_extension = false;
            continue;
        }
        let trimmed_line = line.trim();
        let without_comment = &trimmed_line[2..];
        current_extension.push(without_comment.to_string());
    }

    let code = extension_sections
        .into_iter()
        .map(|section| {
            let last_line = section.len() - 1;
            section
                .into_iter()
                .enumerate()
                .fold("".to_string(), |acc, (idx, ext)| {
                    if idx == 0 || idx == last_line {
                        let new_line = if idx == 0 { "" } else { "\n" };
                        format!("{}{}{}", acc, new_line, ext)
                    } else {
                        format!("{}\n  {}", acc, ext)
                    }
                })
        })
        .fold("".to_string(), |acc, ref section| {
            let new_line = if acc == "" { "" } else { "\n" };
            format!("{}{}{}", acc, new_line, section)
        });

    Ok(code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dart_extensions_single_struct_prims_and_strings() {
        let binding_h = include_str!("../fixtures/prims+strings_binding.h");
        let binding_dart = include_str!("../fixtures/prims+strings_binding.dart");
        let dart = dart_extensions(&binding_h).unwrap();
        assert_eq!(dart, binding_dart.trim_end())
    }

    #[test]
    fn test_dart_extensions_three_structs() {
        let binding_h = include_str!("../fixtures/three_structs_binding.h");
        let binding_dart = include_str!("../fixtures/three_structs_binding.dart");
        let dart = dart_extensions(&binding_h).unwrap();
        println!("{}", &dart);
        assert_eq!(dart, binding_dart.trim_end())
    }
}
