use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::process::Command;

use mditty::utils::*;

use crate::config::*;

// TODO: kind of an issue that this is only working in working directory
pub fn init(config: Config) {
    // labels for sections are mapped to filepaths that will be in those sections
    let (directories_map, entities_map) = build_config_maps(&config);

    let markdown = generate_markdown(&config, &directories_map, &entities_map);

    write_output(&markdown, &PathBuf::from("index.md"));

    md_to_html(&PathBuf::from("index.md"));
}

pub fn build_config_maps(
    config: &Config,
) -> (HashMap<String, Vec<PathBuf>>, HashMap<String, PathBuf>) {
    let mut directories_map: HashMap<String, Vec<PathBuf>> = HashMap::new();
    let mut entities_map: HashMap<String, PathBuf> = HashMap::new();

    if let Some(logo) = &config.main.logo {
        entities_map.insert("logo".to_owned(), PathBuf::from(logo));
    }

    if let Some(directories) = &config.main.directories {
        directory_handler(&directories, &mut directories_map);
    }

    if let Some(entities) = &config.main.entities {
        entity_handler(&entities, &mut entities_map);
    }

    (directories_map, entities_map)
}

pub fn entity_handler(entities: &Vec<Vec<String>>, entities_map: &mut HashMap<String, PathBuf>) {
    // TODO: logic for entity -> md -> html when warranted
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
}

pub fn directory_handler(
    directories: &Vec<Vec<String>>,
    directories_map: &mut HashMap<String, Vec<PathBuf>>,
) {
    // could borrow this from init
    let extension_map = mditty::utils::get_ext_map();

    let extensions: Vec<&String> = extension_map.keys().collect();

    for entry in directories.iter() {
        if entry.len() != 2 {
            panic!(
                "Error in config, directory entries must be of length 2
                   like so: ['Label', 'path'], problem with: {:?}",
                entry
            );
        }

        let mut files: Vec<PathBuf> = Vec::new();
        // Special case: Q2 exports
        if entry[0] == "QIIME2 Exports" {
            let temp_files = find(&PathBuf::from(entry[1].to_owned()), &[&"html".to_owned()]);

            for file in temp_files.iter() {
                if file.file_name() == Path::new("index.html").file_name() {
                    files.push(file.to_owned());
                }
            }
        } else {
            let temp_files: Vec<PathBuf> = find(&PathBuf::from(entry[1].to_owned()), &extensions);

            for file in temp_files.iter() {
                let md_path = file_to_markdown(file, &extension_map);
                let html_path: PathBuf = md_to_html(&md_path).unwrap_or_else(|| {
                    panic!(
                        "pandoc MD to HTML call failed for {}",
                        file.to_str().unwrap()
                    )
                });
                files.push(html_path);
            }
        }

        directories_map.insert(entry[0].to_owned(), files);
    }
}

pub fn md_to_html(md_path: &PathBuf) -> Option<PathBuf> {
    if get_file_extension(md_path) != "md" {
        panic!("utils::to_html: can only accept files with 'md' extension");
    }

    let mut output: Option<PathBuf> = None;

    if pandoc_installed() {
        let mut out_path = md_path.to_owned();
        let _: bool = out_path.set_extension("html");

        let call = Command::new("pandoc")
            .arg("-f")
            .arg("gfm")
            .arg("-t")
            .arg("html")
            .arg("-s")
            .arg("--quiet")
            .arg("-o")
            .arg(&out_path)
            .arg(md_path)
            .status()
            .expect("pandoc failure");

        // TODO: need some additional catches here in case of pandoc problems
        if call.success() {
            output = Some(out_path)
        }
    }

    output
}

pub fn pandoc_installed() -> bool {
    let status = Command::new("pandoc")
        .arg("-v")
        .status()
        .expect("utils::check_for_pandoc: Command::new failed");

    status.success()
}

