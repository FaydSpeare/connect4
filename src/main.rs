
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

use std::env;

const COMMANDS: [&str; 2] = ["move", "undo"];

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

pub fn get_user_input<T: std::str::FromStr>(string: &str) -> Result<T, <T as std::str::FromStr>::Err> {
    println!("{}", string);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("error: unable to read user input");
    input.trim().parse::<T>()
}

pub fn handle_user_move(g: &mut Game) {
    let mut repeat: bool = true;
    while repeat {
        match get_user_input::<i32>("Enter Move: ") {
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

pub fn offer_command() -> i32 {
    let repeat: bool = true;

    while repeat {
        match get_user_input::<String>("Enter Command: "){
            Ok(command) => {
                let command = command.to_lowercase();
                if COMMANDS.contains(&command.as_ref()) {
                    match command.as_ref() {
                        "move" => return 0,
                        "undo" => return 1,
                        _ => ()
                    }
                }
            }
            _ => ()
        }
        println!("Valid Commands: {:?}", COMMANDS);
    }
    -1
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

    let mut skip = false;

    while g.get_result().is_none() {

        if !skip {
            match player_one {
                true => {
                    let i = offer_command();
                    if i == 1 {
                        g.undo_move();
                        if !player_two {
                            g.undo_move();
                            skip = true;
                        }
                    } else {
                        handle_user_move(&mut g)
                    }
                },
                false => {
                    g.make_move(uct(g.replicate(), p1_thinking_time));
                }
            }
            println!("{}", g);
        } else {
            skip = false;
        }

        if g.get_result().is_none() {

            if !skip {
                match player_two {
                    true => {
                        let i = offer_command();
                        if i == 1 {
                            g.undo_move();
                            if !player_one {
                                g.undo_move();
                                skip = true;
                            }
                        } else {
                            handle_user_move(&mut g)
                        }

                    },
                    false => {
                        g.make_move(uct(g.replicate(), p2_thinking_time));
                    }
                }
                println!("{}", g);
            } else {
                skip = false;
            }
        }
    }

    println!("done");
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






