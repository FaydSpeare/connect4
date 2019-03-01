use rand::Rng;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Tree<'t> {
    pub nodes: Vec<Node<'t>>
}

impl<'t> Tree<'t> {

    pub fn new() -> Tree<'t> {
        Tree {
            nodes: vec![]
        }
    }
}

pub struct Node<'t> {

    tree: &'t mut Tree<'t>,

    pub wins: f32,
    pub visits: f32,

    pub parent: Option<usize>,
    pub this: Option<usize>,
    pub last_move: i32,

    pub to_move: bool,

    pub all_moves: Vec<i32>,
    pub to_expand: Vec<i32>,

    children: Vec<Node<'t>>,

    pub terminal: bool,
    pub terminal_value: f32,
    pub terminal_depth: i32,

    pub terminal_sum: f32
}

impl<'t> Node<'t> {

    pub fn new(moves: Vec<i32>, to_move: bool, tree: &'t mut Tree<'t>) -> Node<'t> {
        let mut x = 0;
        {
            x = tree.nodes.len();
        }
        Node {
            tree,
            wins: 0.0,
            visits: 0.0,
            parent: None,
            this: Option::Some(x),
            last_move: -1,
            to_move,
            all_moves: moves.to_vec(),
            to_expand: moves.to_vec(),
            children: vec![],
            terminal: false,
            terminal_value: 0.0,
            terminal_depth: -1,
            terminal_sum: 0.0
        }
    }

    pub fn new_child(parent: Option<usize>, move_index: usize, tree: &'t mut Tree<'t>) -> Node<'t> {
        let mut moves: Vec<i32> = tree.nodes[parent.unwrap()].all_moves.to_vec();
        moves.swap_remove(move_index);
        let mut x= 0;
        {
            x = tree.nodes.len();
        }
        Node {
            tree,
            wins: 0.0,
            visits: 0.0,
            parent,
            this: Option::Some(x), //TODO
            last_move: -1,
            to_move: true, //TODO
            all_moves: moves.to_vec(),
            to_expand: moves,
            children: vec![],
            terminal: false,
            terminal_value: 0.0,
            terminal_depth: -1,
            terminal_sum: 0.0
        }
    }

    pub fn update(&mut self, value: f32){
        self.wins += value;
        self.visits += 1.0;
        if let Some(p) = self.parent {
            self.tree.nodes[p].update(value);
        }

    }

    pub fn select_child(&self) -> usize {

        let mut best_uct = self.children[0].uct();
        let mut best_child = self.children[0].this;

        match self.to_move {
            false => {
                for child in self.children.iter() {
                    let uct = child.uct();
                    if uct <= best_uct {
                        best_uct = uct;
                        best_child = child.this;
                    }
                }
            }
            true => {
                for child in self.children.iter() {
                    let uct = child.uct();
                    if uct >= best_uct {
                        best_uct = uct;
                        best_child = child.this;
                    }
                }
            }
        }
        return best_child.unwrap();
    }

    fn uct(&self) -> f32 {
        match self.parent {
            Some(p) => {
                let mut expand: f32 = (2.0*self.tree.nodes[p].visits.log10())/self.visits;
                expand = expand.sqrt();

                if self.to_move {
                    expand *= -1.0;
                }

                return self.wins/self.visits + expand;
            }
            None => panic!("DEBUG: uct - parent was None")
        }
    }

    pub fn is_not_expandable(&self) -> bool {
        self.to_expand.is_empty()
    }

    pub fn make_move(&'t mut self) -> usize {
        let r_i = rand::thread_rng().gen_range(0, self.to_expand.len());
        let m = self.to_expand[r_i];

        let mut creation = Node::new_child(self.this, r_i, self.tree);

        if m < 35 {
            let new_move = m + 7;
            creation.all_moves.push(new_move);
            creation.to_expand.push(new_move);
        }
        // TODO children capacity

        // TODO move making

        creation.to_move = !creation.to_move;
        self.children.push(creation);
        self.to_expand.pop();

        self.children.last().unwrap().this.unwrap()
    }

    pub fn set_terminal(&mut self, terminal: bool){
        self.terminal = terminal;
    }

    pub fn is_terminal(&self) -> bool {
        self.terminal
    }

    pub fn add_to_sum(&mut self, value: f32){
        self.terminal_sum += value;
    }

    pub fn set_terminal_value(&mut self, value: f32, mut depth: i32){
        self.terminal_value = value;
        self.terminal_depth = depth;
        self.terminal = true;

        match self.parent {
            Some(p) => {
                depth += 1;
                match self.tree.nodes[p].to_move {
                    true => {
                        if self.terminal_value == 1.0 {
                            self.tree.nodes[p].set_terminal_value(value, depth);
                        }
                        else if self.terminal_value == -1.0 {
                            self.tree.nodes[p].add_to_sum(self.terminal_value);
                            if self.tree.nodes[p].terminal_sum / (self.tree.nodes[p].children.len() as f32) == -1.0 {
                                self.tree.nodes[p].set_terminal_value(self.terminal_value, depth);
                            }
                        }
                    }
                    false => {
                        if self.terminal_value == -1.0 {
                            self.tree.nodes[p].set_terminal_value(value, depth);
                        }
                        else if self.terminal_value == 1.0 {
                            self.tree.nodes[p].add_to_sum(self.terminal_value);
                            if self.tree.nodes[p].terminal_sum / (self.tree.nodes[p].children.len() as f32) == 1.0 {
                                self.tree.nodes[p].set_terminal_value(self.terminal_value, depth);
                            }
                        }
                    }
                }
            }
            None => ()
        }
    }

}

pub fn uct(){

    let tree = Rc::new(RefCell::new(Tree::new()));
    let root = Node::new(vec![0,1,2,3,4,5,6], true, &mut tree.borrow_mut());

    for _i in 0..7 {

        let mut node = root;

        let mut depth = 0;
        while node.is_not_expandable() {
            node = tree.borrow().nodes[node.select_child()];
            depth += 1;
            if node.is_terminal() {
                break;
            }
        }

        if node.this.unwrap() == root.this.unwrap() {
            if node.is_terminal() {
                node.update(node.terminal_value);
            }
        }





    }

}