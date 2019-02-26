use rand::Rng;
use super::bits::bit::*;

const MASK: u64 = 0x1FC_0000_0000;

pub struct Game {
    pub light: u64,
    pub dark: u64,
    pub turn: bool
}

impl Game {

    pub fn build_game() -> Game {
        Game {
            light: 0b0,
            dark: 0b0,
            turn: true
        }
    }

    pub fn make_move(&mut self, pos: i32) {

        if is_set(self.light | self.dark, pos) {
            panic!("DEBUG: Spot Already Taken");
        }
        if self.turn {
            self.light |= 1 << pos;
        } else {
            self.dark |= 1 << pos;
        }
        self.turn = !self.turn;
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

        while ((self.light|self.dark) & MASK).count_ones() != 7 {


            let r_i = rand::thread_rng().gen_range(0, moves.len());

            let r_move = moves[r_i];
            moves[r_i] = moves[moves.len()-1];
            moves.pop();

            if r_move + 7 < 42 {
                moves.push(r_move+7);
            }

            self.make_move(r_move);

        }
    }
}