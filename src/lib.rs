use ai::AlphaBetaBot;
use bits::BitBoard;
use game::{Game, State};
use pyo3::prelude::*;

pub mod ai;
pub mod bits;
pub mod consts;
pub mod game;

#[pymodule]
fn _othello(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<BitBoard>()?;
    m.add_class::<Game>()?;
    m.add_class::<AlphaBetaBot>()?;
    m.add_class::<State>()?;
    Ok(())
}
