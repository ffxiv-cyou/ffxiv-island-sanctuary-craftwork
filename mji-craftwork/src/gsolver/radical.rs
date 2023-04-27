use crate::{
    data::{CraftworkInfo, IDataRepo, RecipeState},
    predition::{get_demands, DemandPattern},
    simulator::{simulate, Batch, simulate_batch_seq},
    solver::{BFSolver, Solver},
};

use super::GSolver;

pub struct RadicalSolver<'a, T>
where
    T: IDataRepo,
{
    info: CraftworkInfo,
    data: &'a T
}

impl<'a, T> GSolver for RadicalSolver<'a, T>
where
    T: IDataRepo,
{
    fn solve(
        &self,
        limit: &crate::solver::SolveLimit,
        pat: &[crate::predition::DemandPattern],
    ) -> [Option<Batch>; 6] {
        // D2-D7 收益最高cadidates
        let mut candi_recipe = vec![];
        for i in 1..7 {
            let demands = get_demands(pat, i);
            let ids = self.get_top_value(5, &demands);
            candi_recipe.push(ids);
        }

        let n = 6; // 每种配方的top
        let m = 4; // 配方种类数

        // 计算D2-D7单独收益，得到候选序列
        let mut candi_batch = vec![];
        for i in 1..7 {
            let demands = get_demands(pat, i);
            let candidate = &candi_recipe[i as usize - 1];

            let mut item_count = vec![];
            let mut total_count = m * n;
            for _ in 0..m {
                item_count.push(n)
            }

            let solver = BFSolver::new(self.data, self.info);
            let mut batch = vec![];
            let results = solver.solve(limit, &demands);
            for r in &results {
                for j in 0..m {
                    if item_count[j] == 0 {
                        continue;
                    }
                    if r.steps.contains(&(candidate[j] as u16)) {
                        batch.push(r.steps);

                        item_count[j] -= 1;
                        total_count -= 1;
                        break;
                    }
                }
                if total_count == 0 {
                    break;
                }
            }
            // println!("item count remain: {:?}. cnt: {}", item_count, results.len());
            // println!("results: {:?}", &results[0..10]);
            candi_batch.push(batch);
        }

        // println!("candidate recipe: {:?}", candi_recipe);
        // println!("candidate batch: {:?}", candi_batch);

        let mut max_val = 0; // 最大收益值
        let mut max_batch = [None; 6]; // 最大收益队列

        for rest in 0..6 {
            let mut _indexes = [0;5]; // 每天步骤的index
            let mut pos = 4; // 当前天数的位置

            let mut infos = [self.info; 5];
            let mut demand_subs = [vec![], vec![], vec![], vec![], vec![]];
            let mut batches = [None; 6];
            let mut sum_val = 0;

            // 填充初始值
            demand_subs[0].reserve_exact(pat.len());
            for _ in 0..pat.len() {
                demand_subs[0].push(0);
            }

            for p in 0..4 {
                let day = match p >= rest {
                    true => p + 1,
                    false => p,
                };

                // 更新求解信息
                let rs = self.get_recipe_state(pat, day as u8 + 1, &candi_batch[day][0], &demand_subs[p]);
                let batch;
                (batch, infos[p + 1]) = simulate_batch_seq(&infos[p], &rs);

                // 更新需求变动表
                let mut demand_sub = demand_subs[p].clone();
                self.update_demand_sub(&batch, &mut demand_sub);
                demand_subs[p + 1] = demand_sub;

                // 更新求解值
                batches[day] = Some(batch);
                let val = match limit.with_cost {
                    true => batch.value - batch.cost,
                    false => batch.value,
                };
                sum_val += val;
            }

            loop {
                let _day = match pos >= rest {
                    true => pos + 1,
                    false => pos,
                };

                // 游标在末尾，计算当前序列的值
                if pos >= 4 {
                    let rs = self.get_recipe_state(pat, _day as u8 + 1, &candi_batch[_day][_indexes[pos]], &demand_subs[pos]);
                    let (batch, _) = simulate_batch_seq(&infos[pos], &rs);

                    let calc_val = sum_val + match limit.with_cost {
                        true => batch.value - batch.cost,
                        false => batch.value,
                    };
                    if calc_val > max_val {
                        batches[_day] = Some(batch);
                        max_val = calc_val;
                        max_batch = batches;

                        // println!("{}: rest {}, {:?}", calc_val, rest, _indexes)
                    }
                }

                let next = _indexes[pos] + 1;
                // 判断当前位置是否到顶，如果到顶则向前移动游标准备增加
                if next >= candi_batch[_day].len() {
                    _indexes[pos] = 0; // 提前设置当前位置的游标为0，方便后续操作
                    if pos == 0 {
                        break;
                    }
                    pos -= 1; // 移动位置游标
                    continue;
                }

                // 没有到顶，直接向前移动
                _indexes[pos] = next;
                // 当前游标不在最右边，说明游标右边的项目均已到顶
                // 此时他们已经在前面重置了index，直接移动到目标位置即可
                if pos < 4 {
                    while pos < 4 {
                        // 更新缓存值
                        let day = match pos >= rest {
                            true => pos + 1,
                            false => pos,
                        };
    
                        let rs = self.get_recipe_state(pat, day as u8 + 1, &candi_batch[day][_indexes[pos]], &demand_subs[pos]);
                        let (batch, info) = simulate_batch_seq(&infos[pos], &rs);
    
                        infos[pos + 1] = info;
                        batches[day] = Some(batch);
    
                        demand_subs[pos + 1] = demand_subs[pos].clone();
                        self.update_demand_sub(&batch, &mut demand_subs[pos + 1]);
    
                        pos += 1;
                    }

                    // 更新sum_val
                    sum_val = 0;
                    for i in 0..4 {
                        let day = match i >= rest {
                            true => i + 1,
                            false => i,
                        };
                        match batches[day] {
                            None => (),
                            Some(batch) => {
                                sum_val += match limit.with_cost {
                                    true => batch.value - batch.cost,
                                    false => batch.value,
                                }; 
                            }
                        }
                    }
                }
            }
        }

        max_batch
    }
}

