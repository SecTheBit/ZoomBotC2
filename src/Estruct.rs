use serde::Deserialize;

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
#[derive(Deserialize)]
pub struct RichText {
    start_position: u32,
    end_position: u32,
    format_type: String,
    format_attr: String,
}