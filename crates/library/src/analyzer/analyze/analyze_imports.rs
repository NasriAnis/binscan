use std::collections::HashSet;
use crate::FileType;

#[derive( Debug )]
pub struct Import {
    libraries: String,
    functions: Vec<String>
}

pub fn run(imports: &Vec<String>, file_type: FileType) -> Result<Vec<Import>, std::io::Error> {
    let mut results: Vec<Import> = Vec::new();
    
    if file_type == FileType::PE {
        let mut vec_func: Vec<String> = Vec::new();
        let mut seen_lib: HashSet<String> = HashSet::new();
        let mut seen_func: HashSet<String> = HashSet::new();
        let mut import: Import;
        let mut prev_lib: &str = "";
        let mut first_entry = true;
    
        for i in imports{
            if let Some((func, lib)) = i.trim().split_once(' '){
                if seen_lib.contains(lib){
                    if seen_func.contains(func) == false {
                        seen_func.insert(func.to_string());
                        vec_func.push(func.to_string());
                        prev_lib = lib;
                   } else {
                       prev_lib = lib;
                       continue;
                   }
                } else {
                    seen_lib.clear();
                    seen_func.clear();
                    seen_lib.insert(lib.to_string());
    
                    if first_entry {
                        seen_lib.insert(lib.to_string());
                        first_entry = false;
                    } else {
                        import = Import {
                            libraries: prev_lib.to_string(),
                            functions: vec_func.clone(),
                        };
                        results.push(import);
                        vec_func.clear();
                    }
                }
            }
        }
    } else if file_type == FileType::ELF {
        let mut vec_func: Vec<String> = Vec::new();
        let mut seen_lib: HashSet<String> = HashSet::new();
        let mut seen_func: HashSet<String> = HashSet::new();
        let mut import: Import;
        let mut prev_lib: &str = "";
        let mut first_entry = true;
    
        for i in imports{
            if let Some((func, lib)) = i.trim().split_once("@@"){
                if seen_lib.contains(lib){
                    if seen_func.contains(func) == false {
                        seen_func.insert(func.to_string());
                        vec_func.push(func.to_string());
                        prev_lib = lib;
                   } else {
                       prev_lib = lib;
                       continue;
                   }
                } else {
                    seen_lib.clear();
                    seen_func.clear();
                    seen_lib.insert(lib.to_string());
    
                    if first_entry {
                        seen_lib.insert(lib.to_string());
                        first_entry = false;
                    } else {
                        import = Import {
                            libraries: prev_lib.to_string(),
                            functions: vec_func.clone(),
                        };
                        results.push(import);
                        vec_func.clear();
                    }
                }
            }
        }
    }
    Ok(results)
}