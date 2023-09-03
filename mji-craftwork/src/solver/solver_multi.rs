use super::SolveLimit;

use core::fmt;
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
    pub fn set_result(&mut self, batches: &[(u8, Batch)]) {
        self.batches[0] = batches[0];
        self.batches[1] = batches[1];

        let mut value = 0;
        let mut cost = 0;
        for b in batches {
            value += b.0 as u16 * b.1.value;
            cost += b.0 as u16 * b.1.cost;
        }
        self.value = value;
        self.cost = cost;
    }
    /// 干劲增加量
    pub fn tension_add(&self) -> u8 {
        let mut tension = 0;
        for (worker, batch) in &self.batches {
            if *worker == 0 {
                continue;
            }
            tension += worker * (batch.seq - 1);
        }
        tension
    }

    /// 需求变动量
    pub fn produce_add(&self, demands: &mut [i8]) {
        for (worker, batch) in &self.batches {
            batch.demand_sub(demands, *worker as i8);
        }
    }

    /// 需求变动量
    pub fn produce_sub(&self, demands: &mut [i8]) {
        for (worker, batch) in &self.batches {
            batch.demand_sub(demands, *worker as i8 * -1);
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
impl fmt::Display for Batches {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {}x {:?}, {}x {:?}",
            match self.cmp_value {
                0 => self.value,
                _ => self.cmp_value,
            },
            self.batches[0].0,
            self.batches[0].1.steps,
            self.batches[1].0,
            self.batches[1].1.steps
        )
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

    /// 使用指定函数解唯一最优
    fn solve_best_fn(
        &self,
        limit: &SolveLimit,
        demands: &[i8],
        workers: u8,
        sort_val: impl Fn(u16, &Batches) -> u16,
    ) -> Batches {
        let ret = self.solve_unordered(limit, demands, workers);
        let mut max_val = 0;
        let mut max_batch = Batches::new();
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
}
