use std::sync::Arc;

use actix::Actor;
use tokio::{sync::Mutex, time};

mod alloc;
mod network;

use network::{NetworkManager, Swarm};

#[global_allocator]
static GLOBAL: alloc::MyAllocator = alloc::MyAllocator;

#[actix::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let network_addr = NetworkManager {
        swarm: Arc::new(Mutex::new(Swarm::new(0, 30))),
    }
    .start();

    let mut interval = time::interval(time::Duration::from_secs(2));

    interval.tick().await;

    println!("a");

    interval.tick().await;

    network_addr.send(network::ToSwarm { value: 10 }).await?;

    println!("b");

    interval.tick().await;

    network_addr.send(network::ToSwarm { value: 20 }).await?;

    println!("c");

    interval.tick().await;

    drop(network_addr);
    println!("dropped the networking actor, the swarm should shutdown");

    interval.tick().await;

    println!("All done here!");

    Ok(())
}
