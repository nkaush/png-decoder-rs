use super::{chunk::Chunk, chunk_type::ChunkType, chunk};
use std::convert::{TryFrom, TryInto};
use std::string::FromUtf8Error;
use std::fmt;

pub struct IHDR {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32
}

impl IHDR {
    const CHUNK_TYPE: &'static str = "IHDR";

    fn width(&self) -> u32 {
        u32::from_be_bytes(self.data[..4].try_into().unwrap())
    }

    fn height(&self) -> u32 {
        u32::from_be_bytes(self.data[4..8].try_into().unwrap())
    }

    fn bit_depth(&self) -> u8 {
        self.data[8]
    }

    fn color_type(&self) -> u8 {
        self.data[9]
    } 

    fn compression_method(&self) -> u8 {
        self.data[10]
    }
    
    fn filter_method(&self) -> u8 {
        self.data[11]
    }

    fn interlace_method(&self) -> u8 {
        self.data[12]
    }
}

impl Chunk for IHDR {
    fn new(chunk_type: ChunkType, data: Vec<u8>, crc: u32) -> Result<Self, String> {
        chunk::verify_crc(&chunk_type, &data, crc)?;

        if data.len() != 13 {
            return Err(format!("Chunk length {} does not match expected length 13.", data.len()));
        }

        if format!("{}", chunk_type) != Self::CHUNK_TYPE.to_string() {
            return Err("Chunk type does not match expected chunk type 'IHDR'".into());
        }

        let ihdr = Self {
            length: data.len() as u32,
            chunk_type,
            data,
            crc
        };

        let bit_depth = ihdr.bit_depth();

        let is_valid_combination = match ihdr.color_type() {
            0 => matches!(bit_depth, 1 | 2 | 4 | 8 | 16),
            2 => matches!(bit_depth, 8 | 16),
            3 => matches!(bit_depth, 1 | 2 | 4 | 8),
            4 => matches!(bit_depth, 8 | 16),
            6 => matches!(bit_depth, 8 | 16),
            _ => false
        };

        if !is_valid_combination {
            return Err("Invalid color type and bit depth combination.".into());
        }

        Ok(ihdr)
    }

    fn length(&self) -> u32 {
        self.length
    }

    fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }
    
    fn data(&self) -> &[u8] {
        &self.data
    }
    
    fn crc(&self) -> u32 {
        self.crc
    }
    
    fn data_as_string(&self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.data.clone())
    }
}

impl TryFrom<&[u8]> for IHDR {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let chunk_type = ChunkType::new(value[..4].try_into().unwrap())?;

        let ihdr = Self::new(
            chunk_type, 
            value[4..value.len() - 4].to_vec(),
            u32::from_be_bytes(value[value.len() - 4..].try_into().unwrap())
        )?;

        Ok(ihdr)
    }
}

impl fmt::Display for IHDR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "IHDR Image Header {{",)?;
        writeln!(f, "  Width: {}", self.width())?;
        writeln!(f, "  Height: {}", self.height())?;
        writeln!(f, "  Bit Depth: {}", self.bit_depth())?;
        writeln!(f, "  Color Type: {}", self.color_type())?;
        writeln!(f, "  Compression Method: {}", self.compression_method())?;
        writeln!(f, "  Filter Method: {}", self.filter_method())?;
        writeln!(f, "  Interlace Method: {}", self.interlace_method())?;
        writeln!(f, "}}",)?;
        
        Ok(())
    }
}
