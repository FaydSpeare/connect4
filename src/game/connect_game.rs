use rand::Rng;
use super::bits::bit::*;
use std::fmt;

const MASK: u64 = 0x1FC_0000_0000;

pub struct Game {
    pub light: u64,
    pub dark: u64,
    pub turn: bool,

    history: Vec<i32>,
    moves: Vec<i32>
}

#[allow(dead_code)]
impl Game {

    pub fn build_game() -> Game {
        Game {
            light: 0b0,
            dark: 0b0,
            turn: true,
            history: Vec::new(),
            moves: vec![0, 1, 2, 3, 4, 5, 6]
        }
    }

    pub fn get_moves(&self) -> Vec<i32> {

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

    pub fn simulate_to_end(&mut self) {

        let mut moves: Vec<i32> = self.get_moves();

        while ( (self.light|self.dark) & MASK).count_ones() != 7 {

            let r_i = rand::thread_rng().gen_range(0, moves.len());

            let r_move = moves[r_i];
            moves.swap_remove(r_i);

            if r_move + 7 < 42 {
                moves.push(r_move+7);
            }

            self.make_move(r_move);

        }
    }

    pub fn sudo_make_move(&mut self, pos: i32, player: bool){
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

    pub fn make_move(&mut self, pos: i32) {
        if pos > 41 {
            println!("DEBUG: cannot make move - out of range");
        }
        else if !self.moves.contains(&pos) {
            println!("DEBUG: cannot make move - move not in moves");
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

    pub fn sudo_undo_move(&mut self, pos: i32){
        match pos > 41 {
            true => println!("DEBUG: cannot undo move - out of range"),
            _ => {
                let n = 1 << pos;
                if is_set(self.light, pos){ self.light ^= n}
                if is_set(self.dark, pos){ self.dark ^= n}
            }
        }
    }

    pub fn undo_move(&mut self){
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
                        self.moves.retain(|&e| e != last_move);
                        if last_move > 6 { self.moves.push(last_move - 7); }
                    },
                    false => println!("DEBUG: cannot undo move - last_move was not made")
                }
            },
            None => println!("DEBUG: cannot undo move - board is empty")
        }
    }

}

impl fmt::Display for Game {

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