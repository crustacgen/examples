use std::time;

use async_nats::{Client, Message};

use crate::publish_message;

pub fn handler_sub1(message: Message) {
    println!("Received message {:#?}", message)
}
pub fn handler_sub2(message: Message) {
    println!("Received message {:#?}", message)
}

pub async fn producer1(client: &Client, channel: &str) {
    loop {
        tokio::time::sleep(time::Duration::from_secs(2)).await;
        publish_message(client, channel, "test").await;
    }
}
pub async fn producer2(client: &Client, channel: &str) {
    loop {
        tokio::time::sleep(time::Duration::from_secs(2)).await;
        publish_message(client, channel, "test2").await;
    }
}

pub async fn test(client: &Client, channel: &str) {
    publish_message(client, channel, "from test hello").await;
}
