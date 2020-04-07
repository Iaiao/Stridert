use rand::{thread_rng, RngCore};

#[derive(Clone)]
pub struct UUID {
    most_sig_bits  : u64,
	least_sig_bits : u64
}

pub fn encode(uuid: &UUID) -> [u32; 4] {
    return [
        (uuid.get_most_significant_bits() >> 32) as u32,
         uuid.get_most_significant_bits() as u32,
        (uuid.get_least_significant_bits() >> 32) as u32,
         uuid.get_least_significant_bits() as u32
    ]
}

impl UUID {

    pub fn decode(encoded: [u32; 4]) -> UUID {
        return UUID::new((encoded[0] as u64) << 32 | (encoded[1] as u64) & 0xFFFF_FFFF, (encoded[2] as u64) << 32 | (encoded[3] as u64) & 0xFFFF_FFFF);
    }

    pub fn from_bytes(bytes: [u8; 16]) -> UUID {
        let mut most: u64 = 0;
        let mut least: u64 = 0;

        for i in 0..7 {
            most = most << 8 | (bytes[i] & 255) as u64;
        }

        for i in 8..15 {
            least = least << 8 | (bytes[i] & 255) as u64;
        }

		return UUID {
        	most_sig_bits  : most,
			least_sig_bits : least
		}
    }

    pub fn new(most: u64, least: u64) -> UUID {
		return UUID {
        	most_sig_bits  : most,
			least_sig_bits : least
		}
    }

    pub fn random() -> UUID {
        let mut rng = thread_rng();
        let mut bytes = [0; 16];
        rng.fill_bytes(&mut bytes);
        return UUID::from_bytes(bytes);
    }

    pub fn from_string(string: &str) -> UUID {
        let splitted: Vec<String> = string.split("-").into_iter().map(|x| x.to_string()).collect();
        if splitted.len() != 5 {
            panic!(format!("Invalid UUID string: {}", string));
        } else {
            let mut most = u64::from_str_radix(&splitted[0], 16).unwrap();
            most <<= 16;
            most |= u64::from_str_radix(&splitted[1], 16).unwrap();
            most <<= 16;
            most |= u64::from_str_radix(&splitted[2], 16).unwrap();
            let mut least = u64::from_str_radix(&splitted[3], 16).unwrap();
            least <<= 48;
            least |= u64::from_str_radix(&splitted[4], 16).unwrap();
            return UUID::new(most, least);
        }
    }

    pub fn get_least_significant_bits(&self) -> u64 {
        return self.least_sig_bits;
    }

    pub fn get_most_significant_bits(&self) -> u64 {
        return self.most_sig_bits;
    }

    pub fn to_string(&self) -> String {
        return format!("{}-{}-{}-{}-{}", digits(self.most_sig_bits >> 32, 8), digits(self.most_sig_bits >> 16, 4), digits(self.most_sig_bits, 4), digits(self.least_sig_bits >> 48, 4), digits(self.least_sig_bits, 12))
    }

}

fn digits(num: u64, len: i32) -> String {
	let n = (1 as u64) << len * 4;
	return format!("{:x?}", n | num & n - 1 as u64)[1..].to_string();
}