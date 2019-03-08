extern crate rayon;

use rayon::prelude::*;

use rand::Rng;
use super::bits::bit::*;
use super::generation::generator::WINS;
use std::fmt;
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;


const MASK: u64 = 0x1FC_0000_0000;

#[derive(PartialEq, Eq, Hash)]
pub struct Connect4 {
    pub light: u64,
    pub dark: u64,
    pub turn: bool,

    history: Vec<i32>,
    pub moves: Vec<i32>
}

#[allow(dead_code)]
impl UCTGame for Connect4 {

    fn build_game() -> Connect4 {
        Connect4 {
            light: 0b0,
            dark: 0b0,
            turn: true,
            history: Vec::new(),
            moves: vec![0, 1, 2, 3, 4, 5, 6]
        }
    }

    fn get_moves(&self) -> Vec<i32> {

        let occupied = self.light|self.dark;
        let size = (occupied & MASK).count_ones();
        let mut moves: Vec<i32> = Vec::with_capacity(size as usize);

        for i in 0..=6 {
            let mut x = i;

            while x < 42 {
                if !is_set(occupied, x){
                    moves.push(x);
                    break;
                }
                x += 7;
            }
        }

        return moves;
    }

    fn simulate_to_end(&mut self) {
        while self.get_result() == None {
            self.make_rand_move();
        }
    }

    fn sudo_make_move(&mut self, pos: i32, player: bool){
        match pos > 41 {
            true => println!("DEBUG: cannot make move - out of range"),
            _ => {
                let n = 1 << pos;
                match player {
                    true => self.light |= n,
                    false => self.dark |= n
                }
            }
        }
    }

    fn make_move(&mut self, pos: i32) {
        if pos > 41 {
            println!("DEBUG: cannot make move - out of range");
        }
        else if !self.moves.contains(&pos) {
            println!("DEBUG: cannot make move - move not in moves");
            println!("DEBUG: move: {} - moves: {:?}", pos, self.moves);
            panic!();
        }
        else if is_set(self.light | self.dark, pos) {
            println!("DEBUG: cannot make move - spot already taken");
        }
        else {
            let n = 1 << pos;
            match self.turn {
                true => self.light |= n,
                false => self.dark |= n
            }
            self.turn = !self.turn;

            self.moves.retain(|&e| e != pos);
            if pos < 35 { self.moves.push(pos + 7); }
            self.history.push(pos);
        }

    }

    fn make_rand_move(&mut self){
        let r_i = rand::thread_rng().gen_range(0, self.moves.len());
        let pos = self.moves[r_i];

        let n = 1 << pos;
        match self.turn {
            true => self.light |= n,
            false => self.dark |= n
        }
        self.turn = !self.turn;

        self.moves.swap_remove(r_i);
        if pos < 35 { self.moves.push(pos + 7); }
        self.history.push(pos);
    }

    fn sudo_undo_move(&mut self, pos: i32){
        match pos > 41 {
            true => println!("DEBUG: cannot undo move - out of range"),
            _ => {
                let n = 1 << pos;
                if is_set(self.light, pos){ self.light ^= n}
                if is_set(self.dark, pos){ self.dark ^= n}
            }
        }
    }

    fn undo_move(&mut self){
        match self.history.pop(){
            Some(last_move) => {
                match is_set(self.light|self.dark, last_move) {
                    true => {
                        self.turn = !self.turn;
                        let n = 1 << last_move;
                        match self.turn {
                            true => self.light ^= n,
                            false => self.dark ^= n
                        }
                        self.moves.retain(|&e| e != last_move + 7);
                        self.moves.push(last_move);
                    },
                    false => println!("DEBUG: cannot undo move - last_move was not made")
                }
            },
            None => println!("DEBUG: cannot undo move - board is empty")
        }
    }

    fn get_result(&self) -> Option<(f32, u64)> {

        for &long in WINS.iter() {
            if self.light & long == long {
                return Some((1.0, long));
            }
            else if self.dark & long == long {
                return Some((-1.0, long));
            }
        }
        if (self.light | self.dark) & MASK == MASK {
            return Some((0.0,0));
        }
        return None;
    }

    fn replicate(&self) -> Connect4 {
        Connect4 {
            light: self.light.clone(),
            dark: self.dark.clone(),
            turn: self.turn.clone(),
            history: self.history.to_vec(),
            moves: self.moves.to_vec()
        }
    }

    fn get_turn(&self) -> bool {
        self.turn
    }
}

impl fmt::Display for Connect4 {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();

        s.push_str("------------------------------");
        s.push_str("\n            ");
        s.push_str(" _______________\n");
        for i in vec![5, 4, 3, 2, 1, 0] {
            match i {
                5 => {
                    s.push_str("Game:       ");
                }
                3 => {
                    s.push_str("Move: ");
                    match self.turn {
                        true => s.push('O'),
                        false => s.push('X')
                    }
                    s.push_str("     ");
                }
                2 => {
                    s.push_str("O's: ");
                    s.push_str(&self.light.count_ones().to_string());
                    s.push_str("     ");
                    if self.light.count_ones() < 10 { s.push(' '); }
                }
                1 => {
                    s.push_str("X's: ");
                    s.push_str(&self.dark.count_ones().to_string());
                    s.push_str("     ");
                    if self.dark.count_ones() < 10 { s.push(' '); }
                }
                _ => s.push_str("            ")
            }
            s.push('|');
            s.push(' ');
            for j in 0..7 {
                let k = 7*i + j;
                match is_set(self.light, k){
                    true => s.push('O'),
                    _ => {
                        match is_set(self.dark, k){
                            true => s.push('X'),
                            false => s.push('-')
                        }
                    }
                }
                s.push(' ');
            }
            s.push('|');
            s.push('\n');
        }
        s.push_str("            ");
        s.push_str("\\_______________/");
        s.push_str("\n            ");
        s.push_str(" ||           || \n");
        s.push_str("------------------------------");

        write!(f, "{}", s)
    }

}

pub trait UCTGame {

    fn build_game() -> Self;

    fn get_moves(&self) -> Vec<i32>;

    fn simulate_to_end(&mut self);

    fn sudo_make_move(&mut self, pos: i32, player: bool);

    fn make_move(&mut self, pos: i32);

    fn make_rand_move(&mut self);

    fn sudo_undo_move(&mut self, pos: i32);

    fn undo_move(&mut self);

    fn get_result(&self) -> Option<(f32, u64)>;

    fn replicate(&self) -> Self;

    fn get_turn(&self) -> bool;
}


