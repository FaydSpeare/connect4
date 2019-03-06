extern crate time;

use rand::Rng;
use std::collections::HashMap;

use super::super::game::connect_4_game::*;
use std::thread;
use std::sync::Mutex;
use std::sync::Arc;
use self::time::PreciseTime;

pub struct Tree {
    pub nodes: Vec<Node>,
    pub map: HashMap<(u64, u64), usize>
}

impl Tree {

    pub fn new() -> Tree {
        Tree {
            nodes: vec![],
            map: HashMap::new()
        }
    }

    pub fn update(&mut self, value: f32, id: usize){
        self.nodes[id].wins += value;
        self.nodes[id].visits += 1.0;
        if let Some(p) = self.nodes[id].parent {
            self.update(value, p);
        }
    }

    pub fn set_terminal_value(&mut self, value: f32, mut depth: i32, id: usize){
        self.nodes[id].terminal_value = value;
        self.nodes[id].terminal_depth = depth;
        self.nodes[id].terminal = true;

        match self.nodes[id].parent {
            Some(p) => {
                depth += 1;
                match self.nodes[p].to_move {
                    true => {
                        if self.nodes[id].terminal_value == 1.0 {
                            self.set_terminal_value(value, depth, p);
                        }
                        else if self.nodes[id].terminal_value == -1.0 {
                            self.nodes[p].add_to_sum(value);
                            if self.nodes[p].terminal_sum / (self.nodes[p].children.len() as f32) == -1.0 {
                                self.set_terminal_value(value, depth, p);
                            }
                        }
                    }
                    false => {
                        if self.nodes[id].terminal_value == -1.0 {
                            self.set_terminal_value(value, depth, p);
                        }
                        else if self.nodes[id].terminal_value == 1.0 {
                            self.nodes[p].add_to_sum(value);
                            if self.nodes[p].terminal_sum / (self.nodes[p].children.len() as f32) == 1.0 {
                                self.set_terminal_value(value, depth, p);
                            }
                        }
                    }
                }
            }
            None => ()
        }
    }

    pub fn select_child(&self, id: usize) -> usize {

        let mut best_uct = self.nodes[self.nodes[id].children[0]].uct(self.nodes[id].visits);
        let mut best_child = &self.nodes[self.nodes[id].children[0]];

        match self.nodes[id].to_move {
            false => {
                for &child in self.nodes[id].children.iter() {
                    let uct = self.nodes[child].uct(self.nodes[id].visits);
                    if uct <= best_uct {
                        best_uct = uct;
                        best_child = &self.nodes[child];
                    }
                }
            }
            true => {
                for &child in self.nodes[id].children.iter() {
                    let uct = self.nodes[child].uct(self.nodes[id].visits);
                    if uct >= best_uct {
                        best_uct = uct;
                        best_child = &self.nodes[child];
                    }
                }
            }
        }
        return best_child.this.unwrap();
    }

    pub fn make_move(&mut self, id: usize) -> Node {


        if self.nodes[id].to_expand.len() == 0 { println!("\n\n WHAT WHAT \n\n")}
        let r_i = rand::thread_rng().gen_range(0, self.nodes[id].to_expand.len());
        let m = self.nodes[id].to_expand[r_i];

        //println!("parent a-moves: {:?}", self.nodes[id].all_moves);
        //println!("parent e-moves: {:?}", self.nodes[id].to_expand);

        let mut creation = Node::new_child(self.nodes[id].this, m,
                                           Option::Some(self.nodes.len()), self.nodes[id].all_moves.to_vec(),
                                           self.nodes[id].light.clone(), self.nodes[id].dark.clone(), m, self.nodes[id].to_move);

        //println!("making move: {}", m);
        //println!("pre a-moves: {:?}", creation.all_moves);
        //println!("pre e-moves: {:?}", creation.to_expand);

        if m < 35 {
            let new_move = m + 7;
            creation.all_moves.push(new_move);
            creation.to_expand.push(new_move);
        }
        // TODO children capacity

        // TODO move making

        creation.to_move = !creation.to_move;
        self.nodes[id].children.push(creation.this.unwrap());
        self.nodes[id].to_expand.swap_remove(r_i);

        creation
    }

