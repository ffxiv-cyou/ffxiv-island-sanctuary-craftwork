mod bruce_force;
mod simplify;
mod simplify_adv;
mod solver_multi;
mod solver_single;
mod solver_with_batch;

use super::data::{CraftworkInfo, IDataRepo, Recipe};
use super::simulator::Batch;

pub use bruce_force::BFSolver;
pub use simplify::SimplifySolver;
pub use simplify_adv::AdvancedSimplifySolver;
pub use solver_multi::{Batches, SolverDual};
pub use solver_single::SolverSingle;
pub use solver_with_batch::{BatchWithBatch, SolverWithBatch};

/// 求解器限制
#[derive(Debug, Clone, Copy)]
pub struct SolveLimit<'a> {
    /// 配方最高等级
    level: u8,
    /// 禁用配方
    ban_list: &'a [u8],
    /// 结果数量
    pub max_result: usize,
    /// 是否考虑成本
    pub with_cost: bool,
    /// 工序耗时限制
    pub time: u8,
}

impl<'a> SolveLimit<'a> {
    pub fn new(level: u8, ban_list: &'a [u8], time: u8, with_cost: bool) -> SolveLimit {
        Self {
            level,
            ban_list,
            max_result: 100,
            with_cost,
            time,
        }
    }

    pub fn check(&self, recipe: &Recipe) -> bool {
        return recipe.level <= self.level && !self.ban_list.contains(&recipe.id);
    }
}

/// 求解器的上下文
pub struct SolverCtx<'a, T>
where
    T: IDataRepo,
{
    pub repo: &'a T,
    pub info: CraftworkInfo,
    pub limit: SolveLimit<'a>,
}

impl<'a, T> SolverCtx<'a, T>
where
    T: IDataRepo,
{
    pub fn new(repo: &'a T, info: CraftworkInfo, limit: SolveLimit<'a>) -> Self {
        Self { repo, info, limit }
    }
}
