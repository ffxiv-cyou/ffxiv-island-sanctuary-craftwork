use crate::{
    data::{CraftworkInfo, IDataRepo, RecipeState},
    simulator::{simulate, simulate_multi_batch},
};

use super::{Batch, BatchWithBatch, SolverCtx, SolverSingle, SolverWithBatch};

/// Bruce Force 暴力搜索
///
/// 此方法通过对解空间做暴力遍历，得到所有情况的收益值。故其解一定正确。
pub struct BFSolver {}

impl SolverSingle for BFSolver {
    fn solve_fn<'a, T>(
        &mut self,
        ctx: &SolverCtx<'a, T>,
        demands: &[i8],
        workers: u8,
        mut cb: impl FnMut(&Batch),
    ) where
        T: IDataRepo,
    {
        let mut info = ctx.info.clone();
        if workers != 0 {
            info.workers = workers;
        }

        self.solve_sub(ctx, demands, Batch::new(), info, &mut cb);
    }
}

impl BFSolver {
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

            if current.get_time() > ctx.limit.time - 4 {
                // 当前工序结束
                cb(&current);
            } else {
                self.solve_sub(ctx, demands, current, info, cb)
            }
        });
    }
}

impl SolverWithBatch for BFSolver {
    fn solve_fn<'a, T>(
        &mut self,
        ctx: &SolverCtx<'a, T>,
        set: &[(u8, [u8; 6])],
        demands: &[i8],
        workers: u8,
        filter: impl Fn(&[Option<RecipeState>; 6]) -> bool,
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

        let mut pos = 0;
        let mut sum_time = 0;
        loop {
            // 需要改的recipe
            let recipe = &mut recipes[set.len()].1;
            {
                let mut curr_id = match recipe[pos] {
                    None => 0,
                    Some(r) => r.id() as usize,
                };

                sum_time -= ctx.repo.recipe(curr_id).craft_time;

                while curr_id < ctx.repo.recipe_len() {
                    curr_id += 1;
                    if curr_id >= ctx.repo.recipe_len() {
                        break;
                    }

                    let r = ctx.repo.recipe(curr_id);
                    // println!("recp: {}", r.id);

                    let last = match pos {
                        0 => 0,
                        _ => match recipe[pos - 1] {
                            None => 0,
                            Some(r) => r.id() as usize,
                        },
                    };
                    let last = ctx.repo.recipe(last);

                    // 时间过滤
                    if r.craft_time == 0 || sum_time + r.craft_time > ctx.limit.time as u8 {
                        continue;
                    }
                    if !ctx.limit.check(r) || last.id == r.id {
                        continue;
                    }
                    // 连击过滤
                    let batch_check = pos == 0
                        || (last.theme1 != 0
                            && (last.theme1 == r.theme1 || last.theme1 == r.theme2))
                        || (last.theme2 != 0
                            && (last.theme2 == r.theme1 || last.theme2 == r.theme2))
                        || (last.theme1 == 0 && last.theme2 == 0);
                    if !batch_check {
                        continue;
                    }

                    sum_time += r.craft_time;
                    recipe[pos] = Some(ctx.repo.state(curr_id, demands[curr_id]));
                    break;
                }

                // 当前如果在末尾，说明本循环结束，返回上一级
                if curr_id >= ctx.repo.recipe_len() {
                    if pos == 0 {
                        break; // 完全结束了
                    }
                    recipe[pos] = None;
                    pos -= 1;
                    continue;
                }
            }

            // if 当前时间结束了 -> calc
            // else 进入下一个地方
            // 快速判断是否还能插入新的步骤
            if sum_time + 4 <= ctx.limit.time as u8 && pos < 6 - 1 {
                // 还可以，继续插入！
                pos += 1;
                continue;
            }

            if !filter(recipe) {
                continue;
            }

            // 不行了，直接计算当前值
            let (result, _) = simulate_multi_batch(&ctx.info, &recipes);
            let mut b2b = BatchWithBatch::from_batch(result[result.len() - 1].1);
            for i in 0..result.len() - 1 {
                let (workers, batch) = result[i];
                b2b.value += batch.value * workers as u16;
                b2b.cost += batch.cost * workers as u16;
            }

            // println!("batch: {:?}", b2b.batch);
            cb(&b2b);
        }
    }
}
