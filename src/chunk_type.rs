use std::convert::TryFrom;
use std::str::FromStr;
use std::{fmt, str};

/// A validated PNG chunk type. See the PNG spec for more details.
/// http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
/// 
/// Little Endian Implementation: 0th (first) byte and/or bit at index 0.
#[derive(Eq, PartialEq, Debug)]
pub struct ChunkType {
    type_code: [u8; 4]
}

impl ChunkType {
    /// Returns the raw bytes contained in this chunk.
    pub fn bytes(&self) -> [u8; 4] {
        self.type_code
    }

    /// Returns true if the reserved byte is valid and all four bytes are 
    /// represented by the characters A-Z or a-z. Note that this chunk type 
    /// should always be valid as it is validated during construction.
    pub fn is_valid(&self) -> bool {
        for b in &self.type_code {
            if Self::is_valid_byte(b) {
                return false;
            }
        }

        self.is_reserved_bit_valid()
    }

    /// Returns the property state of the first byte as described in the PNG spec.
    pub fn is_critical(&self) -> bool {
        self.type_code[0].is_ascii_uppercase()
    }

    /// Returns the property state of the second byte as described in the PNG spec.
    pub fn is_public(&self) -> bool {
        self.type_code[1].is_ascii_uppercase()
    }

    /// Returns the property state of the third byte as described in the PNG spec.
    pub fn is_reserved_bit_valid(&self) -> bool {
        self.type_code[2].is_ascii_uppercase()
    }

    /// Returns the property state of the fourth byte as described in the PNG spec.
    pub fn is_safe_to_copy(&self) -> bool {
        self.type_code[3].is_ascii_lowercase()
    }

    /// Valid bytes are represented by the characters A-Z or a-z.
    pub fn is_valid_byte(byte: &u8) -> bool {
        byte.is_ascii_lowercase() || byte.is_ascii_uppercase()
    }
}

impl FromStr for ChunkType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bytes: [u8; 4] = Default::default();
        let input: &[u8] = s.as_bytes();

        if input.len() < bytes.len() {
            return Err("Invalid ChunkType. String must contain 4 characters to satisfy the requirement of a 4-byte chunk type code.".into())
        }

        for idx in 0..bytes.len() {
            bytes[idx] = input[idx];

            if !ChunkType::is_valid_byte(&bytes[idx]) {
                return Err("Type codes are restricted to consist of uppercase and lowercase ASCII letters (A-Z and a-z, or 65-90 and 97-122 decimal).".into());
            }
        }

        Ok(ChunkType{type_code: bytes})
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = String;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        for byte in value.iter() {
            if !ChunkType::is_valid_byte(byte) {
                return Err("Type codes are restricted to consist of uppercase and lowercase ASCII letters (A-Z and a-z, or 65-90 and 97-122 decimal).".into());
            }
        }

        Ok(ChunkType{type_code: value})
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let displayed: &str = str::from_utf8(&self.type_code).unwrap();
        write!(f, "{}", displayed.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
