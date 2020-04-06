use crate::entity::player::Player;
use crate::network::packets::{self, friendlybytebuf, packet::ServerboundPacket};
use std::thread;
use std::io::Read;
use std::sync::{Arc, Mutex};

pub fn start(player: Arc<Mutex<Player>>) {
	thread::spawn(move || {
		let mut l = [0; 1];
		loop {
			match player.lock().unwrap().connection.lock().unwrap().stream.read(&mut l) {
				Ok(size) => {
					if size == 0 {
						break
					}
					if size > 0 {
						let length = l[0];
						let mut bytes = vec![0; length as usize];
						player.lock().unwrap().connection.lock().unwrap().stream.read(&mut bytes).unwrap();
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
			let mut player = player.lock().unwrap();
			if packet.view_distance < player.get_view_distance() {
				player.set_view_distance(if packet.view_distance >= 4 { packet.view_distance } else { 4 });
			}
			packet.handle(&mut player);
			(*crate::SERVER).try_lock().unwrap();
		}
		_ => {}
	}
}