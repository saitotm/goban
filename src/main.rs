mod goban;

use goban::Goban;

fn main() {
    let goban = Goban::new();
    goban.run();
}

