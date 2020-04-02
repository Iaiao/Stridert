use std::net::{TcpListener, TcpStream};
use crate::stridert::Stridert;
use std::sync::{Arc, Mutex};

fn handle_stream(stream: TcpStream, server: Arc<Mutex<Stridert>>) {
	crate::network::connection::handle(stream, server);
}

pub fn start(ip: &str, port: i16, server: Arc<Mutex<Stridert>>) -> std::io::Result<()> {
    let listener = TcpListener::bind(format!("{}:{}", ip, port))?;
    for stream in listener.incoming() {
		match stream {
			Ok(stream) => {
				handle_stream(stream, server.clone());
			}
			Err(_) => {}
		}
	}
	
	Ok(())
}