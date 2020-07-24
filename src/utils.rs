use mditty::utils::*;
use std::path::PathBuf;

use crate::config::*;

// TODO: this needs to be able to update an existing report

pub fn builder(config: Config) {
    // pass this to it
    //let config: Config = get_default_config();

    let mut markdown: Vec<String> = Vec::new();

    // extension map required for markdownifying
    let extension_map = get_map();

    if let Some(logo_path) = config.main.logo {
        markdown.push(format!(
            "<p align='center'>\n\t<img src='{}'/>\n</p>\n\n",
            logo_path
        ));
    }

    if let Some(title) = config.main.title {
        markdown.push(format!("# {}\n\n", title));
    }

    if let Some(notes) = config.main.notes {
        if notes {
            markdown.push("## Notes\n* This is a note\n\n".to_owned());
        }
    }

    if let Some(metadata) = config.main.metadata {
        markdown.push(format!(
            "[This]({}) is the metadata that was used",
            metadata
        ));
    }

    // TODO: pandoc integration may be best at this stage
    if let Some(scripts) = config.main.scripts {
        // TODO: extensions need to be specified as user input
        let temp_ext_0 = "slurm".to_owned();
        let exts_seq_analysis = vec![&temp_ext_0];

        let slurm_scripts = find(&PathBuf::from(&scripts), &exts_seq_analysis);
        if slurm_scripts.len() > 0 {
            markdown.push("## Sequence Analysis Pipelines\n\n".to_owned());
            markdown.push("File | Notes\n--- | ---\n".to_owned());
            for script_path in slurm_scripts.iter() {
                let name = script_path.file_name().unwrap_or_else(|| {
                    panic!(
                        "Error with file_name() call in config::builder() for {:?}",
                        script_path
                    )
                });

                let new_path = file_to_markdown(&script_path, &extension_map);
                markdown.push(format!(
                    "[{}]({}) | \n",
                    name.to_str().unwrap(),
                    new_path.to_str().unwrap()
                ));
            }
        }

        let exts_other: Vec<&String> = extension_map
            .keys()
            .filter(|k| k != &&"slurm".to_owned())
            .collect();

        let other_scripts = find(&PathBuf::from(&scripts), &exts_other);
        if other_scripts.len() > 0 {
            // TODO: all these titles could be specified, realistically
            markdown.push("## Other Scripts\n\n".to_owned());
            markdown.push("File | Notes\n--- | ---\n".to_owned());
            for script_path in other_scripts.iter() {
                let name = script_path.file_name().unwrap_or_else(|| {
                    panic!(
                        "Error with file_name() call in config::builder() for {:?}",
                        script_path
                    )
                });

                let new_path = file_to_markdown(&script_path, &extension_map);
                markdown.push(format!(
                    "[{}]({}) | \n",
                    name.to_str().unwrap(),
                    new_path.to_str().unwrap()
                ));
            }
        }
    }

    //if let Some()
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
pub fn find(parent_dir: &PathBuf, target_extensions: &Vec<&String>) -> Vec<PathBuf> {
    let mut found: Vec<PathBuf> = Vec::new();
    let mut dirs_to_search: Vec<PathBuf> = vec![parent_dir.to_path_buf()];

    while !dirs_to_search.is_empty() {
        let current_dir = dirs_to_search.pop().unwrap();

        for entry in current_dir.read_dir().expect("read_dir call failure") {
            if let Ok(entry) = entry {
                let entry_path = entry.path();

                if entry_path.is_file() {
                    let extension = get_file_extension(&entry_path).to_owned();
                    if target_extensions.contains(&&extension) {
                        found.push(entry_path);
                    }
                } else {
                    dirs_to_search.push(entry_path);
                }
            }
        }
    }

    found
}
