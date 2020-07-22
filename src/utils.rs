
// expected behavior: gnu find, right?
pub fn find(parent_dir: &PathBuf, target_extension: &String) -> Vec<PathBuf> {
    let mut found: Vec<PathBuf> = Vec::new();
    let mut dirs_to_search: Vec<PathBuf> = vec![input_path.to_path_buf()];

    while !dirs_to_search.is_empty() {
        let current_dir = dirs_to_search.pop().unwrap();

        for entry in current_dir.read_dir().expect("read_dir call failure") {
            if let Ok(entry) = entry {
                let entry_path = entry.path();

                if entry_path.is_file() {
                    let extension = get_file_extension(&entry_path); 
                    if extension == target_extension {
                        found.push(entry_path.canonicalize().unwrap_or_else(|why| {
                            panic!("Could not resolve path: {:?} {}", entry_path,
                                   why);
                        }));
                    }
                } else {
                    dirs_to_search.push(entry_path);
                }
            }
        }
    }

    found
}

pub fn get_file_extension(file_path: &PathBuf) -> &str {
   let extension = file_path.extension().unwrap_or(OsStr::new(""));

   extension.to_str().unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_file_extension_0() {
        assert_eq!(get_file_extension(&PathBuf::from("test.rs")), "rs");
    }
    
    #[test]
    fn test_get_file_extension_0() {
        assert_eq!(get_file_extension(&PathBuf::from("test")), "");
    }
}
