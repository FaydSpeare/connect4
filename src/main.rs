
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
use std::collections::HashMap;

fn main() {

    let start = Instant::now();
    let elapsed = start.elapsed();



    let mut g = Game::build_game();
    //g.make_move(3);
    let mut b = true;

    while g.get_result().is_none() {
        if b {
            if g.turn {
                g.make_move(uct(g.replicate(), 1.0));
            } else {
                g.make_move(uct(g.replicate(), 1.0));
            }
        }
        b = true;
        let elapsed = start.elapsed();
        let x = ((elapsed.as_secs() * 1000 +
            elapsed.subsec_nanos() as u64 / 1_000_000) as f32) /1000.0;

        println!("Duration: {}", x);

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



    //hashing();
}

pub fn hashing(){

    let mut g1 = Game::build_game();
    let mut g2 = Game::build_game();
    g2.make_rand_move();
    g1 = g2.replicate();

    let mut x: HashMap<Game, i32> = HashMap::new();

    x.insert(g1, 0);


    println!("{}", x.get(&g2).unwrap());









}






