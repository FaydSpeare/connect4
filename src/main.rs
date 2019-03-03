
mod game {
    pub mod connect_game;
    pub mod bits {
        pub mod bit;
    }
    pub mod generation {
        pub mod generator;
    }
}

mod uct {
    pub mod node;
}


use game::connect_game::*;
use std::{thread, time};
use uct::node::*;

#[allow(unused_imports)]
use game::generation::*;

use std::time::Instant;
use std::time::Duration;


fn main() {



    /*
    let mut g;
    for _i in 0..400000 {
        g = Game::build_game();
        g.simulate_to_end();
        //println!("{}", g);
    }
    */


    /*
    g = Game::build_game();
    g.make_move(0);

    for _j in 0..20 {g.make_rand_move();}

    println!("{}", g);
    */


    let mut g = Game::build_game();
    //g.make_move(3);
    let mut b = true;

    while g.get_result().is_none() {
        let start = Instant::now();
        if b {
            if g.turn {
                g.make_move(uct(g.replicate(), 5000000));
            } else {
                g.make_move(uct(g.replicate(), 500000));
            }
        }
        b = true;
        let elapsed = start.elapsed();

        println!("Duration: {}", elapsed.as_secs() * 1000 +
            elapsed.subsec_nanos() as u64 / 1_000_000);

        //thread::sleep(Duration::from_millis(4000));
        println!("{}", g);







        println!("enter move: ");
        let mut input = String::new();

        std::io::stdin().read_line(&mut input).expect("error: unable to read user input");

        let i = input.trim().parse::<i32>();
        match i {
            Ok(k) => g.make_move(k),
            _ => println!("nope")
        }


    }

    println!("{}", g);







}






