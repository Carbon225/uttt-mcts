use rand::seq::{SliceRandom, IteratorRandom};
use pyo3::prelude::*;
use crate::env::{UTTTEnvImpl, action_to_move, move_to_action};
use crate::game::{Game, Player, Move};

const UCT_C: f32 = 1.41;

struct Node {
    state: Game,
    visits: u32,
    reward: f32,
    parent: Option<usize>,
    action: Option<Move>,
    children: Vec<(Move, Option<usize>)>,
}

impl Node {
    fn new(state: Game, parent: Option<usize>, action: Option<Move>) -> Self {
        Self {
            parent,
            children: state.valid_moves().into_iter().map(|action| (action, None)).collect(),
            state,
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

    fn best_child(&self, tree: &Vec<Node>) -> Option<usize> {
        self.children
            .iter()
            .filter_map(|(_, node)| *node)
            .max_by(|a, b|
                tree[*a].uct_value(self.visits)
                    .partial_cmp(&tree[*b].uct_value(self.visits))
                    .unwrap())
    }
}

#[pyclass]
pub struct MCTS {
    nodes: Vec<Node>,
    root: usize,
    iterations: u32,
}

#[pymethods]
impl MCTS {
    #[new]
    pub fn new(env: UTTTEnvImpl, iterations: u32) -> Self {
        MCTS {
            nodes: vec![Node::new(env.game, None, None)],
            root: 0,
            iterations,
        }
    }

    pub fn run(&mut self) -> u8 {
        for _ in 0..self.iterations {
            self.iter();
        }
        let best = self.nodes[self.root].best_child(&self.nodes).unwrap();
        move_to_action(self.nodes[best].action.unwrap())
    }

    pub fn move_root(&mut self, action: u8) {
        let m = action_to_move(action);
        let new_root = self.nodes[self.root].children.iter()
            .find_map(|(a, node)| if *a == m { Some(node) } else { None })
            .unwrap()
            .unwrap_or_else(|| {
                let mut state = self.nodes[self.root].state.clone();
                state.make_move(m);
                let new_node = Node::new(state, None, None);
                self.nodes.push(new_node);
                self.nodes.len() - 1
            });
        self.root = new_root;
        self.nodes[self.root].parent = None;
        self.nodes[self.root].action = None;
    }

    pub fn tree_size(&self) -> usize {
        self.nodes.len()
    }
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
        // selection
        let mut leaf = self.root;
        while self.nodes[leaf].fully_expanded() && !self.nodes[leaf].is_terminal() {
            leaf = self.nodes[leaf].best_child(&self.nodes).unwrap();
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
            let new_node = Node::new({
                let mut state = self.nodes[leaf].state.clone();
                state.make_move(action);
                state
            }, Some(leaf), Some(action));
            self.nodes.push(new_node);
            leaf = new_id;
        }

        // simulation
        let winner = rollout(self.nodes[leaf].state.clone());
        let reward_x = match winner {
            Some(Player::X) => 1.0,
            Some(Player::O) => 0.0,
            None => 0.5,
        };
        let reward_o = match winner {
            Some(Player::X) => 0.0,
            Some(Player::O) => 1.0,
            None => 0.5,
        };

        // backpropagation
        let mut node = Some(leaf);
        while let Some(n) = node {
            self.nodes[n].visits += 1;
            self.nodes[n].reward += match self.nodes[n].state.current_player().other() {
                Player::X => reward_x,
                Player::O => reward_o,
            };
            node = self.nodes[n].parent;
        }
    }
}
