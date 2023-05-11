pub mod publisher;
pub mod subscriber;

pub async fn run(){
    let handle = tokio::spawn(async {
        println!("startgin subscriber");
            subscriber::start().await.expect("could not start subscriber");
        });
        let handle_pub = tokio::spawn(async {
        println!("startgin publisher");
            publisher::start().await.expect("could not start publisher");
        });
        let _ = handle.await;
        let _ = handle_pub.await;
        println!("Fnished")
}