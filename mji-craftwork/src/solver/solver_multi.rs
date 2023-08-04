use super::SolveLimit;

use std::collections::BinaryHeap;

use super::super::simulator::Batch;

pub struct BatchWithBatch {
    pub batch: Batch,
    pub value: u16,
    pub cost: u16,
    pub cmp_value: u16,
}

impl BatchWithBatch {
    pub fn from_batch(batch: Batch) -> Self {
        BatchWithBatch {
            batch,
            value: 0,
            cost: 0,
            cmp_value: 0,
        }
    }
}

impl PartialOrd for BatchWithBatch {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let a = self.cmp_value as usize * 10 + self.batch.seq;
        let b = other.cmp_value as usize * 10 + other.batch.seq;
        b.partial_cmp(&a)
    }
}

impl PartialEq for BatchWithBatch {
    fn eq(&self, other: &Self) -> bool {
        self.cmp_value == other.cmp_value && self.batch.seq == other.batch.seq
    }
}

impl Ord for BatchWithBatch {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a = self.cmp_value as usize * 10 + self.batch.seq;
        let b = other.cmp_value as usize * 10 + other.batch.seq;
        b.cmp(&a)
    }
}
impl Eq for BatchWithBatch {}

/// 每天排班求解器
pub trait SolverMulti {
    /// 解最优
    ///
    /// - limit: 求解限制
    /// - set: 当前已设定的排班
    /// - demands: 需求
    /// - n: 当前排班的工房数量
    ///
    /// 返回数组，第一项为已设定排班的收益，第二项为当前排班收益
    fn solve_unordered(
        &self,
        limit: &SolveLimit,
        set: &[(u8, [u8; 6])],
        demands: &[i8],
        workers: u8,
    ) -> Vec<BatchWithBatch>;

    /// 解最优后对结果排序
    fn solve(
        &self,
        limit: &SolveLimit,
        set: &[(u8, [u8; 6])],
        demands: &[i8],
        workers: u8,
    ) -> Vec<BatchWithBatch> {
        let ret = self.solve_unordered(limit, set, demands, workers);
        // 结果排序
        let mut heap = BinaryHeap::new();
        for mut item in ret {
            item.cmp_value = match limit.with_cost {
                true => {
                    ((item.batch.value - item.batch.cost) * workers as u16)
                        + (item.value - item.cost)
                }
                false => (item.batch.value) * workers as u16 + (item.value),
            };
            heap.push(item);
            if heap.len() > limit.max_result {
                heap.pop();
            }
        }
        heap.into_sorted_vec()
    }

    /// 解唯一最优
    fn solve_best(
        &self,
        limit: &SolveLimit,
        set: &[(u8, [u8; 6])],
        demands: &[i8],
        workers: u8,
    ) -> BatchWithBatch {
        let ret = self.solve_unordered(limit, set, demands, workers);
        let mut max_val = 0;
        let mut max_batch = BatchWithBatch::from_batch(Batch::new());
        for item in ret {
            let val = match limit.with_cost {
                true => {
                    ((item.batch.value - item.batch.cost) * workers as u16)
                        + (item.value - item.cost)
                }
                false => (item.batch.value) * workers as u16 + (item.value),
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
        set: &[(u8, [u8; 6])],
        demands: &[i8],
        workers: u8,
        sort_val: impl Fn(u16, &BatchWithBatch) -> u16,
    ) -> BatchWithBatch {
        let ret = self.solve_unordered(limit, set, demands, workers);
        let mut max_val = 0;
        let mut max_batch = BatchWithBatch::from_batch(Batch::new());
        for item in ret {
            let val = match limit.with_cost {
                true => {
                    ((item.batch.value - item.batch.cost) * workers as u16)
                        + (item.value - item.cost)
                }
                false => (item.batch.value) * workers as u16 + (item.value),
            };
            let val = sort_val(val, &item);
            if max_val < val {
                max_val = val;
                max_batch = item;
            }
        }
        max_batch
    }
}