use libp2p::futures::StreamExt;
use libp2p::mdns::{Mdns, MdnsConfig, MdnsEvent};
use libp2p::swarm::{Swarm, SwarmEvent};
use libp2p::{identity, PeerId};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let new_key = identity::Keypair::generate_ed25519();
    let new_peer_id = PeerId::from(new_key.public());

    println!("Local Peer ID is: {:?}", new_peer_id);

    let transport = libp2p::development_transport(new_key).await?;
    let behaviour = Mdns::new(MdnsConfig::default()).await?;
    let mut swarm = Swarm::new(transport, behaviour, new_peer_id);

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Listening on Local Address {:?}", address)
            }
            SwarmEvent::Behaviour(MdnsEvent::Discovered(peers)) => {
                for (peer, addr) in peers {
                    println!("Discovered {} {}", peer, addr);
                }
            }
            SwarmEvent::Behaviour(MdnsEvent::Expired(expired)) => {
                for (peer, addr) in expired {
                    println!("Expired {} {}", peer, addr);
                }
            }
            _ => {}
        }
    }
}
