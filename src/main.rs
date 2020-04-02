mod network;
mod config;
mod stridert;
mod util;
mod world;
mod entity;
mod registry;
mod inventory;

use std::sync::{Arc, Mutex};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref SERVER: Arc<Mutex<stridert::Stridert>> = Arc::new(Mutex::new(stridert::Stridert::new()));
}

fn main() {
    network::tcp_listener::start("0.0.0.0", 25565, (*SERVER).clone()).expect("Не получилось запустить слушатель TCP");
    println!("Started")
}
