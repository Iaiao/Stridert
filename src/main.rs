mod network;
mod config;
mod stridert;
mod util;
mod world;
mod entity;
mod registry;

use std::sync::{Arc, Mutex};

fn main() {
    let server = stridert::Stridert::new();
    network::tcp_listener::start("0.0.0.0", 25565, Arc::new(Mutex::new(server))).expect("Не получилось запустить слушатель TCP");
    println!("Started")
}
