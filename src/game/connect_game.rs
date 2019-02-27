use rand::Rng;
use super::bits::bit::*;

const MASK: u64 = 0x1FC_0000_0000;

pub struct Game {
    pub light: u64,
    pub dark: u64,
    pub turn: bool,

    history: Vec<i32>,
    moves: Vec<i32>
}

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

    pub fn undo_move(&mut self){

        match self.history.pop(){
            Some(last_move) => {
                if !is_set(self.light|self.dark, last_move){
                    println!("DEBUG: cannot undo move - last_move was not made");
                } else {
                    self.turn = !self.turn;
                    if self.turn {
                        self.light ^= 1 << last_move;
                    } else {
                        self.dark ^= 1 << last_move;
                    }
                    self.moves.retain(|&e| e != last_move);
                    if last_move > 6 {
                        self.moves.push(last_move - 7);
                    }
                }
            },
            None => println!("DEBUG: cannot undo move - board is empty")
        }



    }

    pub fn make_move(&mut self, pos: i32) {

        if !self.moves.contains(&pos) {
            println!("DEBUG: cannot make move - move not in moves");
        }
        else if is_set(self.light | self.dark, pos) {
            println!("DEBUG: cannot make move - spot already taken");
        }
        else {

            if self.turn {
                self.light |= 1 << pos;
            } else {
                self.dark |= 1 << pos;
            }

            self.turn = !self.turn;

            self.moves.retain(|&e| e != pos);
            if pos < 35 {
                self.moves.push(pos + 7);
            }
            self.history.push(pos);
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
}