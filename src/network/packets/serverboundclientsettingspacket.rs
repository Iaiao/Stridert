use crate::network::packets::{self, packet::{Packet, ServerboundPacket}, friendlybytebuf::FriendlyByteBuf};
use crate::registry::{chatmodes::ChatMode, hands::Hand};
use crate::entity::player::Player;
use crate::SERVER;
use crate::registry::entitystatuses;
use std::sync::{Arc, Mutex};

pub struct ServerboundClientSettingsPacket {
	locale: String,
pub view_distance: u8,
	chat_mode: ChatMode,
	chat_colors: bool,
	skin_parts: u8,
	main_hand: Hand
}

impl Packet for ServerboundClientSettingsPacket {
	const ID: i32 = 0x05;
}

impl ServerboundPacket for ServerboundClientSettingsPacket {
	fn deserialize(buf: &mut FriendlyByteBuf) -> ServerboundClientSettingsPacket {
		let locale = buf.read_string();
		let view_distance = buf.read_byte();
		let chat_mode = match buf.read_varint() {
			1 => ChatMode::CommandsOnly,
			2 => ChatMode::Hidden,
			_ => ChatMode::Enabled
		};
		let chat_colors = buf.read_boolean();
		let skin_parts = buf.read_byte();
		let _cape = skin_parts & 0b00000001 != 0;
		let _jacket = skin_parts & 0b00000010 != 0;
		let _left_sleeve = skin_parts & 0b00000100 != 0;
		let _right_sleeve = skin_parts & 0b00001000 != 0;
		let _left_pants_leg = skin_parts & 0b00010000 != 0;
		let _right_pants_leg = skin_parts & 0b00100000 != 0;
		let _hat = skin_parts & 0b01000000 != 0;
		let main_hand = match buf.read_varint() {
			0 => Hand::Left,
			_ => Hand::Right
		};
		return ServerboundClientSettingsPacket {
			locale,
			view_distance,
			chat_mode,
			chat_colors,
			skin_parts,
			main_hand
		}
	}
}

impl ServerboundClientSettingsPacket {
	pub fn handle(&self, player: Arc<Mutex<Player>>) {
		{
			let p = player.lock().unwrap();
			let mut connection = p.connection.lock().unwrap();
			
			connection.send(&packets::clientboundhelditemchangepacket::ClientboundHeldItemChangePacket::new(0));
			connection.send(&packets::clientbounddeclarerecipespacket::ClientboundDeclareRecipesPacket::new((*SERVER).lock().unwrap().get_recipes()));
			connection.send(&packets::clientboundtagspacket::ClientboundTagsPacket::new());
			let entity_id = p.get_entity().get_id();
			connection.send(&packets::clientboundentitystatuspacket::ClientboundEntityStatusPacket::new(entity_id, entitystatuses::player::OP_PERMISSION_LEVEL_4));
			connection.send(&packets::clientbounddeclarecommandspacket::ClientboundDeclareCommandsPacket::new());
			let recipes = (*SERVER).lock().unwrap().get_recipes();
			connection.send(&packets::clientboundunlockrecipespacket::ClientboundUnlockRecipesPacket::new(
				packets::clientboundunlockrecipespacket::Action::Init,
				true,
				true,
				true,
				true,
				recipes.clone(),
				Option::from(recipes)
			));
			connection.send(&packets::clientboundplayerpositionandlookpacket::ClientboundPlayerPositionPacket::new(
				p.get_x(),
				p.get_y(),
				p.get_z(),
				p.get_yaw(),
				p.get_pitch(),
				false,
				false,
				false,
				false,
				false
			));
			let mut players = (*crate::SERVER).lock().unwrap().get_players();
			players.retain(|p| {
				match p.try_lock() {
					Ok(_) => true,
					_ => false
				}
			});

			connection.send(&packets::clientboundplayerinfopacket::ClientboundPlayerInfoPacket::new(
				packets::clientboundplayerinfopacket::Action::AddPlayer,
				players
			));
		}
		(*crate::SERVER).lock().unwrap().broadcast_packet(&packets::clientboundplayerinfopacket::ClientboundPlayerInfoPacket::new(
			packets::clientboundplayerinfopacket::Action::AddPlayer,
			vec!(player.clone())
		));
		(*crate::SERVER).lock().unwrap().broadcast_packet(&packets::clientboundplayerinfopacket::ClientboundPlayerInfoPacket::new(
			packets::clientboundplayerinfopacket::Action::UpdateLatency,
			vec!(player.clone())
		));
		{
			let p = player.lock().unwrap();
			let mut connection = p.connection.lock().unwrap();
			
			connection.send(&packets::clientboundupdateviewpositionpacket::ClientboundUpdateViewPositionPacket::new(p.get_x(), p.get_z()));
		}
	}
}