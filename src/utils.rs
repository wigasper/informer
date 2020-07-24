use mditty::utils::*;

use std::path::PathBuf;
use std::collections::HashMap;

use crate::config::*;

// TODO: this needs to be able to update an existing report
// updates should only affect certain sections


// pandoc probably best at the end, in case 
pub fn builder(config: Config) {
    //let mut output_data: HashMap<String, Vec<String>> = parse_config(config);
    
    // labels for sections are mapped to filepaths that will be in those sections
    let mut directories_map: HashMap<String, Vec<PathBuf>> = HashMap::new();
    let mut entities_map: HashMap<String, PathBuf> = HashMap::new();
    
    

    if let Some(directories) = config.main.directories {
        directories_map = directory_handler(&directories);
    }

    if let Some(entities) = config.main.entities {
        entities_map = entity_handler(&entities);
    }


}

pub fn entity_handler(entities: &Vec<Vec<String>>) -> HashMap<String, PathBuf> {
    let map_out = HashMap::new();

    map_out
}

pub fn directory_handler(directories: &Vec<Vec<String>>) -> HashMap<String, Vec<PathBuf>> {
    let map_out = HashMap::new();
    
    let extension_map = mditty::utils::get_map();
    let extensions: Vec<&String> = extension_map.keys().collect();

    for entry in directories.iter() {
        if entry.len() != 2 {
            panic!("Error in config, directory entries must be of length 2
                   like so: ['Label', 'path'], problem with: {:?}", entry);
        }
        
        let files = find(&PathBuf::from(entry[1].to_owned()), &extensions);

        map_out.insert(entry[0].to_owned(), files);
    }

    map_out
}
        //markdown.push(format!(
        //    "<p align='center'>\n\t<img src='{}'/>\n</p>\n\n",
        //    logo_path
        //));
    //}
/*
    if let Some(title) = config.main.title {
        //markdown.push(format!("# {}\n\n", title));
    }

    let mut notes_section: bool = false;
    if let Some(notes) = config.main.notes {
        if notes {
            notes_section = true;
            //markdown.push("## Notes\n* This is a note\n\n".to_owned());
        }
    }
    */
/*
    if let Some(metadata) = config.main.metadata {
        //markdown.push(format!(
        //    "[This]({}) is the metadata that was used",
        //    metadata
        //));
    }

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
    }

    if let Some(notebooks) = config.main.notebooks {
        // jupyter notebooks: jupyter nbconvert --to html x.ipynb
        // r notebooks: Rscript -e "rmarkdown::render('fp.Rmd')"
        // need header checks on R notebooks
        //TODO: this should be elsewhere, ... or?
        let rmd = "Rmd".to_owned();
        let ipynb = "ipynb".to_owned();
        
        let notebook_exts = vec![&rmd, &ipynb];

        let notebook_paths = find(&PathBuf::from(&notebooks), &notebook_exts);
        if notebook_paths.len() > 0 {
            // TODO: all these titles could be specified, realistically
            markdown.push("## Notebooks\n\n".to_owned());
            markdown.push("File | Notes\n--- | ---\n".to_owned());
            for notebook_path in notebook_paths.iter() {
                let name = notebook_path.file_name().unwrap_or_else(|| {
                    panic!(
                        "Error with file_name() call in config::builder() for {:?}",
                        notebook_path
                    )
                });

                let new_path = file_to_markdown(&notebook_path, &extension_map);
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
*/
//}

// expected behavior: gnu find, right?
pub fn find(parent_dir: &PathBuf, target_extensions: &[&String]) -> Vec<PathBuf> {
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