    pub fn run<U: UCTGame>(&mut self, game: U, allowed: f32, verbose: bool, start: PreciseTime) -> Vec<(i32, f32)> {

        //let start = time::PreciseTime::now();
        let mut it = 0;
        let mut max_depth = 0;

        let root = Node::new(game.get_moves(), game.get_turn());
        let root_this = root.this.unwrap();
        self.nodes.push(root);

        // ensure search does not run overtime.
        let allowed = allowed - 0.001;

        // loop while time allows
        while (start.to(time::PreciseTime::now()).num_milliseconds() as f32)/1000.0 < allowed {
            it += 1;

            let mut g = game.replicate();
            let mut id = 0;
            let mut depth = 0;

            // SELECT NODE
            while self.nodes[id].is_not_expandable() {
                id = self.select_child(self.nodes[id].this.unwrap());
                g.make_move(self.nodes[id].last_move);
                depth += 1;
                if self.nodes[id].is_terminal() {
                    break;
                }
            }

            // update depth
            if depth > max_depth {
                max_depth = depth;
            }

            // UPDATE IF TERMINAL
            if id != root_this {
                if self.nodes[id].is_terminal() {
                    let i = self.nodes[id].this.unwrap();
                    self.update(self.nodes[id].terminal_value, i);
                    continue;
                }
            }

            // EXPAND
            let expanded = self.make_move(id);
            self.nodes.push(expanded);
            let e_id = self.nodes.len()-1;

            // update game board
            g.make_move(self.nodes[e_id].last_move);

            // update node bit-boards
            //self.nodes[e_id].light = g.light;
            //self.nodes[e_id].dark = g.dark;

            // query result of expanded
            let result = g.get_result();

            // SIMULATE IF NOT TERMINAL
            match result {
                Some((a, _b)) => {
                    self.nodes[e_id].set_terminal(true);
                    self.set_terminal_value(a, 1, e_id);
                }
                None => {
                    g.simulate_to_end();
                }
            }

            // UPDATE WITH RESULT
            self.update(g.get_result().unwrap().0, e_id);
        }

        if self.nodes[0].children.len() == 0{
            println!("no search");
            return Vec::new();
        }

        // assess best move
        let first_child = &self.nodes[self.nodes[0].children[0]];
        let mut score = first_child.wins / first_child.visits;
        let mut best_move = self.nodes[self.nodes[0].children[0]].last_move;


        if verbose {
            println!("{:-<100}{}",">","<");
            println!(" Search Returned the Following: ~ verbose = TRUE");
            println!("{:-<100}{}",">","<");
        }

        let mut children: Vec<(i32, f32)> = Vec::new();
        for &child in self.nodes[0].children.iter() {

            if verbose {
                self.print_info(child);
            }

            let s = self.nodes[child].wins / self.nodes[child].visits;
            match game.get_turn() {
                true => {
                    if s >= score {
                        score = s;
                        best_move = self.nodes[child].last_move;
                    }
                    if children.len() == 0{
                        children.push((self.nodes[child].last_move, s));
                    } else {
                        for i in 0..children.len() {
                            if s >= children[i].1 {
                                children.insert(i, (self.nodes[child].last_move, s));
                                break;
                            }
                            else if i == children.len() - 1 {
                                children.push((self.nodes[child].last_move, s));
                            }
                        }
                    }

                },
                false => {
                    if s <= score {
                        score = s;
                        best_move = self.nodes[child].last_move;
                    }
                    if children.len() == 0{
                        children.push((self.nodes[child].last_move, s));
                    } else {
                        for i in 0..children.len() {
                            if s <= children[i].1 {
                                children.insert(i, (self.nodes[child].last_move, s));
                                break;
                            }
                            else if i == children.len() - 1 {
                                children.push((self.nodes[child].last_move, s));
                            }
                        }
                    }
                }
            }


        }
        if verbose {  println!("{:-<100}{}",">","<"); }
        if verbose {
            let duration = ((start.to(time::PreciseTime::now())).num_milliseconds() as f32)/1000.0;
            println!(" Iterations: {} - Iterations/s {} - Max-Depth: {} - Time: {:?}", it, (it as f32)/duration,
                     max_depth, duration);
            println!("{:-<100}{}",">","<");
        }


        if best_move == 1 {
            // woot
        }
        return children;
    }

    pub fn print_info(&self, child: usize){
        println!("move: {: <4} - wins: {: <8} - visits: {: <10} - value: {: <8.3} - t-val: {: <3} - t-depth: {}",
                 self.nodes[child].last_move,
                 self.nodes[child].wins,
                 self.nodes[child].visits,
                 self.nodes[child].wins / self.nodes[child].visits,
                 self.nodes[child].terminal_value,
                 self.nodes[child].terminal_depth,
        );
    }

}

