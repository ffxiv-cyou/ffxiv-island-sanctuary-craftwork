use crate::{solver::SolveLimit, simulator::Batch, predition::DemandPattern};

mod mild;
pub use mild::MildSolver;
mod radical;
pub use radical::RadicalSolver;


/// Global Solver 整周排班求解器
pub trait GSolver {
    fn solve(&self, limit: &SolveLimit, pat: &[DemandPattern]) -> [Option<Batch>; 6];
}
