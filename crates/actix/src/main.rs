#![allow(dead_code)]

use actix::Actor;
use network::NetworkManager;

mod blob_mgr;
mod context_mgr;
mod data_mgr;
mod network;
mod runtime;

#[actix::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let network_addr = NetworkManager::start_default();

    network_addr
        .send(network::ListenOn {
            address: "".to_owned(),
        })
        .await?;

    println!("All done here!");

    Ok(())
}
