use std::error::Error;

pub async fn start()->Result<(), Box<dyn Error>>{
    let nats_url = "nats://localhost:4222";
    let client = async_nats::connect(nats_url).await?;
    let subject = "foo";
    let payload = "Hello World!";
    client.publish(subject.to_string(), payload.into()).await?;
    // wait for 3 seconds 
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    client.publish("foo".into(), "one more time".into()).await?;
    Ok(())
}