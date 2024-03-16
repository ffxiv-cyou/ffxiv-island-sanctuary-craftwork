use crate::data::{Favor, Favors, IDataRepo};

use super::SolverCtx;

use std::collections::BinaryHeap;

use super::super::simulator::Batch;

/// 步骤解
#[derive(Clone, Copy, Debug)]
pub struct BatchWithBatch {
    /// 当前步骤
    pub batch: Batch,
    /// 已设置步骤的值
    pub value: u16,
    /// 已设置步骤的费用
    pub cost: u16,
    /// 用于比较的总值
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
        let a = self.cmp_value as usize * 10 + self.batch.seq as usize;
        let b = other.cmp_value as usize * 10 + other.batch.seq as usize;
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
        let a = self.cmp_value as usize * 10 + self.batch.seq as usize;
        let b = other.cmp_value as usize * 10 + other.batch.seq as usize;
        b.cmp(&a)
    }
}
impl Eq for BatchWithBatch {}

/// 带已设置排班的当日求解器
pub trait SolverWithBatch {
    fn solve_fn<'a, T>(
        &mut self,
        ctx: &SolverCtx<'a, T>,
        set: &[(u8, [u8; 6])],
        demands: &[i8],
        workers: u8,
        cb: impl FnMut(&BatchWithBatch),
    ) where
        T: IDataRepo;

    /// 解最优
    ///
    /// - limit: 求解限制
    /// - set: 当前已设定的排班
    /// - demands: 需求
    /// - n: 当前排班的工房数量
    ///
    /// 返回数组，第一项为已设定排班的收益，第二项为当前排班收益
    fn solve_unordered<'a, T>(
        &mut self,
        ctx: &SolverCtx<'a, T>,
        set: &[(u8, [u8; 6])],
        demands: &[i8],
        workers: u8,
    ) -> Vec<BatchWithBatch>
    where
        T: IDataRepo,
    {
        let mut result = vec![];
        self.solve_fn(ctx, set, demands, workers, |steps| {
            result.push(*steps);
        });
        result
    }

    /// 解最优后对结果排序
    fn solve<'a, T>(
        &mut self,
        ctx: &SolverCtx<'a, T>,
        set: &[(u8, [u8; 6])],
        demands: &[i8],
        workers: u8,
    ) -> Vec<BatchWithBatch>
    where
        T: IDataRepo,
    {
        // 结果排序
        let mut heap = BinaryHeap::new();
        self.solve_fn(ctx, set, demands, workers, |item| {
            let mut item = *item;
            item.cmp_value = match ctx.limit.with_cost {
                true => {
                    ((item.batch.value - item.batch.cost) * workers as u16)
                        + (item.value - item.cost)
                }
                false => (item.batch.value) * workers as u16 + (item.value),
            };
            heap.push(item);
            if heap.len() > ctx.limit.max_result {
                heap.pop();
            }
        });
        heap.into_sorted_vec()
    }

    /// 解唯一最优
    fn solve_best<'a, T>(
        &mut self,
        ctx: &SolverCtx<'a, T>,
        set: &[(u8, [u8; 6])],
        demands: &[i8],
        workers: u8,
    ) -> BatchWithBatch
    where
        T: IDataRepo,
    {
        let mut max_val = 0;
        let mut max_batch = BatchWithBatch::from_batch(Batch::new());
        self.solve_fn(ctx, set, demands, workers, |item| {
            let val = match ctx.limit.with_cost {
                true => {
                    ((item.batch.value - item.batch.cost) * workers as u16)
                        + (item.value - item.cost)
                }
                false => (item.batch.value) * workers as u16 + (item.value),
            };
            if max_val < val {
                max_val = val;
                max_batch = *item;
            }
        });
        max_batch
    }

    /// 使用指定函数解唯一最优
    fn solve_best_fn<'a, T>(
        &mut self,
        ctx: &SolverCtx<'a, T>,
        set: &[(u8, [u8; 6])],
        demands: &[i8],
        workers: u8,
        sort_val: impl Fn(u16, &BatchWithBatch) -> u16,
    ) -> BatchWithBatch
    where
        T: IDataRepo,
    {
        let mut max_val = 0;
        let mut max_batch = BatchWithBatch::from_batch(Batch::new());
        self.solve_fn(ctx, set, demands, workers, |item| {
            let val = match ctx.limit.with_cost {
                true => {
                    ((item.batch.value - item.batch.cost) * workers as u16)
                        + (item.value - item.cost)
                }
                false => (item.batch.value) * workers as u16 + (item.value),
            };
            let val = sort_val(val, &item);
            if max_val < val {
                max_val = val;
                max_batch = *item;
            }
        });
        max_batch
    }

    /// 解猫耳小员
    fn solve_favor<'a, T>(
        &mut self,
        ctx: &SolverCtx<'a, T>,
        set: &[(u8, [u8; 6])],
        demands: &[i8],
        favors: &[Favor],
    ) -> Vec<BatchWithBatch>
    where
        T: IDataRepo,
    {
        // 结果排序
        let mut heap = BinaryHeap::new();
        self.solve_fn(ctx, set, demands, 1, |item| {
            let mut item = *item;
            let val = match ctx.limit.with_cost {
                true => {
                    (item.batch.value - item.batch.cost) + (item.value - item.cost)
                }
                false => (item.batch.value) + (item.value),
            };
            let mut fav_counter = Favors::<3>::new(favors);
            for i in 0..item.batch.seq {
                fav_counter.add(item.batch.steps[i as usize], match i == 0 {
                    true => 1,
                    false => 2,
                })
            }
            item.cmp_value = ((fav_counter.value() as u16) << 12) + val;
            heap.push(item);
            if heap.len() > ctx.limit.max_result {
                heap.pop();
            }
        });
        heap.into_sorted_vec()
    }
}
