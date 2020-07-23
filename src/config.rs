extern crate toml;

use serde::Deserialize;

use toml::from_str;



#[derive(Deserialize)]
pub struct Config {
    pub title: Option<String>,
    pub logo: Option<String>,
    pub metadata: Option<String>,
    pub scripts: Option<String>,
    pub q2exports: Option<String>,
    pub notebooks: Option<String>,
}

pub fn get_default_config() -> Config {
    
    let config: Config = toml::from_str(r#"
    title = 'Title placeholder'
    logo = './logo.png'
    metadata = './metadata.tsv'
    scripts = './scripts'
    q2exports = './exports'
    notebooks = './notebooks'
    "#).unwrap();

    config
}
