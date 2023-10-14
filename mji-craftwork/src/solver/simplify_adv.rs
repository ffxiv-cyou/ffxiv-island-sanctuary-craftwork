use crate::{
    data::IDataRepo,
    simulator::{simulate_batch, simulate_multi_batch},
    RecipeState,
};

use super::{
    solver_multi::{Batches, SolverDual},
    Batch, SolverCtx, SolverSingle,
};

pub struct AdvancedSimplifySolver<U>
where
    U: SolverSingle,
{
    solver: U,
}

impl<U> AdvancedSimplifySolver<U>
where
    U: SolverSingle,
{
    pub fn new(solver: U) -> Self {
        Self { solver }
    }
}

impl<U> SolverDual for AdvancedSimplifySolver<U>
where
    U: SolverSingle,
{
    fn solve_fn<'b, T>(
        &mut self,
        ctx: &SolverCtx<'b, T>,
        demands: &[i8],
        workers: u8,
        mut cb: impl FnMut(&Batches),
    ) where
        T: IDataRepo,
    {
        let mut info = ctx.info.clone();
        info.workers = workers;

        // 使用最大worker计算在最坏叠箱发生时（即各队列在各种组合下的理论最小值）的最大值
        let max_batch = self.solver.solve_best(ctx, demands, workers);
        let max_batch_val = match ctx.limit.with_cost {
            true => max_batch.value - max_batch.cost,
            false => max_batch.value,
        };

        // 计算非叠箱时的最大值（即各种组合中的理论最大值）
        let mut candidates = vec![];
        self.solver.solve_fn(ctx, demands, 1, |b| {
            // 补偿多个worker的干劲加成
            // let factor = 1f32 + (0.01 * ((b.seq as u8 - 1) * workers) as f32);
            let factor =
                (100 + (b.seq as u8 - 1) * workers) as f32 / (100 + b.seq as u8 - 1) as f32;
            let value = (b.value as f32 * factor) as u16;
            let value = match ctx.limit.with_cost {
                true => value - b.cost,
                false => value,
            };
            if value >= max_batch_val {
                let mut recipe: [Option<RecipeState>; 6] = [None, None, None, None, None, None];
                for i in 0..b.seq as usize {
                    let id = b.steps[i] as usize;
                    recipe[i] = Some(ctx.repo.state(id, demands[id]));
                }
                candidates.push(recipe);
            }
        });

        // println!("{} {:?} {:?}", ctx.repo.recipe_len(), self.info, limit);
        // println!("{} {} {} {}", ctx.repo.popular(1) as u8, ctx.repo.popular(2) as u8, ctx.repo.popular(3) as u8, ctx.repo.popular(4) as u8);
        // println!(
        //     "candidates: {}, max_val: {}",
        //     candidates.len(),
        //     max_batch_val
        // );

        // 模拟计算
        for i in 0..candidates.len() {
            for j in i + 1..candidates.len() {
                let mut k = workers - 1;
                let mut max_batches = Batches::new();
                let mut max_val = 0;
                while k > 0 {
                    let recipes = match k > (workers - k) {
                        true => [(k, candidates[i]), (workers - k, candidates[j])],
                        false => [(workers - k, candidates[j]), (k, candidates[i])],
                    };
                    let (batch, _) = simulate_multi_batch(&ctx.info, &recipes);
                    let batch = Batches::from_batch(&batch);
                    let value = match ctx.limit.with_cost {
                        true => batch.value - batch.cost,
                        false => batch.value,
                    };
                    if value > max_val {
                        max_val = value;
                        max_batches = batch;
                    }
                    k -= 1;
                }
                cb(&max_batches);
            }

            let mut recipe: [RecipeState; 6] = [RecipeState::empty(); 6];
            let mut len = 0;
            for j in 0..candidates[i].len() {
                recipe[j] = match candidates[i][j] {
                    Some(r) => {
                        len += 1;
                        r
                    }
                    None => RecipeState::empty(),
                };
            }

            let batch = simulate_batch(&info, &recipe[0..len]);
            let batches = [(workers, batch), (0, Batch::new())];
            cb(&Batches::from_batch(&batches));
        }
    }
}
