use crate::{
    data::{CraftworkInfo, IDataRepo},
    simulator::{simulate, simulate_multi_batch},
};

use super::{Batch, BatchWithBatch, SolveLimit, Solver, SolverMulti};

/// Bruce Force 暴力搜索
///
/// 此方法通过对解空间做暴力遍历，得到所有情况的收益值。故其解一定正确。
pub struct BFSolver<'a, T>
where
    T: IDataRepo,
{
    info: CraftworkInfo,
    data: &'a T,
}

impl<'a, T> Solver for BFSolver<'a, T>
where
    T: IDataRepo,
{
    fn solve_unordered(&self, limit: &SolveLimit, demands: &[i8]) -> Vec<Batch> {
        let mut ret: Vec<Batch> = vec![];
        self.solve_sub(limit, demands, &mut ret, Batch::new(), self.info.clone());
        ret
    }

    fn update_info(&mut self, info: CraftworkInfo) {
        self.info = info
    }
}

impl<'a, T> BFSolver<'a, T>
where
    T: IDataRepo,
{
    /// 新建一个暴力搜索
    pub fn new(data: &'a T, info: CraftworkInfo) -> Self {
        Self {
            info: info,
            data: data,
        }
    }

    fn solve_sub(
        &self,
        limit: &SolveLimit,
        demands: &[i8],
        vec: &mut Vec<Batch>,
        current: Batch,
        mut info: CraftworkInfo,
    ) {
        let last = current.last();
        let first_batch = last == 0;
        let last = self.data.recipe(last as usize);
        let remain = limit.time - current.get_time();

        // 连击开始后即增加干劲
        if !first_batch {
            info = info.next()
        }

        self.data.foreach(|r| {
            if r.craft_time == 0 || remain < r.craft_time as u16 {
                return;
            }
            if !limit.check(r) || last.id == r.id {
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
            let s = self.data.state(id, demands[id]);
            let demand_sub = current.get_produce(s.id()) * info.workers;
            let val = simulate(&info, &s, demand_sub);
            let current = current.push(r.id, val, r.cost, r.craft_time as u16);

            if current.get_time() > limit.time - 4 {
                // 当前工序结束
                vec.push(current)
            } else {
                self.solve_sub(limit, demands, vec, current, info)
            }
        });
    }
}

impl<'a, T> SolverMulti for BFSolver<'a, T>
where
    T: IDataRepo,
{
    fn solve_unordered(
        &self,
        limit: &SolveLimit,
        set: &[(u8, [u8; 6])],
        demands: &[i8],
        workers: u8,
    ) -> Vec<BatchWithBatch> {
        let mut ret = vec![];

        // 准备计算状态
        let mut recipes = vec![];
        for (num, seq) in set {
            let mut arr = [None; 6];
            for i in 0..seq.len() {
                let id = seq[i] as usize;
                arr[i] = match id {
                    0 => None,
                    _ => Some(self.data.state(id, demands[id])),
                }
            }
            recipes.push((num.clone(), arr));
        }
        recipes.push((workers, [None; 6]));

        let mut pos = 0;
        let mut sum_time = 0;
        loop {
            {
                // 需要改的recipe
                let recipe = &mut recipes[set.len()].1;
                let mut curr_id = match recipe[pos] {
                    None => 0,
                    Some(r) => r.id() as usize,
                };

                sum_time -= self.data.recipe(curr_id).craft_time;

                while curr_id < self.data.recipe_len() {
                    curr_id += 1;
                    if curr_id >= self.data.recipe_len() {
                        break;
                    }

                    let r = self.data.recipe(curr_id);
                    // println!("recp: {}", r.id);

                    let last = match pos {
                        0 => 0,
                        _ => match recipe[pos - 1] {
                            None => 0,
                            Some(r) => r.id() as usize,
                        },
                    };
                    let last = self.data.recipe(last);

                    // 时间过滤
                    if r.craft_time == 0 || sum_time + r.craft_time > limit.time as u8 {
                        continue;
                    }
                    if !limit.check(r) || last.id == r.id {
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
                    recipe[pos] = Some(self.data.state(curr_id, demands[curr_id]));
                    break;
                }

                // 当前如果在末尾，说明本循环结束，返回上一级
                if curr_id >= self.data.recipe_len() {
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
            if sum_time + 4 <= limit.time as u8 && pos < 6 - 1 {
                // 还可以，继续插入！
                pos += 1;
                continue;
            }

            // 不行了，直接计算当前值
            let (result, _) = simulate_multi_batch(&self.info, &recipes);
            let mut b2b = BatchWithBatch::from_batch(result[result.len() - 1].1);
            for i in 0..result.len() - 1 {
                let (workers, batch) = result[i];
                b2b.value += batch.value * workers as u16;
                b2b.cost += batch.cost * workers as u16;
            }

            // println!("batch: {:?}", b2b.batch);
            ret.push(b2b);
        }
        ret
    }
}
