extern crate clap;

use informer::config::*;
use informer::utils::*;
use informer::update::*;

use std::env;
use std::path::{Path, PathBuf};

use clap::{App, Arg};

fn main() {
    let current_dir = env::current_dir().unwrap_or_else(|why| {
        panic!("Could not determine the current working directory: {}", why);
    });

    let matches = App::new("informer")
        .version("0.1")
        .author("William Gasper <wkg@williamgasper.com>")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .default_value(current_dir.to_str().unwrap())
                .long_help(
                    "Input directory. If not provided, the 
                current working directory will be used",
                ),
        )
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                // TODO: FIX THIS
                .default_value("")
                .long_help("Path to a TOML config file, see sample_config.toml for an example"),
        )
        .arg(
            Arg::with_name("update")
                .short("u")
                .long("update")
                .long_help("Update an existing index")
        )
        .get_matches();

    let mut cfg: Config = get_default_config();

    if matches.is_present("config") {
        let cfg_path_str: &str = matches.value_of("config").unwrap();
        let cfg_path = PathBuf::from(cfg_path_str);
        cfg = load_config(&cfg_path);
    }
    
    if matches.is_present("update") {
        update(cfg);
    } else {
        init(cfg);
    }
}
