use rand::Rng;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Tree {
    pub nodes: Vec<Node>
}

impl Tree {

    pub fn new() -> Tree {
        Tree {
            nodes: vec![]
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

    pub fn select_child(&self, id: usize) -> &Node {

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
        return best_child;
    }

    pub fn make_move(&mut self, id: usize) -> Node {

        let r_i = rand::thread_rng().gen_range(0, self.nodes[id].to_expand.len());
        let m = self.nodes[id].to_expand[r_i];

        let mut creation = Node::new_child(self.nodes[id].this, r_i,
                                           Option::Some(self.nodes.len()), self.nodes[id].all_moves.to_vec(),
                                           self.nodes[id].light.clone(), self.nodes[id].dark.clone(), m);

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

    pub fn push(&mut self){
        println!("ok");
    }

    pub fn run(&mut self){

        let mut root = Node::new(vec![0,1,2,3,4,5,6], true);
        let root_this = root.this.unwrap();
        self.nodes.push(root);

        for _i in 0..8 {
            let node = &self.nodes[0];

            let mut depth = 0;
            while node.is_not_expandable() {
                let node = self.select_child(node.this.unwrap());
                depth += 1;
                if node.is_terminal() {
                    break;
                }
                println!("hi");
            }

            if node.this.unwrap() == root_this {
                if node.is_terminal() {
                    self.update(node.terminal_value, node.this.unwrap());
                }
            }

            let expanded = self.make_move(node.this.unwrap());

            let exp = expanded.this.unwrap();
            self.nodes.push(expanded);
            let expanded = &self.nodes[exp];


            println!("node {}", expanded.last_move);
        }
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

    pub fn new_child(parent: Option<usize>, move_index: usize, this: Option<usize>, mut p_moves: Vec<i32>, light: u64, dark: u64, last_move: i32) -> Node {
        p_moves.swap_remove(move_index);
        Node {
            wins: 0.0,
            visits: 0.0,
            parent,
            this,
            last_move,
            to_move: true, //TODO
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
        let mut expand: f32 = (2.0 * visits.log10()) / self.visits;
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

pub fn uct(){
    Tree::new().run();
}



/*
pub fn uct(){

    let tree = Rc::new(RefCell::new(Tree::new()));

    let mut root = Node::new(vec![0,1,2,3,4,5,6], true);
    let root_this = root.this.unwrap();

    {
        (*tree).borrow_mut().nodes.push(root);
    }

    for _i in 0..1 {

        let t = Rc::clone(&tree);
        let t2 = t.borrow_mut();
        let node = &tree.borrow().nodes[0];

        let mut depth = 0;
        let t = tree.borrow();
        while node.is_not_expandable() {
            let node = t.select_child(node.this.unwrap());
            depth += 1;
            if node.is_terminal() {
                break;
            }
        }


        if node.this.unwrap() == root_this {
            if node.is_terminal() {
                //tree.borrow_mut().update(node.terminal_value, node.this.unwrap());
            }
        }



        //let expanded = tree.borrow_mut().make_move(node.this.unwrap());

        /*
        let exp = expanded.this.unwrap();
        tree.nodes.push(expanded);
        let expanded = &tree.nodes[exp];


        println!("node {}", expanded.last_move);
        */

    }
*/
