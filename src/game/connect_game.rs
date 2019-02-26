
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

    pub fn make_move(&mut self, pos: i32) -> bool {
        self.light |= 1 << pos;
        true
    }

}