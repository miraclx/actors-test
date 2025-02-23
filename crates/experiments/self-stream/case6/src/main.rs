use actix::Actor;
use futures_util::stream;
use tokio::time;

mod actor_stream;
mod alloc;
mod network;
mod stream_gen;

use network::NetworkManager;
use stream_gen::StreamGen;

#[global_allocator]
static GLOBAL: alloc::MyAllocator = alloc::MyAllocator;

#[actix::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let network_addr = NetworkManager {
        repeat: Box::new(StreamGen::new(stream::repeat(0), 4)),
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
