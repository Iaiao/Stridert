pub fn from_varint(bytes: Vec<u8>) -> (usize, i32) {
	let mut num_read = 0;
	let mut result: i32 = 0;
	let mut read;

	loop {
		read = bytes[num_read];
		let value = read & 0b01111111;
		result |= (value as i32) << (7 * num_read);
		num_read += 1;
		if (read & 0b10000000) == 0 {
			break
		}
	};

	return (num_read, result);
}

pub fn to_varint(value: i32) -> Vec<u8> {
	let mut bytes = vec!();
	let mut val = value as u32;
	loop {
		let mut b = (val & 0b01111111) as u8;
		val >>= 7;
		if val != 0 {
			b |= 0b10000000;
		}
		bytes.push(b);
		if val == 0 {
			break
		}
	};
	return bytes;
}