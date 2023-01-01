mod bruce_force;

use super::simulator::Batch;
pub use bruce_force::BFSolver;

use crate::data::{CraftworkInfo, Recipe};

pub trait Solver {
    /// 解最优
    fn solve(&self, limit: &SolveLimit) -> Vec<Batch>;
    fn update_info(&mut self, info: CraftworkInfo);
}

/// 求解器限制
pub struct SolveLimit<'a> {
    /// 配方最高等级
    level: u8,
    /// 禁用配方
    ban_list: &'a [u16],
    /// 结果数量
    pub max_result: usize
}

impl<'a> SolveLimit<'a> {
    pub fn new(level: u8, ban_list: &'a [u16]) -> SolveLimit {
        Self { level, ban_list, max_result: 100 }
    }

    pub fn check(&self, recipe: &Recipe) -> bool {
        return recipe.level <= self.level && !self.ban_list.contains(&recipe.id)
    }
}