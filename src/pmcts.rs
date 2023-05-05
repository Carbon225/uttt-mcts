use pyo3::prelude::*;
use rayon::prelude::*;
use rand::seq::SliceRandom;
use crate::env::UTTTEnvImpl;

#[pyclass]
pub struct PMCTS {
    simulations: u32,
}

#[pymethods]
impl PMCTS {
    #[new]
    pub fn new(simulations: u32) -> Self {
        PMCTS { simulations }
    }

    pub fn run(&self, game: UTTTEnvImpl) -> u8 {
        let actions = game.valid_actions();
        let n = self.simulations / (actions.len() as u32);
        let rewards = actions
            .iter()
            .map(|action| {
                let mut game = game.clone();
                game.step(*action);
                (0..n)
                    .into_par_iter()
                    .map(|_| self.rollout(game.clone()))
                    .sum::<f32>()
            });
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
