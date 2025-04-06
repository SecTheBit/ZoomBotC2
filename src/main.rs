use std::{fs::File, path::Path};
use serde::Deserialize;
use reqwest;
mod http;
use http::httpGetRequest;
mod Estruct;
use Estruct::configuration;
mod operation;


fn config_parser() -> configuration{
    let cfile = Path::new("C:\\Users\\Divyanshu\\ZoomBotC2\\src\\config.json");
    let file = File::open(cfile).expect("error in readig config file");
    let config : configuration = serde_json::from_reader(file).expect("error while reading the config file");
    println!("{:?}",config);
    config

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    //parsing the configuration file
    let mut config: configuration = config_parser();
    let mut host : String  = config.endpoint;
    let mut access_token : String = config.access_token;
    let email: String = config.email;
    host.push_str("?to_contact=jaleh91134@dwriters.com");
    //println!("host is {}",host);
    http::httpGetRequest(host.as_str(),access_token.as_str()).await?;
    Ok(())    


    
}