pub struct Node {

    pub wins: f32,
    pub visits: f32,

    pub parent: Option<usize>,
    pub this: Option<usize>,
    pub last_move: i32,

    pub to_move: bool,
    pub light: u64,
    pub dark: u64,

    pub all_moves: Vec<i32>,
    pub to_expand: Vec<i32>,

    children: Vec<usize>,

    pub terminal: bool,
    pub terminal_value: f32,
    pub terminal_depth: i32,

    pub terminal_sum: f32
}

impl Node {
    pub fn new(moves: Vec<i32>, to_move: bool) -> Node {
        Node {
            wins: 0.0,
            visits: 0.0,
            parent: None,
            this: Option::Some(0),
            last_move: -1,
            to_move,
            light: 0b0,
            dark: 0b0,
            all_moves: moves.to_vec(),
            to_expand: moves.to_vec(),
            children: vec![],
            terminal: false,
            terminal_value: 0.0,
            terminal_depth: -1,
            terminal_sum: 0.0
        }
    }

    pub fn new_child(parent: Option<usize>, move_pos: i32, this: Option<usize>,
                     mut p_moves: Vec<i32>, light: u64, dark: u64,
                     last_move: i32, to_move: bool) -> Node {
        p_moves.retain(|&e| e != move_pos);
        Node {
            wins: 0.0,
            visits: 0.0,
            parent,
            this,
            last_move,
            to_move, //TODO
            light,
            dark,
            all_moves: p_moves.to_vec(),
            to_expand: p_moves,
            children: vec![],
            terminal: false,
            terminal_value: 0.0,
            terminal_depth: -1,
            terminal_sum: 0.0
        }
    }

    fn uct(&self, visits: f32) -> f32 {
        let mut expand: f32 = (3.0 * visits.log10()) / self.visits;
        expand = expand.sqrt();
        if self.to_move {
            expand *= -1.0;
        }
        return self.wins / self.visits + expand;
    }

    pub fn is_not_expandable(&self) -> bool {
        self.to_expand.is_empty()
    }

    pub fn set_terminal(&mut self, terminal: bool) {
        self.terminal = terminal;
    }

    pub fn is_terminal(&self) -> bool {
        self.terminal
    }

    pub fn add_to_sum(&mut self, value: f32) {
        self.terminal_sum += value;
    }
}

pub fn uct< U: UCTGame + Send + 'static>(game: U, allowed: f32, threads: i32) -> i32 {
    let start = time::PreciseTime::now();
    if threads > 1 {
        let averages: Arc<Mutex<Vec<(i32, f32)>>> = Arc::new(Mutex::new(Vec::new()));
        for k in game.get_moves() {
            let mut guard = averages.lock().unwrap();
            let a = &mut *guard;
            a.push((k, 0.0));
        }
        // Make a vector to hold the children which are spawned.
        let mut children = vec![];

        for _i in 0..threads {

            let g2 = game.replicate();
            let av = averages.clone();

            // Spin up another thread
            children.push(thread::spawn(move || {
                //println!("start thread {}", i);
                let v = Tree::new().run(g2, allowed - 0.001*(threads as f32), false, start);
                let mut guard = av.lock().unwrap();
                let av = &mut *guard;

                for j in 0..v.len() {
                    for m in 0..v.len() {
                        if av[m].0 == v[j].0 {
                            av[m].1 += v[j].1;
                        }
                    }
                }
            }));
        }

        for child in children {
            // Wait for the thread to finish. Returns a result.
            let _ = child.join();
        }

        let mut guard = averages.lock().unwrap();
        let a = &mut *guard;
        let mut b: Vec<(i32, f32)> = a.iter().map(|&(a, b)| (a, b / (threads as f32))).collect();


        b.sort_by(|a, b| (b.1).partial_cmp(&a.1).unwrap());
        println!("{:?}", b);

        println!("time taken: {}", (start.to(time::PreciseTime::now()).num_milliseconds() as f32)/1000.0);
        if game.get_turn() {
            return b[0].0;
        }

        return b[b.len()-1].0;
    } else {
        let v = Tree::new().run(game.replicate(), allowed, true, start);
        return v[0].0;
    }


}


