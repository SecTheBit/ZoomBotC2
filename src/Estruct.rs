use serde::{Deserialize, Serialize};
use std::{fs::File, path::Path};
//enums and structs will be defined here

#[derive(Debug, Deserialize)]
pub struct configuration {
    pub endpoint: String,
    pub access_token: String,
    pub email: String,
}



#[derive(Deserialize)]
pub struct Root {
    date: String,
    page_size: i32,
    next_page_token: String,
    pub messages : Vec<Message>
}

#[derive(Deserialize)]
pub struct Message {
    pub id: String,
    pub message: String,
    sender: String,
    send_member_id: String,
    sender_display_name: String,
    date_time: String,
    timestamp: u64,
    message_type: String,
}
#[derive(Serialize)]
pub struct RichText {
    pub start_position: u32,
    pub end_position: u32,
    pub format_type: String,
    pub format_attr: String,
}

#[derive(Serialize)]
pub struct AtItems {
    pub at_contact: String,
    pub at_type:i32,
    pub start_position:i32,
    pub end_position: i32,
}

#[derive(Serialize)]
pub struct Payload {
    pub at_items: Vec<AtItems>,
    pub rich_text: Vec<RichText>,
    pub message: String,
    pub to_contact: String

}