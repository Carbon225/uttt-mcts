use rand::seq::{SliceRandom, IteratorRandom};
use pyo3::prelude::*;
use crate::env::{UTTTEnvImpl, action_to_move, move_to_action};
use crate::game::{Game, Player, Move};

const UCT_C: f32 = 1.41;

struct Node {
    state: Game,
    visits: u32,
    reward: f32,
    children: Vec<(Move, Option<Node>)>,
}

impl Node {
    fn new(state: Game) -> Self {
        Self {
            children: state.valid_moves().into_iter().map(|action| (action, None)).collect(),
            state,
            visits: 0,
            reward: 0.0,
        }
    }

    fn fully_expanded(&self) -> bool {
        self.children.iter().all(|(_, child)| child.is_some())
    }

    fn uct_value(&self, parent_visits: u32) -> f32 {
        assert!(parent_visits > 0, "Parent visits must be greater than 0");
        assert!(self.visits > 0, "Visits must be greater than 0");
        self.reward / self.visits as f32 + UCT_C * ((parent_visits as f32).ln() / self.visits as f32).sqrt()
    }

    fn best_child(&self) -> Option<usize> {
        self.children
            .iter()
            .filter_map(|(action, node)| match node {
                Some(node) => Some((action, node)),
                None => None,
            })
            .enumerate()
            .max_by(|(_, (_, a)), (_, (_, b))|
                a.uct_value(self.visits)
                .partial_cmp(&b.uct_value(self.visits))
                .unwrap())
            .map(|(i, _)| i)
    }
}

#[pyclass]
pub struct MCTS {
    root: Node,
    iterations: u32,
}

#[pymethods]
impl MCTS {
    #[new]
    pub fn new(env: UTTTEnvImpl, iterations: u32) -> Self {
        MCTS {
            root: Node::new(env.game),
            iterations,
        }
    }

    pub fn run(&mut self) -> u8 {
        for _ in 0..self.iterations {
            self.iter();
        }
        let best = self.root.best_child().unwrap();
        move_to_action(self.root.children[best].0)
    }

    pub fn move_root(&mut self, action: u8) {
        let m = action_to_move(action);
        let new_root = self.root.children.iter_mut()
            .find_map(|(a, node)| if *a == m { Some(node) } else { None })
            .unwrap()
            .take()
            .unwrap_or_else(|| {
                let mut state = self.root.state.clone();
                state.make_move(m);
                Node::new(state)
            });
        self.root = new_root;
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
        let mut leaf = &mut self.root;
        let mut trajectory = vec![];
        while leaf.fully_expanded() && !leaf.state.is_over() {
            let best = leaf.best_child().unwrap();
            leaf = leaf.children[best].1.as_mut().unwrap();
            trajectory.push(best);
        }

        // expansion
        if !leaf.state.is_over() {
            let new_node = leaf.children
                .iter()
                .enumerate()
                .filter_map(|(i, (_, child))| match child {
                    Some(_) => None,
                    None => i.into(),
                })
                .choose(&mut rand::thread_rng())
                .unwrap();
            trajectory.push(new_node);
            leaf.children[new_node].1 = Some(Node::new({
                let mut state = leaf.state.clone();
                state.make_move(leaf.children[new_node].0);
                state
            }));
            leaf = leaf.children[new_node].1.as_mut().unwrap();
        }

        // simulation
        let winner = rollout(leaf.state.clone());
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
        let mut node = &mut self.root;
        node.visits += 1;
        node.reward += match node.state.current_player().other() {
            Player::X => reward_x,
            Player::O => reward_o,
        };
        for i in trajectory {
            node = node.children[i].1.as_mut().unwrap();
            node.visits += 1;
            node.reward += match node.state.current_player().other() {
                Player::X => reward_x,
                Player::O => reward_o,
            };
        }
    }
}
