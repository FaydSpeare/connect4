
mod game {
    pub mod connect_game;
    pub mod bits {
        pub mod bit;
    }
}

use game::connect_game::*;
use game::bits::bit::*;

use std::time::Instant;


fn main() {

    let start = Instant::now();

    let mut g;
    for _i in 0..100000 {
        g = Game::build_game();
        g.simulate_to_end();
    }


    g = Game::build_game();
    g.make_move(0);

    print_i(g.light);
    g.undo_move();
    print_i(g.light);

    let elapsed = start.elapsed();

    println!("Duration: {}", elapsed.as_secs() * 1000 +
        elapsed.subsec_nanos() as u64 / 1_000_000);


}
