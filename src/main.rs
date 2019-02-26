
mod game {
    pub mod connect_game;
    pub mod bits {
        pub mod bit;
    }
}

use game::connect_game::*;
use game::bits::bit::*;

fn main() {
    for i in 0..100000 {
        let mut g = Game::build_game();
        g.simulate_to_end();
    }


    //print_i(g.light);
    //print_i(g.dark);


}
