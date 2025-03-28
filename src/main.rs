mod network; // ✅ `network.rs`를 가져옴
use network::start_server;
use tokio::sync::mpsc;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (tx, mut rx) = mpsc::unbounded_channel();

    tokio::spawn(async move {
        while let Some(order) = rx.recv().await {
            println!("📩 수신된 주문 데이터: {}", order);
        }
    });

    start_server("127.0.0.1:5000", tx).await?;
    Ok(())
}
