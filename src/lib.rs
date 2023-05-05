mod game;
mod env;
mod mcts;
mod pmcts;

use pyo3::prelude::*;
use env::UTTTEnvImpl;
use mcts::MCTS;
use pmcts::PMCTS;

#[pymodule]
fn uttt_rl(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<UTTTEnvImpl>()?;
    m.add_class::<MCTS>()?;
    m.add_class::<PMCTS>()?;
    Ok(())
}
