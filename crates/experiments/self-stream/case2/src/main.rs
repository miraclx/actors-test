use actix::Actor;
use network::{NetworkManager, Swarm};
use tokio::time;

mod alloc;
mod network;

#[global_allocator]
static GLOBAL: alloc::MyAllocator = alloc::MyAllocator;

#[actix::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let network_addr = NetworkManager {
        swarm: Box::new(Swarm::new(0, 4)),
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

    // pending::<()>().await;

    println!("All done here!");

    Ok(())
}
