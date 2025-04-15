use std::{fs::File, path::Path};
use serde::Deserialize;
use crate::Estruct::configuration;

pub fn config_parser() -> configuration{
    let cfile = Path::new("C:\\Users\\kakashi\\ZoomBotC2\\src\\config.json");
    let file = File::open(cfile).expect("error in readig config file");
    let config : configuration = serde_json::from_reader(file).expect("error while reading the config file");
    println!("{:?}",config);
    config

}