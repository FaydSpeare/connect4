
mod game {
    pub mod connect_game;
}

mod bits {
    pub mod bits;
}

use game::connect_game::*;
use bits::bits::*;

fn main() {
    let mut g = Game::build_game();
    g.make_move(1);
    print_i(g.light);
}
