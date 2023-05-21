mod handler;
use async_nats::{Client, Message, Subscriber};

use futures::StreamExt;

use crate::handler::*;

pub struct Producer {}

async fn listen_for_message(sub: &mut Subscriber, handler: impl Fn(Message)) {
    while let Some(message) = sub.next().await {
        handler(message);
    }
}
async fn publish_message(client: &Client, channel: &str, payload: &'static str) {
    client
        .publish(channel.into(), payload.into())
        .await
        .unwrap();
    println!("sent");
}

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    let client = async_nats::connect("demo.nats.io").await?;

    let mut sub1 = client.subscribe("foo".into()).await?;
    let mut sub2 = client.subscribe("bar".into()).await?;

    test(&client, "foo").await;

    tokio::join!(
        producer1(&client, "foo"),
        producer2(&client, "bar"),
        listen_for_message(&mut sub1, handler_sub1),
        listen_for_message(&mut sub2, handler_sub2),
    );

    println!("fin");
    Ok(())
}
