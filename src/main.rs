
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
use uct::node::*;

#[allow(unused_imports)]
use game::generation::*;
use std::num::ParseIntError;

use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 { panic!("Invalid Arguments Given"); }

    let p1: bool = match args[1].parse(){
        Ok(t) => t,
        Err(e) => panic!("{}", e)
    };

    let p2: bool = match args[2].parse(){
        Ok(t) => t,
        Err(e) => panic!("{}", e)
    };

    let mut p1_time: Option<f32> = None;
    let mut p2_time: Option<f32> = None;

    if !p1 {
        if args.len() <= 3 { panic!("Invalid Arguments Given"); }
        match args[3].parse() {
            Ok(t) => p1_time = Some(t),
            Err(e) => panic!("{}", e)
        }
    }

    if !p2 {
        let mut x = 3;
        if !p1 { x += 1 }
        if args.len() <= x { panic!("Invalid Arguments Given"); }
        match args[x].parse() {
            Ok(t) => p2_time = Some(t),
            Err(e) => panic!("{}", e)
        }
    }

    play_game(p1, p2, p1_time, p2_time);
}

pub fn get_user_move() -> Result<i32, ParseIntError> {
    println!("enter move: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("error: unable to read user input");
    input.trim().parse::<i32>()
}

pub fn handle_user_move(g: &mut Game) {
    let mut repeat: bool = true;
    while repeat {
        match get_user_move() {
            Ok(k) => {
                if !g.moves.contains(&k){
                    println!("Invalid Entry - move cannot be made");
                } else {
                    g.make_move(k);
                    repeat = false;
                }
            },
            _ => println!("Invalid Entry - enter a move")
        }
    }
}

pub fn play_game(player_one: bool, player_two: bool, p1_time: Option<f32>, p2_time: Option<f32>){

    let mut g = Game::build_game();
    println!("Starting Position!");
    println!("{}", g);

    let mut p1_thinking_time = 0.0;
    if !player_one {
        match p1_time {
            Some(time) => p1_thinking_time = time,
            None => {
                panic!("Invalid Time Argument");
            }
        }
    }

    let mut p2_thinking_time = 0.0;
    if !player_two {
        match p2_time {
            Some(time) => p2_thinking_time = time,
            None => {
                panic!("Invalid Time Argument")
            }
        }
    }

    while g.get_result().is_none() {

        match player_one {
            true => handle_user_move(&mut g),
            false => {
                g.make_move(uct(g.replicate(), p1_thinking_time));
            }
        }
        println!("{}", g);

        if g.get_result().is_none() {
            match player_two {
                true => handle_user_move(&mut g),
                false => {
                    g.make_move(uct(g.replicate(), p2_thinking_time));
                }
            }
        }
        println!("{}", g);
    }
}

//thread::sleep(Duration::from_millis(4000));

/*
use std::collections::HashMap;

pub fn hashing(){

    let mut g1 = Game::build_game();
    let mut g2 = Game::build_game();
    g2.make_rand_move();
    g1 = g2.replicate();

    let mut x: HashMap<Game, i32> = HashMap::new();

    x.insert(g1, 0);


    println!("{}", x.get(&g2).unwrap());
}
*/






