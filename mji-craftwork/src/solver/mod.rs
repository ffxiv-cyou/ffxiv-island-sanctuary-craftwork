mod bruce_force;
mod simplify;

use std::collections::BinaryHeap;

use super::simulator::Batch;
pub use bruce_force::BFSolver;
pub use simplify::SimplifySolver;

use crate::data::{CraftworkInfo, Recipe};

/// 每天排班求解器
pub trait Solver {
    /// 解最优
    fn solve_unordered(&self, limit: &SolveLimit, demands: &[i8]) -> Vec<Batch>;

    /// 解最优后对结果排序
    fn solve(&self, limit: &SolveLimit, demands: &[i8]) -> Vec<Batch> {
        let ret = self.solve_unordered(limit, demands);
        // 结果排序
        let mut heap = BinaryHeap::new();
        for mut item in ret {
            item.set_cmp_value(limit.with_cost);
            heap.push(item);
            if heap.len() >= limit.max_result {
                heap.pop();
            }
        }
        heap.into_sorted_vec()
    }

    /// 解唯一最优
    fn solve_best(&self, limit: &SolveLimit, demands: &[i8]) -> Batch {
        let ret = self.solve_unordered(limit, demands);
        let mut max_val = 0;
        let mut max_batch = Batch::new();
        for item in ret {
            let val = match limit.with_cost {
                true => item.value - item.cost,
                false => item.value,
            };
            if max_val < val {
                max_val = val;
                max_batch = item;
            }
        }
        max_batch
    }

    /// 使用指定函数解唯一最优
    fn solve_best_fn(&self, limit: &SolveLimit, demands: &[i8], sort_val: impl Fn(u16,&Batch)->u16) ->Batch {
        let ret = self.solve_unordered(limit, demands);
        let mut max_val = 0;
        let mut max_batch = Batch::new();
        for item in ret {
            let val = match limit.with_cost {
                true => item.value - item.cost,
                false => item.value,
            };
            let val = sort_val(val, &item);
            if max_val < val {
                max_val = val;
                max_batch = item;
            }
        }
        max_batch
    }

    fn update_info(&mut self, info: CraftworkInfo);
}

/// 求解器限制
pub struct SolveLimit<'a> {
    /// 配方最高等级
    level: u8,
    /// 禁用配方
    ban_list: &'a [u16],
    /// 结果数量
    pub max_result: usize,
    /// 是否考虑成本
    pub with_cost: bool,
    /// 工序耗时限制
    pub time: u16,
}

impl<'a> SolveLimit<'a> {
    pub fn new(level: u8, ban_list: &'a [u16], time: u16, with_cost: bool) -> SolveLimit {
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
