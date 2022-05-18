mod goban;
mod translator;
mod params;

use goban::Goban;

fn main() {
    let goban = Goban::new();
    goban.run();
}

