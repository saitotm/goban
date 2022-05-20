use clap::Parser;

use crate::{translator::HandlebarsTrans, goban::Goban};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Command to execution
    command: String,

    /// Path to a parameter file
    #[clap(short, long, default_value = "parameters.json")]
    filepath: String,
}

pub fn launch() {
    let args = Args::parse();

    let translator = HandlebarsTrans::new();
    let goban = Goban::new(args.command, args.filepath, translator);
    goban.run();
}