pub fn generate_markdown(
    config: &Config,
    directories: &HashMap<String, Vec<PathBuf>>,
    entities: &HashMap<String, PathBuf>,
) -> Vec<String> {
    let mut markdown: Vec<String> = Vec::new();

    let mut title: String = "Title".to_owned();

    if let Some(cfg_title) = &config.main.title {
        title = cfg_title.to_owned();
    }

    let mut notes: bool = true;

    if let Some(cfg_notes) = &config.main.notes {
        notes = cfg_notes.to_owned();
    }

    let mut order: Vec<String> = get_default_order(&directories, &entities, &notes);

    if let Some(cfg_order) = &config.main.order {
        // TODO: validation here, order should probably only contain strings that
        // match an entity or dir
        order = dedup_respectfully(cfg_order);
    }

    let extension_map: HashMap<String, String> = get_ext_map();

    for item in order.iter() {
        match item.as_str() {
            // Special cases
            "logo" => write_logo(&mut markdown, &entities),
            "title" => write_title(&mut markdown, &title),
            "notes" => write_notes(&mut markdown, &notes),
            "Metadata" => write_metadata(&mut markdown, &entities),

            // All other cases: either file or directory
            &_ => {
                if directories.contains_key(item) {
                    write_directory(&mut markdown, &directories, &extension_map, item);
                } else if entities.contains_key(item) {
                    write_entity(&mut markdown, &entities, item);
                }
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

// TODO: problem here, name extensions are writing as .html
pub fn write_directory(
    markdown: &mut Vec<String>,
    directories: &HashMap<String, Vec<PathBuf>>,
    extension_map: &HashMap<String, String>,
    label: &str,
) {
    // order items should be validated prior to this step so maybe unwrap_or_else
    // can just be unwrap
    let paths = directories
        .get(label)
        .unwrap_or_else(|| panic!("No '{}' key in directories"));

    let rev_ext_map = reverse_map(extension_map);

    let mut lines: Vec<String> = Vec::new();
    lines.push(format!("## {}\n\n", label));
    lines.push("File | Notes\n--- | ---\n".to_owned());
    for path in paths.iter() {
        let name = get_pretty_name(&path, &rev_ext_map);
        //let name = path.file_name().unwrap_or_else(|| {
        //  panic!(
        //        "Error with file_name() call in utils::write_directory() for {:?}",
        //        path
        //    )
        //});

        lines.push(format!(
            "[{}]({}) | Description\n",
            name,
            path.to_str().unwrap()
        ));
    }

    insert_delimiters(&mut lines, label);
    //markdown.push(format!("<!--/{}-->", label));
    markdown.extend(lines);
}

pub fn write_entity(markdown: &mut Vec<String>, entities: &HashMap<String, PathBuf>, label: &str) {
    // order items should be validated as existing prior to this so that this
    // unwrap_or_else can be jsut changed to unwrap
    let item = entities.get(label).unwrap_or_else(|| {
        panic!("No '{}' key in entities, utils::write_entity()", label);
    });

    // NOTE: currently no conversion to markdown, need to think
    // about how logic to do this could be incorporated
    let file_path = item.to_str().unwrap();
    let file_name = item.file_name().unwrap_or_else(|| {
        panic!(
            "Error with file_name() call in utils::write_directory() for {:?}",
            file_path
        )
    });

    markdown.push(format!(
        "## {}\n[{}]({}) Description\n",
        label,
        file_name.to_str().unwrap(),
        file_path
    ));
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
    let metadata = entities.get("Metadata").unwrap_or_else(|| {
        panic!("No 'metadata' key in entities, utils::write_metadata()");
    });

    let metadata_path = metadata.to_str().unwrap();
    markdown.push(format!(
        "## Metadata\n[This]({}) is the metadata that was used\n",
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

pub fn get_default_order(
    directories: &HashMap<String, Vec<PathBuf>>,
    entities: &HashMap<String, PathBuf>,
    notes: &bool,
) -> Vec<String> {
    let mut order: Vec<String> = Vec::new();

    if entities.contains_key("logo") {
        order.push("logo".to_owned());
    }

    // NOTE: this is based on the logic in generate_markdown where a title
    // will be present no matter what
    order.push("title".to_owned());

    if *notes {
        order.push("notes".to_owned());
    }

    let mut remaining_sections: Vec<String> = directories.keys().map(|k| k.to_owned()).collect();
    remaining_sections.extend(
        entities
            .keys()
            .filter(|k| k != &&"logo".to_owned())
            .map(|k| k.to_owned())
            .collect::<Vec<String>>(),
    );

    remaining_sections.sort();

    order.extend(remaining_sections);

    order
}

// inserts delimiters for update function later
pub fn insert_delimiters(lines: &mut Vec<String>, label: &str) {
    lines.insert(1, format!("<!---{}--->\n", label));
    lines.push(format!("<!---/{}--->\n", label));
}

// greedily dedups with (greedy) respect for original order
pub fn dedup_respectfully(slice_in: &[String]) -> Vec<String> {
    let mut out = Vec::new();

    for item in slice_in.iter() {
        if !out.contains(item) {
            out.push(item.to_owned());
        }
    }

    out
}

// expected behavior: gnu find, right?
pub fn find(parent_dir: &PathBuf, target_extensions: &[&String]) -> Vec<PathBuf> {
    let mut found: Vec<PathBuf> = Vec::new();
    let mut dirs_to_search: Vec<PathBuf> = vec![parent_dir.to_path_buf()];

    while !dirs_to_search.is_empty() {
        let current_dir = dirs_to_search.pop().unwrap();

        for entry in current_dir.read_dir().expect("read_dir call failure") {
            if let Ok(entry) = entry {
                // TODO: ignore hidden
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

// this seems a kind of dumb way to determine the prior extension
// might be better to just store it before
//
// TODO this is so bad
//
// also a bad function name, doesn't really make it pretty
pub fn get_pretty_name(path_in: &PathBuf, rev_ext_map: &HashMap<String, String>) -> String {
    let mut name = String::new();

    let mut path = path_in.to_owned();

    path.set_extension("md");

    if path.exists() {
        let file = File::open(&path).unwrap_or_else(|why| {
            panic!("Could not open {}: {}", path.to_str().unwrap(), why);
        });
        let mut buf_reader = BufReader::new(file);

        let mut first_line = String::new();
        buf_reader.read_line(&mut first_line);

        if first_line.starts_with("```") {
            let file_type = first_line.trim_start_matches("`").trim();

            let extension = rev_ext_map.get(file_type).unwrap_or_else(|| {
                panic!("Problem at utils::get_pretty_name");
            });
            let mut temp_path = path.to_owned();
            temp_path.set_extension(extension);
            name = temp_path.file_name().unwrap().to_str().unwrap().to_owned();
        } else {
            name = path.file_name().unwrap().to_str().unwrap().to_owned();
        }
    } else {
        path = path_in.to_owned();
        if path.file_name().unwrap().to_str().unwrap() == "index.html" {
            name = path.parent().unwrap().file_name().unwrap().to_str().unwrap().to_owned();
        } else {
            name = path.file_name().unwrap().to_str().unwrap().to_owned();
        }
    }

    name
}

pub fn reverse_map(map: &HashMap<String, String>) -> HashMap<String, String> {
    let mut map_out = HashMap::new();

    for (key, val) in map.iter() {
        map_out.insert(val.to_owned(), key.to_owned());
    }

    map_out
}
