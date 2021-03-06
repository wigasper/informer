use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use std::process::Command;

use crate::config::*;
use crate::utils::*;
use mditty::utils::*;

use regex::Regex;

// TODO: need logic that looks at config and if there is a section that
// needs to be added it will add it

pub fn update(config: Config) {
    // check for index, if not exists, quit
    let index_path = PathBuf::from("index.md");
    if !index_path.exists() {
        panic!("{} does not exist", index_path.to_str().unwrap());
    }

    let temp_index_path = PathBuf::from(format!(".tmp.{}", index_path.to_str().unwrap()));

    // move index to temp
    let mut call = Command::new("mv")
        .arg(&index_path)
        .arg(&temp_index_path)
        .spawn()
        .expect("mv failed");

    let markdown = get_updated_markdown(&temp_index_path, &config);

    write_output(&markdown, &index_path);
    
    md_to_html(&PathBuf::from("index.md"));

    call = Command::new("rm")
        .arg(&temp_index_path)
        .spawn()
        .expect("rm failed");
}

pub fn get_updated_markdown(index_path: &PathBuf, config: &Config) -> Vec<String> {
    let (directories_map, entities_map) = build_config_maps(config);
    // compile regexes
    let delim_start = Regex::new(r"^<!---([^-]*)--->$").unwrap();
    let delim_stop = Regex::new(r"^<!---/.*--->$").unwrap();

    let mut markdown: Vec<String> = Vec::new();
    // read in current index line for line
    let file = File::open(&index_path).unwrap_or_else(|why| {
        panic!("Could not open {}: {}", index_path.to_str().unwrap(), why);
    });

    let mut delim_flag: bool = false;
    let mut delim_lines: Vec<String> = Vec::new();
    let mut section_label: String = "".to_owned();

    let buf_reader = BufReader::new(file);

    for line in buf_reader.lines() {
        let mut this_line = line.unwrap();

        if delim_flag {
            if delim_stop.is_match(&this_line.as_str()) {
                // handle these lines
                section_handler(
                    &delim_lines,
                    &section_label,
                    &mut markdown,
                    &directories_map,
                );
                // reset
                delim_flag = false;
                delim_lines = Vec::new();
                // probably can delete:
                section_label = "".to_owned();
            } else {
                //this_line.push_str("\n");
                delim_lines.push(this_line);
            }
        } else if delim_start.is_match(&this_line.as_str()) {
            delim_flag = true;
            let caps = delim_start.captures(&this_line.as_str()).unwrap();
            section_label = caps.get(1).unwrap().as_str().to_owned();
        } else {
            this_line.push_str("\n");
            markdown.push(this_line);
        }
    }
    markdown
}

pub fn section_handler(
    lines: &Vec<String>,
    label: &String,
    markdown: &mut Vec<String>,
    directories: &HashMap<String, Vec<PathBuf>>,
) {
    // can pass this probably to avoid creating every time
    // this does not work
    let path_regex = Regex::new(r"^\[.*\]\((.*)\)\s\|.*").unwrap();

    let mut lines_out: Vec<String> = Vec::new();

    if directories.contains_key(label) {
        let mut needed_paths: Vec<PathBuf> = directories.get(label).unwrap().to_owned();
        let current_items: Vec<String> = Vec::new();

        for line in lines.iter() {
            if path_regex.is_match(line) {
                let caps = path_regex.captures(line).unwrap();
                let this_path = caps.get(1).unwrap().as_str();
                needed_paths.retain(|x| x.to_str().unwrap() != this_path);
                lines_out.push(format!("{}\n", line.to_owned()));
            } else {
                lines_out.push(format!("{}\n", line.to_owned()));
            }
        }
        
        let extension_map = get_ext_map();
        let rev_ext_map = reverse_map(&extension_map);

        // this is repeated
        for path in needed_paths.iter() {
            let name = get_pretty_name(&path, &rev_ext_map);
            lines_out.push(format!(
                "[{}]({}) | Description\n",
                name,
                path.to_str().unwrap()
            ));
        }

        insert_delimiters(&mut lines_out, label);
    } else {
        lines_out.extend(lines.to_owned());
        insert_delimiters(&mut lines_out, label);
    }

    markdown.extend(lines_out);
}
