mod network;
use network::{send_order, Order, create_client_endpoint};
use std::error::Error;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let orders = vec![
        Order { order_id: "12345".to_string(), order_type: "buy".to_string(), amount: 1.5, price: 3000.0 },
        Order { order_id: "67890".to_string(), order_type: "sell".to_string(), amount: 2.0, price: 3100.0 },
        Order { order_id: "24680".to_string(), order_type: "buy".to_string(), amount: 3.5, price: 3200.0 }
    ];

    println!("🚀 클라이언트: 주문 전송 준비 중...");

    // let endpoint = create_client_endpoint("127.0.0.1:5133").await?;

    // ✅ 여러 개의 주문을 동시에 전송
    let mut handles = vec![];
    for order in orders {
        let order_clone = order.clone();
        let handle = tokio::spawn(async move {
            println!("🚀 주문 전송 중: {:?}", order_clone);
            if let Err(e) = send_order("127.0.0.1:5000", &order_clone).await {
                eprintln!("❌ 주문 전송 실패: {:?}", e);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await?;
    }

    println!("✅ 클라이언트: 모든 주문 전송 완료!");

    // drop(endpoint); // ✅ 종료 시 소켓 해제
    Ok(())
}
