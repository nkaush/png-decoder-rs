use super::chunk_type::ChunkType;
use std::string::FromUtf8Error;
use crc::Crc;

/// A validated PNG chunk. See the PNG Spec for more details
/// http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
pub trait Chunk {
    /// Constructs a new `Chunk` from a given `ChunckType` and the associated 
    /// byte data given as a `Vec<u8>`.
    fn new(chunk_type: ChunkType, data: Vec<u8>, crc: u32) 
        -> Result<Self, String> where Self: std::marker::Sized;

    /// The length of the data portion of this chunk.
    fn length(&self) -> u32;

    /// The `ChunkType` of this chunk.
    fn chunk_type(&self) -> &ChunkType;
    
    /// The raw data contained in this chunk in bytes.
    fn data(&self) -> &[u8];
    
    /// The CRC of this chunk.
    fn crc(&self) -> u32;

    /// Returns the data stored in this chunk as a `String`. This function will return an error
    /// if the stored data is not valid UTF-8.
    fn data_as_string(&self) -> Result<String, FromUtf8Error>;
    
    /// Returns this chunk as a byte sequences described by the PNG spec.
    /// The following data is included in this byte sequence in order:
    /// 1. Length of the data *(4 bytes)*
    /// 2. Chunk type *(4 bytes)*
    /// 3. The data itself *(`length` bytes)*
    /// 4. The CRC of the chunk type and data *(4 bytes)*
    fn as_bytes(&self) -> Vec<u8> {
        self.length()
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type().bytes().iter())
            .chain(self.data().iter())
            .chain(self.crc().to_be_bytes().iter())
            .copied()
            .collect()
    }
}

pub fn verify_crc(chunk_type: &ChunkType, data: &Vec<u8>, crc: u32) -> Result<(), String> {
    let crc_bytes: Vec<u8> = chunk_type
        .bytes()
        .iter()
        .chain(data.iter())
        .copied()
        .collect();

    let computed_crc = Crc::<u32>::new(&crc::CRC_32_ISO_HDLC).checksum(&crc_bytes);

    if crc != computed_crc {
        return Err("Computed CRC does not match given CRC.".into());
    }

    Ok(())
}