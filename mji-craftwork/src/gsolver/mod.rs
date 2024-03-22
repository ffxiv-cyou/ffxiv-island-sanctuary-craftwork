use crate::{
    data::{Favor, IDataRepo},
    predition::DemandPattern,
    simulator::Batch,
    solver::{Batches, SolverCtx},
};

mod mild;
pub use mild::MildSolver;
mod radical;
pub use radical::RadicalSolver;
mod mild_multi;
pub use mild_multi::MildMulitSolver;
mod favor;
pub use favor::FavorSolver;

/// Global Solver 整周排班求解器
pub trait GSolver {
    fn solve<'a, T>(&mut self, ctx: &SolverCtx<'a, T>, pat: &[DemandPattern]) -> [Option<Batch>; 6]
    where
        T: IDataRepo;
}

/// Global Multi Solver 整周多工房排班求解器
pub trait GMultiSolver {
    fn solve_part<'a, T>(
        &mut self,
        ctx: &SolverCtx<'a, T>,
        pat: &[DemandPattern],
        part_id: usize,
    ) -> ([Option<Batches>; 6], u16)
    where
        T: IDataRepo;

    fn parts() -> usize;

    fn solve<'a, T>(
        &mut self,
        ctx: &SolverCtx<'a, T>,
        pat: &[DemandPattern],
    ) -> [Option<Batches>; 6]
    where
        T: IDataRepo,
    {
        let mut max_result = [None; 6];
        let mut max_value = 0;

        for id in 0..Self::parts() {
            let (result, value) = self.solve_part(ctx, pat, id);
            if value > max_value {
                max_value = value;
                max_result = result;
            }
        }
        max_result
    }
}

/// Global Favor Solver 简易猫票求解器
pub trait GFavorSolver {
    fn solve<'a, T>(&mut self, ctx: &mut SolverCtx<'a, T>, pat: &[DemandPattern], favors: &[Favor]) -> [Option<Batches>; 6]
    where
        T: IDataRepo;
}

pub fn print_week_result(arr: &[Option<Batches>]) {
    for i in 0..arr.len() {
        match &arr[i] {
            Some(batch) => println!("D{}: {}", i + 2, batch),
            None => println!("D{}: Rest", i + 2),
        }
    }
}
