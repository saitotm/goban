use anyhow::Result;
use clap::Parser;

use crate::{goban::Goban, translator::HandlebarsTrans};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Command to execution
    command: String,

    /// Path to a parameter file
    #[clap(short, long, default_value = "parameters.json")]
    filepath: String,
}

pub fn launch() -> Result<()> {
    let args = Args::parse();

    let translator = HandlebarsTrans::new();
    let goban = Goban::new(args.command, args.filepath, translator)?;
    goban.run()
}
