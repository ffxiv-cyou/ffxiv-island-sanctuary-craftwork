use crate::{
    predition::DemandPattern,
    simulator::Batch,
    solver::{Batches, SolveLimit},
};

mod mild;
pub use mild::MildSolver;
mod radical;
pub use radical::RadicalSolver;
mod mild_multi;
pub use mild_multi::MildMulitSolver;

/// Global Solver 整周排班求解器
pub trait GSolver {
    fn solve(&self, limit: &SolveLimit, pat: &[DemandPattern]) -> [Option<Batch>; 6];
}

/// Global Multi Solver 整周多工房排班求解器
pub trait GMultiSolver {
    fn solve_part(
        &mut self,
        limit: &SolveLimit,
        pat: &[DemandPattern],
        part_id: usize,
    ) -> ([Option<Batches>; 6], u16);
    
    fn parts() -> usize;

    fn solve(&mut self, limit: &SolveLimit, pat: &[DemandPattern]) -> [Option<Batches>; 6] {
        let mut max_result = [None; 6];
        let mut max_value = 0;

        for id in 0..Self::parts() {
            let (result, value) = self.solve_part(limit, pat, id);
            if value > max_value {
                max_value = value;
                max_result = result;
            }
        }
        max_result
    }
}

pub fn print_week_result(arr: &[Option<Batches>]) {
    for i in 0..arr.len() {
        match &arr[i] {
            Some(batch) => println!("D{}: {}", i + 2, batch),
            None => println!("D{}: Rest", i + 2),
        }
    }
}
