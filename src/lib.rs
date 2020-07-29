pub mod config;
pub mod utils;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::PathBuf;
    use std::process::Command;

    use crate::config::*;
    use crate::utils::*;

    #[test]
    fn test_pandoc_to_html() {
        let out = md_to_html(&PathBuf::from("./test/test.md"));
        println!("{:?}", out);
        assert_eq!(out, Some(PathBuf::from("./test/test.html")));
    }

    #[test]
    fn get_default_config_0() {
        let cfg = get_default_config();

        println!("{:?}", cfg.main.directories);
        assert_eq!(cfg.main.title, Some("Title placeholder".to_owned()));
        assert_eq!(cfg.main.notes, Some(true));
    }

    #[test]
    fn find_0() {
        let mut its = find(&PathBuf::from("."), &[&"rs".to_owned()]);

        let mut expected = vec![
            PathBuf::from("./src/config.rs"),
            PathBuf::from("./src/main.rs"),
            PathBuf::from("./src/utils.rs"),
            PathBuf::from("./src/lib.rs"),
        ];

        its.sort();
        expected.sort();

        assert_eq!(its, expected);
    }

    #[test]
    fn load_config_0() {
        let cfg = load_config(&PathBuf::from("sample_config.toml"));
        assert_eq!(cfg.main.title, Some("Title placeholder".to_owned()));
        assert_eq!(cfg.main.notes, Some(true));
    }

    #[test]
    fn dedup_respectfully_0() {
        let temp = vec!["this", "is", "this", "is", "a", "test", "is"];
        let this_vec: Vec<String> = temp.iter().map(|i| i.to_owned().to_owned()).collect();

        let temp_e = vec!["this", "is", "a", "test"];
        let expected: Vec<String> = temp_e.iter().map(|i| i.to_owned().to_owned()).collect();

        assert_eq!(expected, dedup_respectfully(&this_vec));
    }

    #[test]
    fn dedup_respectfully_1() {
        let temp = vec!["this", "is", "a", "test"];
        let this_vec: Vec<String> = temp.iter().map(|i| i.to_owned().to_owned()).collect();

        let temp_e = vec!["this", "is", "a", "test"];
        let expected: Vec<String> = temp_e.iter().map(|i| i.to_owned().to_owned()).collect();

        assert_eq!(expected, dedup_respectfully(&this_vec));
    }

    #[test]
    fn get_default_order_0() {
        let mut directories: HashMap<String, Vec<PathBuf>> = HashMap::new();
        let mut entities: HashMap<String, PathBuf> = HashMap::new();
        let notes: bool = true;

        directories.insert("b".to_owned(), Vec::new());
        directories.insert("d".to_owned(), Vec::new());
        entities.insert("a".to_owned(), PathBuf::new());
        entities.insert("c".to_owned(), PathBuf::new());
        entities.insert("logo".to_owned(), PathBuf::new());

        let temp = vec!["logo", "title", "notes", "a", "b", "c", "d"];
        let expected: Vec<String> = temp.iter().map(|i| i.to_owned().to_owned()).collect();

        let actual = get_default_order(&directories, &entities, &notes);

        assert_eq!(expected, actual);
    }
}
