use crate::FileType;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Import {
    pub libraries: String,
    pub functions: Vec<String>,
}

pub fn run(imports: &Vec<String>, file_type: FileType) -> Result<Vec<Import>, std::io::Error> {
    if file_type == FileType::PE {
        Ok(parse(imports, &" ".to_string()))
    } else if file_type == FileType::ELF {
        Ok(parse(imports, &"@@".to_string()))
    } else {
        panic!()
    }
}

fn parse(imports: &Vec<String>, split: &String) -> Vec<Import> {
    let mut results: Vec<Import> = Vec::new();
    let mut seen_lib: HashSet<String> = HashSet::new();

    for i in imports {
        if let Some((func, lib)) = i.trim().split_once(split) {
            if !seen_lib.contains(lib) {
                seen_lib.insert(lib.to_string());
                results.push(Import {
                    libraries: lib.to_string(),
                    functions: vec![func.to_string()],
                });
            } else if seen_lib.contains(lib)
                && let Some(r) = results.iter_mut().find(|r| r.libraries == lib)
                && !r.functions.contains(&func.to_string())
            {
                r.functions.push(func.to_string());
            }
        }
    }
    results
}
