mod commands;
mod args;
mod png;

use args::{Executable, Subcommands};
use clap::Parser;

use std::convert::TryFrom;
use png::chunk_specs::IHDR;

fn main() -> Result<(), String> {    
    let args = Executable::parse();

    match &args.command {
        Subcommands::Encode(args) => commands::encode(args)?,
        Subcommands::Decode(args) => commands::decode(args)?,
        Subcommands::Remove(args) => commands::remove(args)?,
        Subcommands::Print(args) => commands::print_chunks(args)?,
    }

    Ok(())
}

// fn main() -> Result<(), String> {
//     let image = png::Png::from_file("images/dice.png")?;

//     let ihdr = IHDR::try_from(image.chunks()[0].clone())?;
//     println!("{}", ihdr);

//     Ok(())
// }
