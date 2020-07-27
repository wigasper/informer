use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use mditty::utils::*;

use crate::config::*;

// TODO: this needs to be able to update an existing report
// updates should only affect certain sections

// pandoc probably best at the end, in case
pub fn init(config: Config) {
    //let mut output_data: HashMap<String, Vec<String>> = parse_config(config);

    // labels for sections are mapped to filepaths that will be in those sections
    let mut directories_map: HashMap<String, Vec<PathBuf>> = HashMap::new();
    let mut entities_map: HashMap<String, PathBuf> = HashMap::new();

    if let Some(logo) = config.main.logo.to_owned() {
        entities_map.insert("logo".to_owned(), PathBuf::from(logo));
    }

    if let Some(directories) = config.main.directories.to_owned() {
        directories_map = directory_handler(&directories);
    }

    if let Some(entities) = config.main.entities.to_owned() {
        entity_handler(&entities, &mut entities_map);
    }

    let markdown: Vec<String> = generate_markdown(config, &directories_map, &entities_map);

    write_output(&markdown, &PathBuf::from("index.md"));
}

pub fn entity_handler(entities: &Vec<Vec<String>>, entities_map: &mut HashMap<String, PathBuf>) {
    //let mut map_out = HashMap::new();

    // maybe need to add title and logo here since they are valid entities
    for entity in entities.iter() {
        if entity.len() != 2 {
            panic!(
                "Error in config, entity entries must be of length 2
                  like so: ['Label', 'path'], problem with: {:?}",
                entity
            );
        }

        entities_map.insert(entity[0].to_owned(), PathBuf::from(entity[1].to_owned()));
    }

    //map_out
}

pub fn directory_handler(directories: &Vec<Vec<String>>) -> HashMap<String, Vec<PathBuf>> {
    let mut map_out = HashMap::new();

    // could borrow this from init
    let extension_map = mditty::utils::get_ext_map();
    // Need to deal with special cases, i.e. Q2 exports directories should only
    // get HTML
    let extensions: Vec<&String> = extension_map.keys().collect();

    for entry in directories.iter() {
        if entry.len() != 2 {
            panic!(
                "Error in config, directory entries must be of length 2
                   like so: ['Label', 'path'], problem with: {:?}",
                entry
            );
        }

        let files = find(&PathBuf::from(entry[1].to_owned()), &extensions);

        map_out.insert(entry[0].to_owned(), files);
    }

    map_out
}

pub fn generate_markdown(
    config: Config,
    directories: &HashMap<String, Vec<PathBuf>>,
    entities: &HashMap<String, PathBuf>,
) -> Vec<String> {
    let mut markdown: Vec<String> = Vec::new();
    let mut order: Vec<String> = get_default_order();

    if let Some(cfg_order) = config.main.order {
        order = cfg_order;
    }

    let mut title: String = "Title".to_owned();

    if let Some(cfg_title) = config.main.title {
        title = cfg_title;
    }

    let mut notes: bool = true;

    if let Some(cfg_notes) = config.main.notes {
        notes = cfg_notes;
    }

    let extension_map: HashMap<String, String> = get_ext_map();
    // TODO: ensure no duplicates in order container

    for item in order.iter() {
        match item.as_str() {
            "logo" => write_logo(&mut markdown, &entities),
            "title" => write_title(&mut markdown, &title),
            "notes" => write_notes(&mut markdown, &notes),
            "Metadata" => write_metadata(&mut markdown, &entities),
            // probably don't need any of this logic, just use the any case logic
            "Scripts" => write_directory(&mut markdown, &directories, &extension_map, item),
            "Pipelines" => write_directory(&mut markdown, &directories, &extension_map, item),
            "Notebooks" => write_directory(&mut markdown, &directories, &extension_map, item),
            "QIIME2 Exports" => write_directory(&mut markdown, &directories, &extension_map, item),
            &_ => {
                // use logic to determine what to do, for testing: write dir
                write_directory(&mut markdown, &directories, &extension_map, item)
            }
        }
    }

    markdown
}

pub fn write_output(markdown: &Vec<String>, out_path: &PathBuf) {
    let mut out_buffer = File::create(&out_path).unwrap_or_else(|why| {
        panic!("Could not create output file: {}", why);
    });

    for item in markdown.iter() {
        out_buffer.write(item.as_bytes()).unwrap_or_else(|why| {
            panic!("utils::write_output, out_buffer could not write: {}", why);
        });
    }

    out_buffer.flush().unwrap();
}

pub fn write_directory(
    markdown: &mut Vec<String>,
    directories: &HashMap<String, Vec<PathBuf>>,
    extension_map: &HashMap<String, String>,
    title: &str,
) {
    let paths = directories
        .get(title)
        .unwrap_or_else(|| panic!("No '{}' key in directories"));

    markdown.push(format!("## {}\n\nFile | Notes\n--- | ---\n", title));
    for path in paths.iter() {
        let name = path.file_name().unwrap_or_else(|| {
            panic!(
                "Error with file_name() call in utils::write_directory() for {:?}",
                path
            )
        });

        let new_path = file_to_markdown(&path, extension_map);
        println!("new_path: {}", new_path.to_str().unwrap());
        markdown.push(format!(
            "[{}]({}) | Description\n",
            name.to_str().unwrap(),
            new_path
                .to_str()
                .unwrap_or_else(|| { panic!("utils::write_directory, new_path is None") })
        ));
    }
}

pub fn write_notes(markdown: &mut Vec<String>, notes: &bool) {
    if notes.to_owned() {
        markdown.push("## Notes\n* This is a note\n\n".to_owned());
    }
}

pub fn write_title(markdown: &mut Vec<String>, title: &String) {
    markdown.push(format!("# {}\n\n", title));
}

pub fn write_metadata(markdown: &mut Vec<String>, entities: &HashMap<String, PathBuf>) {
    let metadata = entities.get("metadata").unwrap_or_else(|| {
        panic!("No 'metadata' key in entities, utils::write_metadata()");
    });

    let metadata_path = metadata.to_str().unwrap();
    markdown.push(format!(
        "## Metadata\n[This]({}) is the metadata that was used",
        metadata_path
    ));
}

pub fn write_logo(markdown: &mut Vec<String>, entities: &HashMap<String, PathBuf>) {
    let logo = entities.get("logo").unwrap_or_else(|| {
        panic!("No 'logo' key in entities, utils::generate_markdown()");
    });

    let logo_path = logo.to_str().unwrap();
    markdown.push(format!(
        "<p align='center'>\n\t<img src='{}'/>\n</p>\n\n",
        logo_path
    ));
}

pub fn get_default_order() -> Vec<String> {
    let temp: Vec<&str> = vec![
        "logo",
        "title",
        "notes",
        "Metadata",
        "Scripts",
        "Pipelines",
        "Notebooks",
        "QIIME2 Exports",
    ];
    temp.iter().map(|i| i.to_owned().to_owned()).collect()
}

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
