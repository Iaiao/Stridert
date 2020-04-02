use std::net::TcpStream;
use std::io::{Read, Write};
use std::thread;
use std::sync::{Arc, Mutex};

use crate::{network, stridert::Stridert};
use crate::network::packets::{self, friendlybytebuf, packet::{ServerboundPacket, ClientboundPacket}};
use crate::entity::player::Player;

pub struct Connection {
	pub stream   : TcpStream,
	pub state    : network::states::State,
	pub server   : Arc<Mutex<Stridert>>,
	pub protocol : i32,
	pub username : String
}

pub fn handle(stream: TcpStream, server: Arc<Mutex<Stridert>>) {
	let mut conn = Connection {
		stream,
		state: network::states::State::HANDSHAKING,
		server: server.clone(),
		protocol: -1,
		username: String::from("")
	};
	let _jh = thread::spawn(move || {
		let mut l = [0; 1];
		loop {
			match conn.stream.read(&mut l) {
				Ok(size) => {
					if size == 0 {
						break
					}
					if size > 0 {
						let length = l[0];
						let mut bytes = vec![0; length as usize];
						conn.stream.read(&mut bytes).unwrap();
						let mut buf = friendlybytebuf::FriendlyByteBuf::from(bytes);
						conn.handle_incoming(&mut buf);
						if conn.state == network::states::State::PLAY {
							let server = conn.server.clone();
							let player = Player::new(conn.username.clone(), conn);
							server.lock().unwrap().add_player(player);
							break
						}
					}
				}
				Err(_) => {
					println!("ERROR");
					break;
				}
			} {};
		}
	});
}

impl Connection {
	pub fn send(&mut self, packet: &dyn ClientboundPacket) {
		let bytes = packet.serialize();
		let _ = self.stream.write(&bytes);
	}
	pub fn disconnect(&mut self, reason: String) {
		let packet = packets::clientboundlogindisconnectpacket::ClientboundLoginDisconnectPacket::new(reason);
		self.send(&packet);
		self.stream.flush().unwrap();
		self.stream.shutdown(std::net::Shutdown::Both).unwrap();
	}
	pub fn handle_incoming(&mut self, buf: &mut friendlybytebuf::FriendlyByteBuf) {
		match self.state {
			network::states::State::HANDSHAKING => {
				match buf.read_varint() {
					0x00 => {
						let packet = packets::serverboundhandshakepacket::ServerboundHandshakePacket::deserialize(buf);
						packet.handle(self);
					}
					_ => {}
				}
			}
			network::states::State::STATUS => {
				match buf.read_varint() {
					0x00 => {
						let packet = packets::serverboundrequestpacket::ServerboundRequestPacket::deserialize(buf);
						packet.handle(self);
					}
					_ => {}
				}
			}
			network::states::State::LOGIN => {
				match buf.read_varint() {
					0x00 => {
						let packet = packets::serverboundloginstartpacket::ServerboundLoginStartPacket::deserialize(buf);
						packet.handle(self);
						
					}
					_ => {}
				}
			}
			_ => {}
		}
	}
}