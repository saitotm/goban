use clap::Parser;

use crate::{translator::HandlebarsTrans, goban::Goban};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    name: String,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

pub fn cui() {
    let args = Args::parse();
    
    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }

    let translator = HandlebarsTrans::new();

    let goban = Goban::new("echo {{N}} {{M}}".to_string(), "parameters.json".to_string(), translator);
    goban.run();
}
