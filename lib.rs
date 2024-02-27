// lib.rs

pub struct ProtectedInt {
    encoded: u8,
}

impl ProtectedInt {
    pub fn new(value: u8) -> Self {
        assert!(value < 16, "Value must be less than 16 to fit in 4 bits.");
        let encoded = Self::encode(value);
        ProtectedInt { encoded }
    }

    pub fn get(&self) -> u8 {
        Self::decode(self.encoded)
    }

    fn encode(value: u8) -> u8 {
        let p1 = (value >> 2) ^ (value >> 1) ^ (value >> 0) & 1;
        let p2 = (value >> 2) ^ (value >> 1) ^ (value >> 3) & 1;
        let p3 = (value >> 1) ^ (value >> 0) ^ (value >> 3) & 1;
        (p1 << 6) | (p2 << 5) | (value << 1) | p3
    }

    fn decode(encoded: u8) -> u8 {
        let p1 = (encoded >> 6) & 1;
        let p2 = (encoded >> 5) & 1;
        let p3 = encoded & 1;
        let data = (encoded >> 1) & 0b1111;

        let c1 = p1 ^ ((data >> 2) & 1) ^ ((data >> 1) & 1) ^ (data & 1);
        let c2 = p2 ^ ((data >> 2) & 1) ^ ((data >> 3) & 1) ^ ((data >> 1) & 1);
        let c3 = p3 ^ ((data >> 3) & 1) ^ ((data >> 1) & 1) ^ (data & 1);

        let error_pos = (c3 << 2) | (c2 << 1) | c1;
        if error_pos != 0 {
            println!("Error detected and corrected at position: {}", error_pos);
        }

        data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protected_int() {
        let original_value = 13; // 0b1101
        let protected_int = ProtectedInt::new(original_value);

        // Simulate a single-bit error in the encoded data
        let corrupted_encoded = protected_int.encoded ^ 0b00000100; // Flip one bit

        // Decode with error correction
        let decoded_value = ProtectedInt::decode(corrupted_encoded);
        assert_eq!(decoded_value, original_value, "The decoded value should match the original value after error correction.");
    }
}
