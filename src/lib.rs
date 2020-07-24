pub mod config;
pub mod utils;

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::config::*;
    use crate::utils::*;
    //extern crate mditty;
    use mditty::utils::get_file_extension;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_using_mditty() {
        assert_eq!(get_file_extension(&PathBuf::from("test.rs")), "rs");
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
        let its = find(&PathBuf::from("."), &[&"rs".to_owned()]);

        let expected = vec![
            PathBuf::from("./src/config.rs"),
            PathBuf::from("./src/utils.rs"),
            PathBuf::from("./src/lib.rs"),
        ];

        assert_eq!(its, expected);
    }
}
