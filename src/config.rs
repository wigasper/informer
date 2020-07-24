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
    pub metadata: Option<String>,
    pub scripts: Option<String>,
    pub q2exports: Option<String>,
    pub notebooks: Option<String>,
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
    metadata = './metadata.tsv'
    scripts = './scripts'
    q2exports = './exports'
    notebooks = './notebooks'
    "#,
    )
    .unwrap();

    config
}
