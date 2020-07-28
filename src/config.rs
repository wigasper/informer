extern crate toml;

use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use toml::from_str;

pub enum OutputFormat {
    HTML,
    Markdown,
}
// TODO: bools should probably be Strings that are then parsed to allow for
// fuzzy language usage
#[derive(Deserialize)]
pub struct MainCfg {
    pub title: Option<String>,
    pub logo: Option<String>,
    pub notes: Option<bool>,
    pub entities: Option<Vec<Vec<String>>>,
    pub directories: Option<Vec<Vec<String>>>,
    pub order: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct Config {
    pub main: MainCfg,
}

pub fn load_config(file_path: &PathBuf) -> Config {
    let mut file = File::open(file_path.as_path()).unwrap_or_else(|why| {
        panic!(
            "Could not open config file: {}, why: {}",
            file_path.to_str().unwrap(),
            why
        );
    });
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap_or_else(|why| {
        panic!(
            "Could not read config file: {}, why: {}",
            file_path.to_str().unwrap(),
            why
        );
    });

    toml::from_str(contents.as_str()).unwrap()
}

pub fn get_default_config() -> Config {
    let config: Config = toml::from_str(
        r#"
    [main]
    title = 'Title placeholder'
    logo = './logo.png'
    notes = true
   
    # Distinct entities that are given their own 
    # section with a link
    # Format: ['Label', 'Path']
    entities = [
        ['Metadata', './metadata.tsv']
    ]
    
    # Directories that will be searched
    # Format: ['Label', 'Path']
    directories = [
        ['Pipelines', './pipelines'],
        ['Scripts', './scripts'],
        ['QIIME2 Exports', './exports'],
        ['Notebooks', './notebooks']
    ]

    order = [
            'logo', 
            'title', 
            'notes', 
            'Metadata',
            'Scripts',
            'Pipelines',
            'Notebooks',
            'QIIME2 Exports',
    ]

    
    "#,
    )
    .unwrap();

    config
}
