use bytes::Bytes;
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    let client = async_nats::connect("demo.nats.io").await?;
    let mut subscriber = client.subscribe("user/signedup".into()).await?.take(10);

    for _ in 0..10 {
        client.publish("user/signedup".into(), "data".into()).await?;
    }

    while let Some(message) = subscriber.next().await {
      println!("Received message {:?}", message);
    }

    Ok(())
}
