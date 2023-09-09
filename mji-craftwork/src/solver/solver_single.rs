use super::SolverCtx;
use crate::data::IDataRepo;

use std::collections::BinaryHeap;

use super::super::simulator::Batch;

/// 每天排班求解器，单一工坊
pub trait SolverSingle {
    fn solve_fn<'a, T>(
        &mut self,
        ctx: &SolverCtx<'a, T>,
        demands: &[i8],
        workers: u8,
        cb: impl FnMut(&Batch),
    ) where
        T: IDataRepo;

    /// 解最优
    fn solve_unordered<'a, T>(
        &mut self,
        ctx: &SolverCtx<'a, T>,
        demands: &[i8],
        workers: u8,
    ) -> Vec<Batch>
    where
        T: IDataRepo,
    {
        let mut result = vec![];
        self.solve_fn(ctx, demands, workers, |steps| {
            result.push(*steps);
        });
        result
    }

    /// 解最优后对结果排序
    fn solve<'a, T>(&mut self, ctx: &SolverCtx<'a, T>, demands: &[i8], workers: u8) -> Vec<Batch>
    where
        T: IDataRepo,
    {
        // 结果排序
        let mut heap = BinaryHeap::new();
        self.solve_fn(ctx, demands, workers, |item| {
            let mut item = *item;
            item.set_cmp_value(ctx.limit.with_cost);
            heap.push(item);
            if heap.len() > ctx.limit.max_result {
                heap.pop();
            }
        });
        heap.into_sorted_vec()
    }

    /// 解唯一最优
    fn solve_best<'a, T>(&mut self, ctx: &SolverCtx<'a, T>, demands: &[i8], workers: u8) -> Batch
    where
        T: IDataRepo,
    {
        let mut max_val = 0;
        let mut max_batch = Batch::new();
        self.solve_fn(ctx, demands, workers, |item| {
            let val = match ctx.limit.with_cost {
                true => item.value - item.cost,
                false => item.value,
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
        demands: &[i8],
        workers: u8,
        sort_val: impl Fn(u16, &Batch) -> u16,
    ) -> Batch
    where
        T: IDataRepo,
    {
        let mut max_val = 0;
        let mut max_batch = Batch::new();
        self.solve_fn(ctx, demands, workers, |item| {
            let val = match ctx.limit.with_cost {
                true => item.value - item.cost,
                false => item.value,
            };
            let val = sort_val(val, &item);
            if max_val < val {
                max_val = val;
                max_batch = *item;
            }
        });
        max_batch
    }
}
