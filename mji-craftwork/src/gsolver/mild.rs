use crate::{
    data::{CraftworkInfo, IDataRepo},
    predition::get_demands,
    simulator::{simulate_batch_seq, Batch},
    solver::{Solver, SimplifySolver},
};

use super::GSolver;

pub struct MildSolver<'a, T>
where
    T: IDataRepo,
{
    info: CraftworkInfo,
    data: &'a T,
}

impl<'a, T> GSolver for MildSolver<'a, T>
where
    T: IDataRepo,
{
    fn solve(
        &self,
        limit: &crate::solver::SolveLimit,
        pat: &[crate::predition::DemandPattern],
    ) -> [Option<Batch>; 6] {
        let mut current = [None; 6];
        let mut max = [None; 6];
        let mut max_val = 0;
        let mut demand = vec![0; pat.len()];
        self.dfs(limit, pat, &mut demand, &mut current, 0, &mut max, &mut max_val);
        
        max
    }
}

impl<'a, T> MildSolver<'a, T>
where
    T: IDataRepo,
{
    pub fn new(data: &'a T, info: CraftworkInfo) -> Self {
        Self { info, data }
    }

    fn dfs(
        &self,
        limit: &crate::solver::SolveLimit,
        pat: &[crate::predition::DemandPattern],
        demand_sub: &mut [i8],
        current: &mut [Option<Batch>; 6],
        depth: u8,
        max: &mut [Option<Batch>; 6],
        max_val: &mut u16,
    ) {
        for i in 0..6 {
            if current[i] != None {
                continue;
            }

            // 计算当前干劲
            let mut tension = 0;
            for i in 0..i {
                match current[i] {
                    None => continue,
                    Some(b) => {
                        tension += (b.seq - 1) as u8 * self.info.workers;
                    }
                }
            }

            // 计算需求值
            let mut info = self.info;
            info.tension = tension;
            let solver = SimplifySolver::new(self.data, info);

            let mut demand = get_demands(pat, i as u8 + 1);
            for i in 0..demand.len().min(demand_sub.len()) {
                demand[i] -= demand_sub[i];
            }

            // 计算最佳
            let batch = solver.solve_best(limit, &demand);
            current[i] = Some(batch);

            // if depth == 1 {
            //     println!("{} {} {:?}", depth, max_val, max);
            // }

            if depth >= 4 {
                // 当前所有都求解完毕了
                let (real_batch, val, cost) = self.calc_value(pat, current);
                let val_cmp = match limit.with_cost {
                    true => val - cost,
                    false => val,
                };

                if val_cmp > *max_val {
                    *max_val = val_cmp;
                    *max = real_batch;
                }
            } else {
                // 计算需求变动值
                for i in 0..batch.steps.len() {
                    let id = batch.steps[i] as usize;
                    if id == 0 {
                        break;
                    }
                    demand_sub[id] += match i {
                        0 => 1,
                        _ => 2,
                    } * self.info.workers as i8;
                }

                self.dfs(limit, pat, demand_sub, current, depth + 1, max, max_val);
            
                // 还原需求变动值
                for i in 0..batch.steps.len() {
                    let id = batch.steps[i] as usize;
                    if id == 0 {
                        break;
                    }
                    demand_sub[id] -= match i {
                        0 => 1,
                        _ => 2,
                    } * self.info.workers as i8;
                }
            }
            current[i] = None;
        }
    }

    /// 计算一周的总收益
    fn calc_value(
        &self,
        pat: &[crate::predition::DemandPattern],
        current: &[Option<Batch>; 6],
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
                    let demand = get_demands(pat, i as u8 + 1);
                    let mut recipe = vec![];
                    for i in 0..batch.seq {
                        let id = batch.steps[i] as usize;
                        recipe.push(self.data.state(id, demand[id] - demand_sub[id]))
                    }

                    let batch;
                    (batch, info) = simulate_batch_seq(&info, &recipe);

                    // 更新需求变动表
                    for i in 0..batch.seq {
                        let id = batch.steps[i] as usize;
                        let produce = match i {
                            0 => 1,
                            _ => 2,
                        } * self.info.workers as i8;
                        demand_sub[id] += produce;
                    }
                    result[i] = Some(batch);
                    val += batch.value;
                    cost += batch.cost;
                }
            }
        }

        (result, val, cost)
    }
}
