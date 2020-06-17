//! Solvers for systems of linear equations (linear systems).

// TODO @Daniel: keep all solvers even though only sparse_cholesky is used? If yes, take care of dead_code compiler warnings

#![allow(non_snake_case)]

use nalgebra::{DMatrix, DVector};

pub mod cholesky;
pub mod lu;
pub mod sparse_cholesky;

/// Trait which all solvers should implement.
pub trait Solver {
    /// Solves the linear system defined by H*x = b.
    /// H is expected column-by-column.
    fn solve(H: DMatrix<f64>, b: &DVector<f64>) -> Result<Vec<f64>, String>;
}
