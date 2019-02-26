
mod game {
    pub mod connect_game;
    pub mod bits {
        pub mod bit;
    }
}

use game::connect_game::*;
use game::bits::bit::*;

fn main() {
    let mut g = Game::build_game();
    g.make_move(1);
    g.make_move(8);
    g.make_move(5);
    print_i(0x1FC_0000_0000);

    let x: Vec<i32> = g.get_moves();

    for i in x {
        print!("{}, ", i);
    }

}
