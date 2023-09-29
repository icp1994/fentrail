#![warn(clippy::pedantic)]

mod model;

mod ask;
mod pack;

use model::CliSubCmd;

impl CliSubCmd {
    pub fn run(self) -> anyhow::Result<()> {
        match self {
            CliSubCmd::Pack(pack_cmd) => pack::run(pack_cmd),
            CliSubCmd::Ask(ask_cmd) => ask::run(ask_cmd),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let cli: model::Cli = argh::from_env();
    CliSubCmd::run(cli.nested)
}
