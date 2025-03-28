mod network;
mod order;

use futures::prelude::*;
use libp2p::{identity, PeerId};
use network::start_p2p_network;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::unbounded_channel();
    
    // 랜덤 키 생성 (노드 ID)
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());

    println!("🔥 P2P 노드 시작: {:?}", local_peer_id);

    // 네트워크 시작
    let _network_handle = tokio::spawn(start_p2p_network(local_key, tx));

    while let Some(msg) = rx.recv().await {
        println!("📩 수신된 메시지: {}", msg);
    }
}
