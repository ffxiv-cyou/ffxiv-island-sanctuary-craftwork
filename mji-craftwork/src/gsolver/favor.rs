use crate::{
    data::{Favors, IDataRepo},
    predition::get_demands,
    simulator::simulate_multi_batch,
    solver::{Batches, SolverWithBatch},
};

use super::{GFavorSolver, GSolver};

pub struct FavorSolver<G, S>
where
    G: GSolver,
    S: SolverWithBatch,
{
    gsolver: G,
    solver: S,
}

impl<G, S> FavorSolver<G, S>
where
    G: GSolver,
    S: SolverWithBatch,
{
    pub fn new(gsolver: G, solver: S) -> Self {
        Self { gsolver, solver }
    }

    fn solve_complex<'b, T>(
        &mut self,
        ctx: &mut crate::solver::SolverCtx<'b, T>,
        pat: &[crate::predition::DemandPattern],
        favors: &[crate::data::Favor],
    ) -> [Option<crate::solver::Batches>; 6]
    where
        T: IDataRepo,
    {
        ctx.info.workers -= 1;
        let result = self.gsolver.solve(ctx, pat);

        // 预先计算当前需求变动值
        let mut demands_pre_sub = vec![0i8; pat.len()];
        for batch in result {
            match batch {
                None => (),
                Some(batch) => batch.demand_sub(&mut demands_pre_sub, ctx.info.workers as i8),
            }
        }

        let mut base_result = [None; 6];
        for i in 0..6 {
            base_result[i] = match result[i] {
                None => None,
                Some(batch) => Some(Batches::from_single_batch(batch, ctx.info.workers)),
            }
        }

        let mut skip_day = 0;
        for i in 0..6 {
            match result[i] {
                None => skip_day = i,
                Some(_) => (),
            }
        }

        let mut favors = Favors::<3>::new(favors);
        for item in result {
            if let Some(batch) = item {
                favors.add_seq(&batch, ctx.info.workers);
            }
        }
        let favors = favors.to_favors();

        ctx.info.workers += 1;

        let mut best_result = [None; 6];
        let mut best_val = 0;

        let mut last_day_arr = [5; 5];
        for index in 0..((1..=5).product()) {
            // 计算日序列
            let mut day_arr = [0; 5];
            for j in 0..day_arr.len() {
                let factor_mod: i32 = 5 - j as i32;
                let factor: i32 = (1..factor_mod).product();
                let mut seq = ((index as i32 / factor) % factor_mod) as usize;

                let mut arr = [5; 5];
                for k in 0..j {
                    arr[k] = day_arr[k];
                }
                arr.sort();

                for k in 0..j {
                    if seq >= arr[k] {
                        seq += 1;
                    }
                }
                day_arr[j] = seq;
            }
            // 根据缓存数据计算起始位置
            let mut recalc_index = 5;
            for j in 0..5 {
                if day_arr[j] != last_day_arr[j] {
                    recalc_index = j;
                    break;
                }
            }
            last_day_arr = day_arr;

            // 排猫票的班
            let mut demand_sub = vec![0i8; pat.len()];
            let mut favors = Favors::<3>::new(&favors);

            for j in 0..5 {
                let day = day_arr[j];
                let day = match day >= skip_day {
                    true => day + 1,
                    false => day,
                };

                if let Some(base) = &mut base_result[day] {
                    if j >= recalc_index {
                        // 当日需求
                        let mut demands = get_demands(pat, day as u8 + 1);
                        for k in 0..demands.len() {
                            demands[k] += demands_pre_sub[k];
                            demands[k] += demand_sub[k];
                        }

                        let set = (base.batches[0].0, base.batches[0].1.steps);
                        let best = self.solver.solve_favor_best(
                            ctx,
                            &[set],
                            &demands,
                            1,
                            &favors.to_favors(),
                        );
                        base.set_second_result(best.batch, 1);
                    }

                    // 更新需求变动表
                    base.batches[1].1.demand_sub(&mut demand_sub, 1);
                    // 更新猫票表
                    favors.add_seq(&base.batches[1].1, base.batches[1].0)
                }
            }

            // 计算排班表收益
            let mut result = [None; 6];
            let mut value = 0;
            let mut cost = 0;
            let mut demand_sub = vec![0i8; pat.len()];
            let mut info = ctx.info.clone();
            for i in 0..base_result.len() {
                result[i] = match base_result[i] {
                    None => None,
                    Some(batch) => {
                        let demand = get_demands(pat, i as u8 + 1);
                        let mut recipes = vec![];
                        for (worker, batch) in &batch.batches {
                            let mut recipe = [None; 6];
                            for i in 0..batch.seq as usize {
                                let id = batch.steps[i] as usize;
                                recipe[i] = Some(ctx.repo.state(id, demand[id] - demand_sub[id]));
                            }
                            recipes.push((*worker, recipe));
                        }

                        let batches;
                        (batches, info) = simulate_multi_batch(&info, &recipes);
                        // 更新需求变动表
                        for (worker, b) in &batches {
                            b.demand_sub(&mut demand_sub, *worker as i8);
                        }

                        let batches = Batches::from_batch(&batches);
                        value += batches.value;
                        cost += batches.cost;
                        Some(batches)
                    }
                }
            }

            // 选择最佳
            let cmp_value = match ctx.limit.with_cost {
                true => value - cost,
                false => value,
            };
            if cmp_value > best_val {
                best_val = cmp_value;
                best_result = result;
            }

            println!("day: {:?}, val: {}", day_arr, cmp_value);
        }

        best_result
    }
}

impl<G, S> GFavorSolver for FavorSolver<G, S>
where
    G: GSolver,
    S: SolverWithBatch,
{
    fn solve<'b, T>(
        &mut self,
        ctx: &mut crate::solver::SolverCtx<'b, T>,
        pat: &[crate::predition::DemandPattern],
        favors: &[crate::data::Favor],
    ) -> [Option<crate::solver::Batches>; 6]
    where
        T: crate::data::IDataRepo,
    {
        ctx.info.workers -= 1;
        let result = self.gsolver.solve(ctx, pat);

        // 预先计算当前需求变动值
        let mut demands_sub = vec![0i8; pat.len()];
        for batch in result {
            match batch {
                None => (),
                Some(batch) => batch.demand_sub(&mut demands_sub, ctx.info.workers as i8),
            }
        }

        let mut base_result = [None; 6];
        for i in 0..6 {
            base_result[i] = match result[i] {
                None => None,
                Some(batch) => Some(Batches::from_single_batch(batch, ctx.info.workers)),
            }
        }

        let mut favors = Favors::<3>::new(favors);
        for item in result {
            if let Some(batch) = item {
                favors.add_seq(&batch, ctx.info.workers);
            }
        }

        for i in 0..6 {
            if let Some(item) = &mut base_result[i] {
                // 当日需求
                let mut demands = get_demands(pat, i as u8 + 1);
                for k in 0..demands.len() {
                    demands[k] += demands_sub[k];
                }

                let set = (item.batches[0].0, item.batches[0].1.steps);
                let best =
                    self.solver
                        .solve_favor_best(ctx, &[set], &demands, 1, &favors.to_favors());
                item.set_second_result(best.batch, 1);
                // 更新需求变动表
                item.batches[1].1.demand_sub(&mut demands_sub, 1);
                // 更新猫票表
                favors.add_seq(&item.batches[1].1, item.batches[1].0)
            }
        }

        return base_result;
    }
}
