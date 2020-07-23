use mditty::utils::get_file_extension;
use std::path::PathBuf;

use crate::config::*;

pub fn builder(config: Config) {
    // pass this to it
    //let config: Config = get_default_config();
    
    let mut markdown: Vec<String> = Vec::new();
    
    //let logo_path = config.logo;
    if let Some(logo_path) = config.logo {
        markdown.push(format!("<p align='center'>\n\t<img src='{}'/>\n</p>", logo_path).to_owned()); 
    }

    if let Some
    // get list of sections, create map
    // default sections: [title, intro, notes, metadata, utility scripts, pipelines,
    // notebooks, qiime2 reports, qiime2 raw data]
    // pull this from default config, so
    // get default config
    //
    // get data for each section
    //
    // pretty output
}

// expected behavior: gnu find, right?
pub fn find(parent_dir: &PathBuf, target_extensions: &Vec<String>) -> Vec<PathBuf> {
    let mut found: Vec<PathBuf> = Vec::new();
    let mut dirs_to_search: Vec<PathBuf> = vec![parent_dir.to_path_buf()];

    while !dirs_to_search.is_empty() {
        let current_dir = dirs_to_search.pop().unwrap();

        for entry in current_dir.read_dir().expect("read_dir call failure") {
            if let Ok(entry) = entry {
                let entry_path = entry.path();

                if entry_path.is_file() {
                    let extension = get_file_extension(&entry_path).to_owned(); 
                    if target_extensions.contains(&extension) {
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

