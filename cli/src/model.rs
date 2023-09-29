use std::path::PathBuf;

use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
/// Build and query opening lookup tables for chess positions.
pub struct Cli {
    #[argh(subcommand)]
    pub nested: CliSubCmd,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum CliSubCmd {
    Pack(PackCmd),
    Ask(AskCmd),
}

/// Initiate or populate KV-stores.
#[derive(Debug, PartialEq, FromArgs)]
#[argh(subcommand, name = "pack")]
pub struct PackCmd {
    #[argh(option, short = 'd')]
    /// moves from each side to process in a game [default: 12]
    pub depth: Option<u8>,

    #[argh(option, short = 'e')]
    /// path to a tsv file to be used as an ECO
    pub ecotsv: Option<PathBuf>,

    #[argh(option, short = 'o')]
    /// directory to store the fentrail database [default: $PWD]
    pub outdir: Option<PathBuf>,

    #[argh(positional)]
    /// path to the game database in PGN format
    pub pgn: PathBuf,
}

/// Inquire a store about a specific position.
#[derive(Debug, PartialEq, FromArgs)]
#[argh(subcommand, name = "ask")]
pub struct AskCmd {
    #[argh(option, short = 's')]
    /// path to the fentrail database [default: $PWD/fentrail.redb]
    pub store: Option<PathBuf>,

    #[argh(positional)]
    /// FEN string to query
    pub fen: String,
}
