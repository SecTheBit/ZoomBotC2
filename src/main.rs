use reqwest;
mod http;
use http::httpGetRequest;
mod Estruct;
use Estruct::configuration;
mod operation;
mod parser;
use parser::config_parser;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    //parsing the configuration file
    let mut config: configuration = config_parser();
    let mut host : String  = config.endpoint;
    let mut access_token : String = config.access_token;
    let email: String = config.email;
    host.push_str("?to_contact=email@email.com");
    host.push_str("&page_size=1000");
    //println!("host is {}",host);
    http::httpGetRequest(host.as_str(),access_token.as_str()).await?;
    Ok(())    


    
}
