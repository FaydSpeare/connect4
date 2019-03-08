
mod game {
    pub mod connect_4_game;
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

use game::connect_4_game::*;
use uct::node::*;

#[allow(unused_imports)]
use game::generation::*;
//use std::env;

const COMMANDS: [&str; 4] = ["move", "undo", "analyse", "engine"];
const MODES: [&str; 2] = ["analysis", "play"];

fn main() {
    //let args: Vec<String> = env::args().collect();

    #[allow(unused_assignments)]
    let mut commands = false;
    let mut p1 = true;
    let mut p2 = true;

    let mut p1_settings = None;
    let mut p2_settings = None;

    #[allow(unused_mut, unused_assignments)]
    let mut p1_time= 0.0;

    #[allow(unused_mut, unused_assignments)]
    let mut p2_time= 0.0;

    let p1_threads;
    let p2_threads;

    loop {
        match get_user_input::<String>("Enter Mode: "){
            Ok(s) => {
                if s == "analysis" {
                    commands = true;
                    break;

                } else if s == "play"{
                    commands = false;
                    loop {
                        match get_user_input::<String>("Choose Player One (human/comp): "){
                            Ok(p) => {
                                if p == "human" {
                                    p1 = true;
                                    break;
                                }
                                else if p == "comp" {
                                    p1 = false;
                                    loop {
                                        match get_user_input::<f32>("How much time per move: "){
                                            Ok(t) => {
                                                p1_time = t;
                                                break;
                                            }
                                            _ => println!("Invalid Time")
                                        }
                                    }
                                    loop {
                                        match get_user_input::<i32>("How many threads: "){
                                            Ok(t) => {
                                                if t > 50 {
                                                    println!("Invalid - Max. 50 Threads")
                                                } else {
                                                    p1_threads = t;
                                                    break;
                                                }
                                            }
                                            _ => println!("Invalid Number")
                                        }
                                    }
                                    p1_settings = Some((p1_time, p1_threads));
                                    break;
                                } else {
                                    println!("Invalid Player - Player Types: [human, comp]");
                                }
                            }
                            _ => println!("Invalid Player - Player Types: [human, comp]")
                        }
                    }
                    loop {
                        match get_user_input::<String>("Choose Player Two (human/comp): "){
                            Ok(p) => {
                                if p == "human" {
                                    p2 = true;
                                    break;
                                }
                                else if p == "comp" {
                                    p2 = false;
                                    loop {
                                        match get_user_input::<f32>("How much time per move: "){
                                            Ok(t) => {
                                                p2_time = t;
                                                break;
                                            }
                                            _ => println!("Invalid Time")
                                        }
                                    }
                                    loop {
                                        match get_user_input::<i32>("How many threads: "){
                                            Ok(t) => {
                                                if t > 50 {
                                                    println!("Invalid - Max. 50 Threads")
                                                } else {
                                                    p2_threads = t;
                                                    break;
                                                }
                                            }
                                            _ => println!("Invalid Number")
                                        }
                                    }
                                    p2_settings = Some((p2_time, p2_threads));
                                    break;
                                } else {
                                    println!("Invalid Player - Player Types: [human, comp]");
                                }
                            }
                            _ => println!("Invalid Player - Player Types: [human, comp]")
                        }
                    }
                    break;
                } else {
                    println!("Invalid Mode - Modes: {:?}", MODES);
                }
            }
            _ => println!("Invalid Mode - Modes: {:?}", MODES)
        }
    }

    play_game(p1, p2, p1_settings, p2_settings, commands.to_owned());
}

pub fn get_user_input<T: std::str::FromStr>(string: &str) -> Result<T, <T as std::str::FromStr>::Err> {
    println!("{}", string);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("error: unable to read user input");
    input.trim().parse::<T>()
}

pub fn handle_user_move(g: &mut Connect4) {
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

pub fn offer_command() -> (i32, f32, i32) {
    let repeat: bool = true;

    while repeat {
        match get_user_input::<String>("Enter Command: "){
            Ok(command) => {
                let command = command.to_lowercase();
                if COMMANDS.contains(&command.as_ref()) {
                    match command.as_ref() {
                        "move" => return (0, 0.0, 0),
                        "undo" => return (1, 0.0, 0),
                        "analyse" => {
                            loop {
                                match get_user_input::<f32>("Enter Analysis Time: ") {
                                    Ok(time) => {
                                        loop {
                                            match get_user_input::<i32>("Enter No. Threads: ") {
                                                Ok(threads) => {
                                                    if threads > 50 {
                                                        println!("Invalid Thread Entry - Max. 50 Threads")
                                                    } else {
                                                        return (2, time, threads)
                                                    }
                                                },
                                                _ => println!("Invalid Thread Entry.")
                                            }

                                        }
                                    },
                                    _ => println!("Invalid Time Entry.")
                                }
                            }
                        }
                        "engine" => {
                            loop {
                                match get_user_input::<f32>("Enter Analysis Time: ") {
                                    Ok(time) => {
                                        loop {
                                            match get_user_input::<i32>("Enter No. Threads: ") {
                                                Ok(threads) => {
                                                    if threads > 50 {
                                                        println!("Invalid Thread Entry - Max. 50 Threads")
                                                    } else {
                                                        return (3, time, threads)
                                                    }
                                                },
                                                _ => println!("Invalid Thread Entry.")
                                            }

                                        }
                                    },
                                    _ => println!("Invalid Time Entry.")
                                }
                            }
                        }
                        _ => ()
                    }
                }
            }
            _ => ()
        }
        println!("Valid Commands: {:?}", COMMANDS);
    }
    (-1, 0.0, 0)
}

pub fn execute_command(g: &mut Connect4, command: (i32, f32, i32)) -> bool {
    if command.0 == 0 {
        handle_user_move(g);
        return false;
    }
    else if command.0 == 1 {
        g.undo_move();
    }
    else if command.0 == 2 {
        uct(g.replicate(), command.1, command.2);
    }
    else if command.0 == 3 {
        g.make_move(uct(g.replicate(), command.1, command.2));
        return false;
    }
    true
}

pub fn play_game(player_one: bool, player_two: bool, p1_settings: Option<(f32, i32)>, p2_settings: Option<(f32, i32)>, commands: bool){


    let mut g = Connect4::build_game();

    println!("Starting Position!");
    println!("{}", g);

    let mut p1_thinking_time = 0.0;
    let mut p1_threads = 0;
    if !player_one {
        match p1_settings {
            Some(time) => {
                p1_thinking_time = time.0;
                p1_threads = time.1;
            },
            None => {
                panic!("Invalid Time Argument");
            }
        }
    }

    let mut p2_thinking_time = 0.0;
    let mut p2_threads = 0;
    if !player_two {
        match p2_settings {
            Some(time) => {
                p2_thinking_time = time.0;
                p2_threads = time.1;
            },
            None => {
                panic!("Invalid Time Argument")
            }
        }
    }

    while g.get_result().is_none() {

        if commands {
            let mut repeat = true;
            while repeat {
                let command = offer_command();
                repeat = execute_command(&mut g, command);
            }
        } else {
            match player_one {
                true => {
                    handle_user_move(&mut g)
                },
                false => {
                    g.make_move(uct(g.replicate(), p1_thinking_time, p1_threads));
                }
            }
        }

        println!("{}", g);



        if g.get_result().is_none() {

            if commands {
                let mut repeat = true;
                while repeat {
                    let command = offer_command();
                    repeat = execute_command(&mut g, command);
                }
            } else {
                match player_two {
                    true => {
                        handle_user_move(&mut g);
                    },
                    false => {
                        g.make_move(uct(g.replicate(), p2_thinking_time, p2_threads));
                    }
                }
            }
            println!("{}", g);
        }
    }

    println!("Game Over.")
}

/*
//thread::sleep(Duration::from_millis(4000));

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






