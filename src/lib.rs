use ai::AlphaBeta;
use bits::BitBoard;
use board::{Board, State};
use pyo3::prelude::*;

pub mod ai;
pub mod bits;
pub mod board;
pub mod consts;

#[pymodule]
fn othello(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<BitBoard>()?;
    m.add_class::<Board>()?;
    m.add_class::<AlphaBeta>()?;
    m.add_class::<State>()?;
    Ok(())
}
