use super::SolveLimit;
use crate::data::CraftworkInfo;

use std::collections::BinaryHeap;

use super::super::simulator::Batch;

/// 每天排班求解器，单一工坊
pub trait SolverSingle {
    /// 解最优
    fn solve_unordered(&self, limit: &SolveLimit, demands: &[i8], workers: u8) -> Vec<Batch>;

    /// 解最优后对结果排序
    fn solve(&self, limit: &SolveLimit, demands: &[i8], workers: u8) -> Vec<Batch> {
        let ret = self.solve_unordered(limit, demands, workers);
        // 结果排序
        let mut heap = BinaryHeap::new();
        for mut item in ret {
            item.set_cmp_value(limit.with_cost);
            heap.push(item);
            if heap.len() > limit.max_result {
                heap.pop();
            }
        }
        heap.into_sorted_vec()
    }

    /// 解唯一最优
    fn solve_best(&self, limit: &SolveLimit, demands: &[i8], workers: u8) -> Batch {
        let ret = self.solve_unordered(limit, demands, workers);
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
    fn solve_best_fn(
        &self,
        limit: &SolveLimit,
        demands: &[i8],
        workers: u8,
        sort_val: impl Fn(u16, &Batch) -> u16,
    ) -> Batch {
        let ret = self.solve_unordered(limit, demands, workers);
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
