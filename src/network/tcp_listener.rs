use std::net::{TcpListener, TcpStream};

fn handle_stream(stream: TcpStream) {
	crate::network::connection::handle(stream);
}

pub fn start(ip: &str, port: i16) -> std::io::Result<()> {
    let listener = TcpListener::bind(format!("{}:{}", ip, port))?;
    for stream in listener.incoming() {
		match stream {
			Ok(stream) => {
				handle_stream(stream);
			}
			Err(_) => {}
		}
	}
	
	Ok(())
}