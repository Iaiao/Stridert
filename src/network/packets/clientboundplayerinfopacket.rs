use crate::network::packets::{friendlybytebuf::FriendlyByteBuf, packet::{ClientboundPacket, Packet}};
use crate::entity::player::Player;
use crate::util::uuid;
use crate::registry::gamemodes::GameMode;
use crate::entity::property::Property;
use std::sync::{Arc, Mutex};

pub struct ClientboundPlayerInfoPacket {
	action: Action,
	players: Vec<PlayerData>
}

struct PlayerData {
	name: String,
	uuid: uuid::UUID,
	gamemode: GameMode,
	properties: Vec<Property>
}

impl Packet for ClientboundPlayerInfoPacket {
	const ID: i32 = 0x34;
}

impl ClientboundPacket for ClientboundPlayerInfoPacket {
	fn serialize(&self) -> Vec<u8> {
		let mut buf = FriendlyByteBuf::new();
		buf.write_varint(ClientboundPlayerInfoPacket::ID);
		buf.write_varint(match self.action {
			Action::AddPlayer => 0,
			Action::UpdateGamemode => 1,
			Action::UpdateLatency => 2,
			Action::UpdateDisplayName => 3,
			Action::RemovePlayer => 4
		});

		buf.write_varint(self.players.len() as i32);
		for player in &self.players {
			for i in &uuid::encode(&player.uuid) {
				buf.write_uint(*i);
			}
			match self.action {
				Action::AddPlayer => {
					buf.write_string(&player.name);
					buf.write_varint(player.properties.len() as i32);
					for prop in &player.properties {
						buf.write_string(&prop.name);
						buf.write_string(&prop.value);
						match &prop.signature {
							Some(signature) => {
								buf.write_boolean(true);
								buf.write_string(signature);
							}
							None => {
								buf.write_boolean(false);
							}
						}
					}
					buf.write_varint(match player.gamemode {
						GameMode::SURVIVAL => 0,
						GameMode::CREATIVE => 1,
						GameMode::ADVENTURE => 2,
						GameMode::SPECTATOR => 3
					});
					buf.write_varint(10); // Пинг
					buf.write_boolean(false); // displayName
				},
				Action::UpdateGamemode => {
					buf.write_varint(match player.gamemode {
						GameMode::SURVIVAL => 0,
						GameMode::CREATIVE => 1,
						GameMode::ADVENTURE => 2,
						GameMode::SPECTATOR => 3
					});
				},
				Action::UpdateLatency => {
					buf.write_varint(10);
				},
				Action::UpdateDisplayName => {
					buf.write_boolean(false);
				},
				Action::RemovePlayer => {}
			}
		}

		return buf.to_bytes();
	}
}

impl ClientboundPlayerInfoPacket {
	pub fn new(action: Action, players: Vec<Arc<Mutex<Player>>>) -> ClientboundPlayerInfoPacket {
		return ClientboundPlayerInfoPacket {
			action,
			players: players.into_iter().map(|p| {
				let player = p.lock().unwrap();
				return PlayerData {
					name: player.get_name(),
					uuid: player.get_uuid().clone(),
					gamemode: player.get_gamemode(),
					properties: player.get_properties().clone()
				}
			}).collect()
		}
	}
	pub fn add_player(&mut self, player: &Player) {
		self.players.push(PlayerData {
			name: player.get_name(),
			uuid: player.get_uuid().clone(),
			gamemode: player.get_gamemode(),
			properties: player.get_properties().clone()
		})
	}
}

pub enum Action {
	AddPlayer,
	UpdateGamemode,
	UpdateLatency,
	UpdateDisplayName,
	RemovePlayer
}