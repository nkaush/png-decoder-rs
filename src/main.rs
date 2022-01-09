mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use args::{Executable, Subcommands};
use clap::Parser;

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
