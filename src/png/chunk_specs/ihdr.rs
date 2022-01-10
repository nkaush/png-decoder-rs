use crate::png::{Chunk, ChunkType};
use std::convert::{TryFrom, TryInto};
use std::str::FromStr;
use std::fmt;

pub struct IHDR {
    width: u32,
    height: u32,
    bit_depth: u8,
    color_type: u8,
    compression_method: u8,
    filter_method: u8,
    interlace_method: u8
}

impl IHDR {
    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn bit_depth(&self) -> u8 {
        self.bit_depth
    }

    fn color_type(&self) -> u8 {
        self.color_type
    } 

    fn compression_method(&self) -> u8 {
        self.compression_method
    }
    
    fn filter_method(&self) -> u8 {
        self.filter_method
    }

    fn interlace_method(&self) -> u8 {
        self.interlace_method
    }
}

impl TryFrom<Chunk> for IHDR {
    type Error = String;

    fn try_from(value: Chunk) -> Result<Self, Self::Error> {
        let expected_type: ChunkType = ChunkType::from_str("IHDR")?;

        if &expected_type != value.chunk_type() {
            return Err("Chunk type does not match expected chunk type 'IHDR'".into());
        }

        if value.length() != 13 {
            return Err(format!("Chunk length {} does not match expected length 13.", value.length()));
        }

        let ihdr = IHDR {
            width: u32::from_be_bytes(value.data()[0..4].try_into().unwrap()),
            height: u32::from_be_bytes(value.data()[4..8].try_into().unwrap()),
            bit_depth: value.data()[8],
            color_type: value.data()[9],
            compression_method: value.data()[10],
            filter_method: value.data()[11],
            interlace_method: value.data()[12]
        };

        let valid_combination = match ihdr.color_type {
            0 => matches!(ihdr.bit_depth, 1 | 2 | 4 | 8 | 16),
            2 => matches!(ihdr.bit_depth, 8 | 16),
            3 => matches!(ihdr.bit_depth, 1 | 2 | 4 | 8),
            4 => matches!(ihdr.bit_depth, 8 | 16),
            6 => matches!(ihdr.bit_depth, 8 | 16),
            _ => false
        };

        if !valid_combination {
            return Err("Invalid color type and bit depth combination.".into());
        }

        Ok(ihdr)
    }
}

impl fmt::Display for IHDR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "IHDR Image header {{",)?;
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
