use rand::seq::{SliceRandom, IteratorRandom};
use pyo3::prelude::*;
use crate::env::{UTTTEnvImpl, action_to_move, move_to_action};
use crate::game::{Game, Player, Move};

const UCT_C: f32 = 1.41;

struct Node {
    visits: u32,
    reward: f32,
    parent: Option<usize>,
    action: Option<Move>,
    children: Vec<(Move, Option<usize>)>,
}

impl Node {
    fn new(state: &Game, parent: Option<usize>, action: Option<Move>) -> Self {
        Self {
            parent,
            children: state.valid_moves().into_iter().map(|action| (action, None)).collect(),
            visits: 0,
            reward: 0.0,
            action,
        }
    }

    fn fully_expanded(&self) -> bool {
        self.children.iter().all(|(_, node)| node.is_some())
    }

    fn is_terminal(&self) -> bool {
        self.children.is_empty()
    }

    fn uct_value(&self, parent_visits: u32) -> f32 {
        assert!(parent_visits > 0, "Parent visits must be greater than 0");
        assert!(self.visits > 0, "Visits must be greater than 0");
        self.reward / self.visits as f32 + UCT_C * ((parent_visits as f32).ln() / self.visits as f32).sqrt()
    }

    fn robust_child(&self, tree: &Vec<Node>) -> Option<usize> {
        self.children
            .iter()
            .filter_map(|(_, node)| *node)
            .max_by_key(|&child| tree[child].visits)
    }

    fn uct_child(&self, tree: &Vec<Node>) -> Option<usize> {
        self.children
            .iter()
            .filter_map(|(_, node)| *node)
            .max_by(|&a, &b|
                tree[a].uct_value(self.visits)
                    .partial_cmp(&tree[b].uct_value(self.visits))
                    .unwrap())
    }
}

#[pyclass]
pub struct MCTS {
    nodes: Vec<Node>,
    root: usize,
    root_state: Game,
    time_budget: std::time::Duration,
}

#[pymethods]
impl MCTS {
    #[new]
    pub fn new(env: UTTTEnvImpl, time_budget_s: f32) -> Self {
        MCTS {
            nodes: vec![Node::new(&env.game, None, None)],
            root: 0,
            root_state: env.game,
            time_budget: std::time::Duration::from_secs_f32(time_budget_s),
        }
    }

    pub fn run(&mut self) -> u8 {
        let start = std::time::Instant::now();
        while start.elapsed() < self.time_budget {
            self.iter();
        }
        let best = self.nodes[self.root].robust_child(&self.nodes).unwrap();
        move_to_action(self.nodes[best].action.unwrap())
    }

    pub fn move_root(&mut self, action: u8) {
        let m = action_to_move(action);
        let new_root = self.nodes[self.root].children.iter()
            .find_map(|(a, node)| if *a == m { Some(node) } else { None })
            .unwrap();
        self.root_state.make_move(m);
        if let Some(new_root) = new_root {
            self.root = *new_root;
            self.nodes[self.root].parent = None;
            self.nodes[self.root].action = None;
        } else {
            self.nodes.push(Node::new(&self.root_state, None, None));
            self.root = self.nodes.len() - 1;
        }
    }

    pub fn tree_size(&self) -> usize {
        count_nodes(&self.nodes[self.root], &self.nodes)
    }
}

fn count_nodes(node: &Node, tree: &Vec<Node>) -> usize {
    1 + node.children.iter()
        .filter_map(|(_, child)|
            child.map(|child| count_nodes(&tree[child], tree)))
        .sum::<usize>()
}

fn rollout(mut state: Game) -> Option<Player> {
    let mut rng = rand::thread_rng();
    while !state.is_over() {
        let actions = state.valid_moves();
        let action = actions.choose(&mut rng).unwrap();
        state.make_move(*action);
    }
    state.winner()
}

impl MCTS {
    fn iter(&mut self) {
        let mut state = self.root_state.clone();

        // selection
        let mut leaf = self.root;
        while self.nodes[leaf].fully_expanded() && !self.nodes[leaf].is_terminal() {
            leaf = self.nodes[leaf].uct_child(&self.nodes).unwrap();
            state.make_move(self.nodes[leaf].action.unwrap());
        }

        // expansion
        if !self.nodes[leaf].is_terminal() {
            let new_id = self.nodes.len();
            let (action, child) = self.nodes[leaf].children
                .iter_mut()
                .filter_map(|(action, child)| match child {
                    Some(_) => None,
                    None => Some((action, child)),
                })
                .choose(&mut rand::thread_rng())
                .unwrap();
            *child = Some(new_id);
            let action = *action;
            state.make_move(action);
            self.nodes.push(Node::new(&state, Some(leaf), Some(action)));
            leaf = new_id;
        }

        // simulation
        let mut reward = {
            let leaf_player = state.current_player().other();
            let winner = rollout(state);
            match (leaf_player, winner) {
                (_, None) => 0.5,
                (a, Some(b)) => if a == b { 1.0 } else { 0.0 },
            }
        };

        // backpropagation
        let mut node = Some(leaf);
        while let Some(n) = node {
            self.nodes[n].visits += 1;
            self.nodes[n].reward += reward;
            node = self.nodes[n].parent;
            reward = 1.0 - reward;
        }
    }
}
