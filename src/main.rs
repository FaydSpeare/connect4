
mod game {
    pub mod connect_game;
    pub mod bits {
        pub mod bit;
    }
}

use game::connect_game::*;

use std::time::Instant;


fn main() {

    let start = Instant::now();

    let mut g;
    for _i in 0..1000 {
        g = Game::build_game();
        g.simulate_to_end();
    }


    g = Game::build_game();
    g.make_move(0);

    println!("{}", g);

    let elapsed = start.elapsed();

    println!("Duration: {}", elapsed.as_secs() * 1000 +
        elapsed.subsec_nanos() as u64 / 1_000_000);


}
