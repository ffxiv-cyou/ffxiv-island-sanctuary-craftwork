use super::SolveLimit;
use crate::data::CraftworkInfo;

use std::collections::BinaryHeap;

use super::super::simulator::Batch;

/// 多序列工坊值
#[derive(Debug, Clone, Copy)]
pub struct Batches {
    pub batches: [(u8, Batch); 2],
    pub value: u16,
    pub cost: u16,
    pub cmp_value: u16,
}

impl Batches {
    pub fn new() -> Self {
        Self {
            batches: [(0, Batch::new()), (0, Batch::new())],
            value: 0,
            cost: 0,
            cmp_value: 0,
        }
    }
    pub fn from_batch(batches: &[(u8, Batch)]) -> Self {
        let mut value = 0;
        let mut cost = 0;
        for b in batches {
            value += b.0 as u16 * b.1.value;
            cost += b.0 as u16 * b.1.cost;
        }
        Batches {
            batches: [batches[0], batches[1]],
            value,
            cost,
            cmp_value: 0,
        }
    }
}
impl PartialOrd for Batches {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.cmp_value.partial_cmp(&self.cmp_value)
    }
}
impl PartialEq for Batches {
    fn eq(&self, other: &Self) -> bool {
        self.cmp_value == other.cmp_value
    }
}
impl Eq for Batches {}

impl Ord for Batches {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cmp_value.cmp(&self.cmp_value)
    }
}

/// 单日工坊多序列求解器
pub trait SolverDual {
    /// 解最优
    ///
    /// - limit: 求解限制
    /// - demands: 需求
    /// - workers: 工坊数量
    fn solve_unordered(&self, limit: &SolveLimit, demands: &[i8], workers: u8) -> Vec<Batches>;

    fn solve(&self, limit: &SolveLimit, demands: &[i8], workers: u8) -> Vec<Batches> {
        let ret = self.solve_unordered(limit, demands, workers);
        let mut heap = BinaryHeap::new();
        for mut item in ret {
            item.cmp_value = match limit.with_cost {
                true => item.value - item.cost,
                false => item.value,
            };
            heap.push(item);
            if heap.len() > limit.max_result {
                heap.pop();
            }
        }
        heap.into_sorted_vec()
    }

    /// 解唯一最优
    fn solve_best(&self, limit: &SolveLimit, demands: &[i8], workers: u8) -> Batches {
        let ret = self.solve_unordered(limit, demands, workers);
        let mut max_val = 0;
        let mut max_batch = Batches::new();
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
}
