use crate::{
    data::{CraftworkInfo, IDataRepo},
    predition::get_demands,
    simulator::simulate_multi_batch,
    solver::{AdvancedSimplifySolver, BFSolver, Batches, SolverDual},
};

use super::GMultiSolver;

/// Mild 全局求解器
///
/// 尝试使用每天的最优解作为整周的最优解。
///
/// 通过对求解顺序做简单的排列组合，辅以整周的需求变动算法，使其能部分考虑到物品需求对后续求解的影响。
/// 求解时还会估计当前天的干劲对后续求解的影响，使其更准确。
pub struct MildMulitSolver<'a, T>
where
    T: IDataRepo,
{
    info: CraftworkInfo,
    data: &'a T,
}

impl<'a, T> GMultiSolver for MildMulitSolver<'a, T>
where
    T: IDataRepo,
{
    fn solve_part(
        &self,
        limit: &crate::solver::SolveLimit,
        pat: &[crate::predition::DemandPattern],
        part_id: usize,
    ) -> ([Option<Batches>; 6], u16) {
        let mut current = [None; 6];
        let mut max = [None; 6];
        let mut max_val = 0;
        let mut demand = vec![0; pat.len()];
        let mut seq = [0; 6];

        let id1 = part_id / 5; // 第一天位置
        let id2 = part_id % 5; // 第二天位置
        let id2 = match id2 >= id1 {
            true => id2 + 1,
            false => id2,          
        };
        // 更新seq用于调试
        seq[0] = id1 as u8 + 2;
        seq[1] = id2 as u8 + 2;

        // 更新需求变动
        let batch = self.add_at(limit, pat, &mut demand, &mut current, id1);
        batch.produce_add(&mut demand);
        let batch = self.add_at(limit, pat, &mut demand, &mut current, id2);
        batch.produce_add(&mut demand);

        self.dfs(
            limit,
            pat,
            &mut demand,
            &mut current,
            2,
            &mut seq,
            &mut max,
            &mut max_val,
        );

        (max, max_val)
    }

    fn parts() -> usize {
        6 * 5
    }
}

impl<'a, T> MildMulitSolver<'a, T>
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
        current: &mut [Option<Batches>; 6],
        depth: u8,
        seq: &mut [u8],
        max: &mut [Option<Batches>; 6],
        max_val: &mut u16,
    ) {
        for i in 0..6 {
            if current[i] != None {
                continue;
            }

            let batch = self.add_at(limit, pat, demand_sub, current, i);

            // if depth == 1 {
            //     println!("{} {} {:?}", depth, max_val, max);
            // }
            seq[depth as usize] = i as u8 + 2;
            if depth >= 4 {
                // 当前所有都求解完毕了
                let (real_batch, val, cost) = self.calc_value(pat, current, 0, 0);
                let val_cmp = match limit.with_cost {
                    true => val - cost,
                    false => val,
                };

                if val_cmp > *max_val {
                    *max_val = val_cmp;
                    *max = real_batch;
                    // println!("{}: {:?}", val_cmp, seq);
                    // print_week_result(&real_batch);
                }
            } else {
                // 计算需求变动值
                batch.produce_add(demand_sub);

                self.dfs(
                    limit,
                    pat,
                    demand_sub,
                    current,
                    depth + 1,
                    seq,
                    max,
                    max_val,
                );

                // 还原需求变动值
                batch.produce_sub(demand_sub);
            }
            seq[depth as usize] = 0;
            current[i] = None;
        }
    }

    fn add_at(
        &self,
        limit: &crate::solver::SolveLimit,
        pat: &[crate::predition::DemandPattern],
        demand_sub: &mut [i8],
        current: &mut [Option<Batches>; 6],
        i: usize,
    ) -> Batches {
        // 计算当前干劲
        let mut tension = 0;
        for i in 0..i {
            match current[i] {
                None => continue,
                Some(b) => {
                    tension += b.tension_add();
                }
            }
        }

        // 计算当前干劲可能为后续带来的收益增加量
        let mut tension_delta = [0; 6];
        if tension < self.info.max_tension {
            for j in 0..tension_delta.len() {
                let tension = tension + (j as u8) * self.info.workers;
                let tension = tension.min(self.info.max_tension);

                let (_, value, _) = self.calc_value(pat, current, i + 1, tension);
                tension_delta[j] = value;
            }
        }

        // 计算需求值
        let mut info = self.info;
        info.tension = tension;
        let solver = BFSolver::new(self.data, info);
        let solver = AdvancedSimplifySolver::new(self.data, &solver, info);

        let mut demand = get_demands(pat, i as u8 + 1);
        for i in 0..demand.len().min(demand_sub.len()) {
            demand[i] -= demand_sub[i];
        }

        // 计算最佳
        let batch = solver.solve_best_fn(limit, &demand, info.workers, |v, b| {
            return v + tension_delta[(b.tension_add() / self.info.workers) as usize];
        });
        current[i] = Some(batch);
        batch
    }

    /// 计算一周的总收益
    fn calc_value(
        &self,
        pat: &[crate::predition::DemandPattern],
        current: &[Option<Batches>; 6],
        begin: usize,
        tension: u8,
    ) -> ([Option<Batches>; 6], u16, u16) {
        let mut result = [None; 6];

        let mut info = self.info.clone();
        info.tension = tension;
        let (mut val, mut cost) = (0, 0);

        let mut demand_sub = vec![];
        demand_sub.reserve_exact(pat.len());

        // 计算需求变动值
        for _ in 0..pat.len() {
            demand_sub.push(0);
        }

        for i in begin..6 {
            match &current[i] {
                None => result[i] = None,
                Some(batch) => {
                    // 计算需求变动
                    let demand = get_demands(pat, i as u8 + 1);
                    let mut recipes = vec![];
                    for (worker, batch) in &batch.batches {
                        let mut recipe = [None; 6];
                        for i in 0..batch.seq as usize {
                            let id = batch.steps[i] as usize;
                            recipe[i] = Some(self.data.state(id, demand[id] - demand_sub[id]));
                        }
                        recipes.push((*worker, recipe));
                    }

                    let batches;
                    (batches, info) = simulate_multi_batch(&info, &recipes);

                    // 更新需求变动表
                    for (worker, b) in &batches {
                        b.demand_sub(&mut demand_sub, *worker as i8);
                    }

                    let mut batch = *batch;
                    batch.set_result(&batches);

                    result[i] = Some(batch);
                    val += batch.value;
                    cost += batch.cost;
                }
            }
        }

        (result, val, cost)
    }
}
