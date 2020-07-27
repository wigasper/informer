extern crate toml;

use serde::Deserialize;

use toml::from_str;

// TODO: bools should probably be Strings that are then parsed to allow for
// different options
#[derive(Deserialize)]
pub struct MainCfg {
    pub title: Option<String>,
    pub logo: Option<String>,
    pub notes: Option<bool>,
    pub entities: Option<Vec<Vec<String>>>,
    pub directories: Option<Vec<Vec<String>>>,
    //pub metadata: Option<String>,
    //pub pipelines: Option<String>,
    //pub scripts: Option<String>,
    //pub q2exports: Option<String>,
    //pub notebooks: Option<String>,
    pub order: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct Config {
    pub main: MainCfg,
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