impl<'a, T> RadicalSolver<'a, T>
where
    T: IDataRepo,
{
    pub fn new(data: &'a T, info: CraftworkInfo) -> Self {
        Self {
            info,
            data
        }
    }

    /// 获取当前最大配方
    fn get_top_value(&self, size: usize, demands: &[i8]) -> Vec<usize> {
        let mut vec = vec![];
        for i in 0..self.data.recipe_len() {
            let r = self.data.recipe(i);
            let state = self.data.state(i, demands[i]);
            let val = simulate(&self.info, &state, 0);
            vec.push((i, val as f32 / r.craft_time as f32, val));
        }

        vec.sort_by(|a, b| {
            return b.1.total_cmp(&a.1);
        });

        let mut result = vec![];
        for i in 0..size {
            result.push(vec[i].0 as usize);
        }
        return result;
    }

    fn get_recipe_state(&self, pat: &[DemandPattern], day: u8, batch: &[u16; 6], demand_sub: &[i8]) -> Vec<RecipeState>{
        let demand = get_demands(pat, day);
        let mut recipe = vec![];
        for i in 0..6 {
            let id = batch[i] as usize;
            if id == 0 {
                break;
            }
            recipe.push(self.data.state(id, demand[id] - demand_sub[id]))
        }
        recipe
    }

    fn update_demand_sub(&self, batch: &Batch, dst: &mut [i8]) {
        for i in 0..batch.seq {
            let id = batch.steps[i] as usize;
            let produce = match i {
                0 => 1,
                _ => 2,
            } * self.info.workers as i8;
            dst[id] += produce;
        }
    }

    fn calc_value(&self,
        pat: &[crate::predition::DemandPattern],
        current: &[Option<[u16;6]>; 6],
    ) -> ([Option<Batch>; 6], u16, u16) {

        let mut result = [None; 6];

        let mut info = self.info.clone();
        let (mut val, mut cost) = (0, 0);

        let mut demand_sub = vec![];
        demand_sub.reserve_exact(pat.len());

        // 计算需求变动值
        for _ in 0..pat.len() {
            demand_sub.push(0);
        }

        for i in 0..6 {
            match &current[i] {
                None => result[i] = None,
                Some(batch) => {
                    // 计算需求变动
                    let recipe = self.get_recipe_state(pat, i as u8 + 1, batch, &demand_sub);

                    let batch;
                    (batch, info) = simulate_batch_seq(&info, &recipe);

                    // 更新需求变动表
                    self.update_demand_sub(&batch, &mut demand_sub);
                    result[i] = Some(batch);
                    val += batch.value;
                    cost += batch.cost;
                }
            }
        }

        (result, val, cost)
    }
}
