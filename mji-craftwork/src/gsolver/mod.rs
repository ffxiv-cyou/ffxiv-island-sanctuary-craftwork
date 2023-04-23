use crate::{solver::SolveLimit, simulator::Batch, predition::DemandPattern};

mod mild;
pub use mild::MildSolver;

/// Global Solver 解全局最优
pub trait GSolver {
    fn solve(&self, limit: &SolveLimit, pat: &[DemandPattern]) -> [Option<Batch>; 6];
}
