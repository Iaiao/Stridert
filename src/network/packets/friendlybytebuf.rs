use crate::util;
use crate::registry::identifier::Identifier;

pub struct FriendlyByteBuf {
	bytes: Vec<u8>,
pub pointer: usize
}

impl FriendlyByteBuf {
	pub fn new() -> FriendlyByteBuf {
		return FriendlyByteBuf {
			bytes: Vec::new(),
			pointer: 0
		}
	}
	pub fn from(bytes: Vec<u8>) -> FriendlyByteBuf {
		return FriendlyByteBuf {
			bytes,
			pointer: 0
		}
	}
	pub fn write_byte(&mut self, value: u8) {
		self.bytes.push(value);
	}
	pub fn write_varint(&mut self, value: i32) {
		self.bytes.append(&mut util::var::to_varint(value));
	}
	pub fn write_int(&mut self, value: i32) {
		self.bytes.append(&mut value.to_be_bytes().to_vec());
	}
	pub fn write_long(&mut self, value: i64) {
		self.bytes.append(&mut value.to_be_bytes().to_vec())
	}
	pub fn write_float(&mut self, value: f32) {
		self.bytes.append(&mut value.to_be_bytes().to_vec());
	}
	pub fn write_bytes(&mut self, value: &mut Vec<u8>) {
		self.bytes.append(value)
	}
	pub fn write_boolean(&mut self, value: bool) {
		self.write_byte(if value { 1 } else { 0 })
	}
	pub fn write_string(&mut self, value: &String) {
		self.write_varint(value.len() as i32);
		self.bytes.append(&mut value.as_bytes().to_vec());
	}
	pub fn write_identifier(&mut self, value: Identifier) {
		self.write_string(&value.to_string());
	}
	pub fn read_byte(&mut self) -> u8 {
		self.pointer += 1;
		return self.bytes[self.pointer - 1];
	}
	pub fn read_varint(&mut self) -> i32 {
		let (size, varint) = util::var::from_varint(self.bytes[self.pointer..].to_vec());
		self.pointer += size;
		return varint;
	}
	pub fn read_ushort(&mut self) -> u16 {
		self.pointer += 2;
		return ((self.bytes[self.pointer - 2] as u16) << 8) | self.bytes[self.pointer - 1] as u16;
	}
	pub fn read_string(&mut self) -> String {
		let length = self.read_varint() as usize;
		let value = String::from_utf8(self.bytes[self.pointer..self.pointer + length].to_vec());
		self.pointer += length;
		return match value {
			Ok(value) => value,
			_ => String::from("")
		}
	}
	pub fn read_boolean(&mut self) -> bool {
		return self.read_byte() == 0x01;
	}
	pub fn read_bytes(&mut self, count: usize) -> Vec<u8> {
		self.pointer += count;
		return self.bytes[self.pointer - count .. self.pointer].to_vec()
	}
	pub fn read_identifier(&mut self) -> Identifier {
		return Identifier::from_string(self.read_string())
	}
	pub fn to_bytes(&self) -> Vec<u8> {
		let length = self.bytes.len();
		let mut bytes = util::var::to_varint(length as i32);
		bytes.append(&mut self.bytes.clone());
		return bytes;
	}
	pub fn len(&self) -> usize { self.bytes.len() }
}