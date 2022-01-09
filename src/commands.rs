use std::str::FromStr;
use std::fs;

use crate::png::{Chunk, ChunkType, Png};
use crate::args::{
    EncodeArguments, 
    DecodeArguments, 
    RemoveArguments, 
    PrintArguments
};

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: &EncodeArguments) -> Result<(), String> {
    let mut png: Png = Png::from_file(&args.path)?;
    let new_chunk: Chunk = Chunk::new(
        ChunkType::from_str(args.chunk_type.as_str())?, 
        args.message.as_bytes().to_vec()
    );
    png.append_chunk(new_chunk);

    match fs::write(&args.path, png.as_bytes()) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("Could not save to file '{:#?}'", &args.path))
    }
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: &DecodeArguments) -> Result<(), String> {
    let png: Png = Png::from_file(&args.path)?;
    let cmp: ChunkType = ChunkType::from_str(args.chunk_type.as_ref())?;

    for chunk in png.chunks().iter() {
        if !chunk.chunk_type().is_critical() && chunk.chunk_type() == &cmp {
            if let Ok(message) = chunk.data_as_string() {
                println!("{}", message);
            }
        }
    }

    Ok(())
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: &RemoveArguments) -> Result<(), String> {
    let mut png: Png = Png::from_file(&args.path)?;
    let chunk_type: &str = args.chunk_type.as_str();

    while png.remove_chunk(chunk_type).is_ok() {
        ()
    }

    match fs::write(&args.path, png.as_bytes()) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("Could not save to file '{:#?}'", &args.path))
    }
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: &PrintArguments) -> Result<(), String> {
    let png: Png = Png::from_file(&args.path)?;

    for chunk in png.chunks() {
        println!("{}", chunk);
    }

    Ok(())
}
