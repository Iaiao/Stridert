use crate::entity::player::Player;
use crate::network::packets::{self, friendlybytebuf, packet::ServerboundPacket};
use std::thread;
use std::io::Read;
use std::sync::{Arc, Mutex};

pub fn start(player: Arc<Mutex<Player>>) {
	thread::spawn(move || {
		let mut l = [0; 1];
		loop {
			let res;
			{
				res = player.lock().unwrap().connection.lock().unwrap().stream.read(&mut l);
			}
			match res {
				Ok(size) => {
					if size == 0 {
						break
					}
					if size > 0 {
						let length = l[0];
						let mut bytes = vec![0; length as usize];
						{
							let player = player.lock().unwrap();
							let mut connection = player.connection.lock().unwrap();
							connection.stream.read(&mut bytes).unwrap();
						}
						let mut buf = friendlybytebuf::FriendlyByteBuf::from(bytes);
						let pid = buf.read_varint();
						handle(player.clone(), &mut buf, pid);
					}
				}
				Err(_) => {
					println!("ERROR");
					break;
				}
			};
		}
	});
}

fn handle(player: Arc<Mutex<Player>>, buf: &mut friendlybytebuf::FriendlyByteBuf, pid: i32) {
	dbg!(pid);
	match pid {
		0x0B => {
			let packet = packets::serverboundpluginmessagepacket::ServerboundPluginMessagePacket::deserialize(buf);
			packet.handle();
		}
		0x05 => {
			let packet = packets::serverboundclientsettingspacket::ServerboundClientSettingsPacket::deserialize(buf);
			{
				let mut p = player.lock().unwrap();
				if packet.view_distance < p.get_view_distance() {
					p.set_view_distance(if packet.view_distance >= 4 { packet.view_distance } else { 4 });
				}
			}
			packet.handle(player.clone());
		}
		_ => {}
	}
}