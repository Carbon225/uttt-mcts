use pyo3::prelude::*;
use rand::seq::SliceRandom;
use crate::env::UTTTEnvImpl;

#[pyclass]
pub struct PMCTS {
    time_budget: std::time::Duration,
}

#[pymethods]
impl PMCTS {
    #[new]
    pub fn new(time_budget_s: f32) -> Self {
        PMCTS { time_budget: std::time::Duration::from_secs_f32(time_budget_s) }
    }

    pub fn run(&self, game: UTTTEnvImpl) -> u8 {
        let actions = game.valid_actions();
        let mut rewards = vec![0.0; actions.len()];
        let start = std::time::Instant::now();
        while start.elapsed() < self.time_budget {
            for (i, action) in actions.iter().enumerate() {
                let mut game = game.clone();
                game.step(*action);
                rewards[i] += self.rollout(game);
            }
        }
        *actions
            .iter()
            .zip(rewards)
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap().0
    }
}

impl PMCTS {
    fn rollout(&self, mut game: UTTTEnvImpl) -> f32 {
        let mut rng = rand::thread_rng();
        let enemy = game.current_player();
        while !game.done() {
            let actions = game.valid_actions();
            let action = actions.choose(&mut rng).unwrap();
            game.step(*action);
            
        }
        match enemy {
            0 => return -game.reward(),
            _ => return game.reward(),
        }
    }
}
