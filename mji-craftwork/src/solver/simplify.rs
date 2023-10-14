use std::env::consts::FAMILY;

use crate::{
    data::{CraftworkInfo, IDataRepo},
    simulator::{simulate, simulate_multi_batch},
};

use super::{Batch, BatchWithBatch, SolverCtx, SolverSingle, SolverWithBatch};

/// Simplify 简易剪枝
///
/// 此方法在暴力求解的过程中，动态对部分可能解做剪枝。
///
/// 当一个配方大于等于12小时后，计算其每小时收益，并与最高收益对比。
/// 若其收益小于最高收益的69%，则将其剪枝。
///
/// 此方法不保证所有解一定正确，仅能尽量保证最高几个的解是正确的。
///
/// 69%是通过实测得出的，对于其他配方可能不适用。
pub struct SimplifySolver {}

impl SolverSingle for SimplifySolver {
    fn solve_fn<'a, T>(
        &mut self,
        ctx: &SolverCtx<'a, T>,
        demands: &[i8],
        workers: u8,
        mut cb: impl FnMut(&Batch),
    ) where
        T: IDataRepo,
    {
        let mut avg = 0;
        let mut info = ctx.info.clone();
        if workers != 0 {
            info.workers = workers;
        }

        self.solve_sub(ctx, demands, Batch::new(), info, &mut avg, &mut cb);
    }
}

impl SolverWithBatch for SimplifySolver {
    fn solve_fn<'a, T>(
        &mut self,
        ctx: &SolverCtx<'a, T>,
        set: &[(u8, [u8; 6])],
        demands: &[i8],
        workers: u8,
        mut cb: impl FnMut(&BatchWithBatch),
    ) where
        T: IDataRepo,
    {
        // 准备计算状态
        let mut recipes = vec![];
        for (num, seq) in set {
            let mut arr = [None; 6];
            for i in 0..seq.len() {
                let id = seq[i] as usize;
                arr[i] = match id {
                    0 => None,
                    _ => Some(ctx.repo.state(id, demands[id])),
                }
            }
            recipes.push((num.clone(), arr));
        }
        recipes.push((workers, [None; 6]));

        let mut avg = 0;
        let mut info = ctx.info.clone();
        if workers != 0 {
            info.workers = workers;
        }

        self.solve_sub(
            ctx,
            demands,
            Batch::new(),
            info,
            &mut avg,
            &mut |item: &Batch| {
                for i in 0..item.steps.len() {
                    let id = item.steps[i] as usize;
                    recipes[set.len()].1[i] = match i >= item.seq as usize {
                        true => None,
                        false => Some(ctx.repo.state(id, demands[id])),
                    }
                }
                let (result, _) = simulate_multi_batch(&ctx.info, &recipes);
                let mut b2b = BatchWithBatch::from_batch(result[result.len() - 1].1);
                for i in 0..result.len() - 1 {
                    let (workers, batch) = result[i];
                    b2b.value += batch.value * workers as u16;
                    b2b.cost += batch.cost * workers as u16;
                }
    
                cb(&b2b);
            },
        );
    }
}

impl SimplifySolver {
    /// 新建一个暴力搜索
    pub fn new() -> Self {
        Self {}
    }

    fn solve_sub<'a, T>(
        &mut self,
        ctx: &SolverCtx<'a, T>,
        demands: &[i8],
        current: Batch,
        mut info: CraftworkInfo,
        max: &mut u16,
        cb: &mut impl FnMut(&Batch),
    ) where
        T: IDataRepo,
    {
        let last = current.last();
        let first_batch = last == 0;
        let last = ctx.repo.recipe(last as usize);
        let remain = ctx.limit.time - current.get_time();

        // 连击开始后即增加干劲
        if !first_batch {
            info = info.next()
        }

        ctx.repo.foreach(|r| {
            if r.craft_time == 0 || remain < r.craft_time {
                return;
            }
            if !ctx.limit.check(r) || last.id == r.id {
                return;
            }
            // 连击过滤
            let batch_check = first_batch
                || (last.theme1 != 0 && (last.theme1 == r.theme1 || last.theme1 == r.theme2))
                || (last.theme2 != 0 && (last.theme2 == r.theme1 || last.theme2 == r.theme2))
                || (last.theme1 == 0 && last.theme2 == 0);
            if !batch_check {
                return;
            }

            let id = r.id as usize;
            let s = ctx.repo.state(id, demands[id]);
            let demand_sub = current.get_produce(s.id()) * info.workers;
            let val = simulate(&info, &s, demand_sub);
            let current = current.push(r.id, val, r.cost, r.craft_time);

            if current.get_time() >= 12 {
                let avg = current.value / current.get_time() as u16;

                // 平均收益不达标，剪枝
                if avg < (*max as f32 * 0.69) as u16 {
                    return;
                }
            }

            if current.get_time() > ctx.limit.time - 4 {
                // 当前工序结束
                cb(&current);

                let avg = current.value / ctx.limit.time as u16;
                if avg > *max {
                    *max = avg;
                }
            } else {
                self.solve_sub(ctx, demands, current, info, max, cb)
            }
        });
    }
}
