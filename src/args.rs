use clap::{AppSettings, Parser, Subcommand, Args};
use std::path::PathBuf;

/// A PNG image encoder/decoder used to hide messages in PNG images.
#[derive(Parser)]
#[clap(name = "png")]
#[clap(version, author)]
#[clap(about = "A PNG image encoder/decoder used to hide messages in PNG images.")]
pub struct Executable {
    #[clap(subcommand)]
    pub command: Subcommands,
}

#[derive(Subcommand)]
pub enum Subcommands {
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Encode(EncodeArguments),
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Decode(DecodeArguments),
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Remove(RemoveArguments),
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Print(PrintArguments)
}

#[derive(Args)]
pub struct EncodeArguments {
    #[clap(required = true, parse(from_os_str))]
    /// The path to the PNG image to encode a message within
    pub path: PathBuf,
    /// The 4-byte chunk type code to use to add messages under
    pub chunk_type: String,
    /// The message to encode
    pub message: String,
    /// [Optional] The filepath to write the encoded image to
    pub destination: Option<PathBuf>
}

#[derive(Args)]
pub struct DecodeArguments {
    #[clap(required = true, parse(from_os_str))]
    /// The path to the PNG image to decode messages from
    pub path: PathBuf,
    /// The 4-byte chunk type code to use to search for messages to decode
    pub chunk_type: String
}

#[derive(Args)]
pub struct RemoveArguments {
    #[clap(required = true, parse(from_os_str))]
    /// The path to the PNG image to remove encoded messages from
    pub path: PathBuf,
    /// The 4-byte chunk type code to use to search for messages to remove
    pub chunk_type: String,
}

#[derive(Args)]
pub struct PrintArguments {
    #[clap(required = true, parse(from_os_str))]
    /// The path to the PNG image to print chunks for
    pub path: PathBuf
}
