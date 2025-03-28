use libp2p::{
    gossipsub, identity, mdns, noise, swarm::SwarmBuilder, tcp, yamux, PeerId, Swarm, Transport,
};
use serde_json;
use tokio::{sync::mpsc, task};
use crate::order::Order;

pub async fn start_p2p_network(
    local_key: identity::Keypair,
    tx: mpsc::UnboundedSender<String>,
) {
    let peer_id = PeerId::from(local_key.public());

    let gossipsub_config = gossipsub::GossipsubConfig::default();
    let gossipsub = gossipsub::Gossipsub::new(
        gossipsub::MessageAuthenticity::Signed(local_key.clone()),
        gossipsub_config,
    )
    .expect("Gossipsub 생성 실패");

    let mdns = mdns::Mdns::new(mdns::MdnsConfig::default()).expect("MDNS 생성 실패");

    let transport = tcp::TokioTcpTransport::new(tcp::GenTcpConfig::default().nodelay(true))
        .upgrade(yamux::Config::default())
        .authenticate(noise::NoiseConfig::xx(local_key.clone()).unwrap())
        .multiplex(yamux::Config::default())
        .boxed();

    let mut swarm = SwarmBuilder::new(transport, gossipsub, peer_id.clone())
        .with_mdns(mdns)
        .with_executor(Box::new(|fut| {
            task::spawn(fut);
        }))
        .build();

    println!("🚀 네트워크 시작: {:?}", peer_id);

    loop {
        tokio::select! {
            event = swarm.next() => {
                if let Some(event) = event {
                    if let libp2p::swarm::SwarmEvent::Behaviour(libp2p::gossipsub::GossipsubEvent::Message {
                        propagation_source: _,
                        message_id: _,
                        message,
                    }) = event
                    {
                        let received_data = String::from_utf8_lossy(&message.data).to_string();
                        tx.send(received_data).expect("메시지 송신 실패");
                    }
                }
            }
        }
    }
}
