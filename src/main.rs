mod goban;
mod translator;
mod params;

use goban::Goban;
use translator::HandlebarsTrans;

fn main() {
    let translator = HandlebarsTrans::new();

    let goban = Goban::new("echo {{N}} {{M}}".to_string(), "parameters.json".to_string(), translator);
    goban.run();
}

