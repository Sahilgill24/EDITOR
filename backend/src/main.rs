use tokio_tungstenite::connect_async;

#[tokio::main]
 async fn main() {
    let url = "ws://localhost:8080/ws";
    let (stream,_)= connect_async(url).await.expect("fail hogya bhai");
    







}
