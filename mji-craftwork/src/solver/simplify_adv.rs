use crate::{
    data::{CraftworkInfo, IDataRepo},
    simulator::{simulate_batch, simulate_multi_batch},
    RecipeState,
};

use super::{
    solver_multi::{Batches, SolverDual},
    Batch, SolveLimit, SolverSingle,
};

pub struct AdvancedSimplifySolver<'a, T, U>
where
    T: IDataRepo,
    U: SolverSingle,
{
    info: CraftworkInfo,
    data: &'a T,
    solver: &'a U,
}

impl<'a, T, U> AdvancedSimplifySolver<'a, T, U>
where
    T: IDataRepo,
    U: SolverSingle,
{
    pub fn new(data: &'a T, solver: &'a U, info: CraftworkInfo) -> Self {
        Self { info, data, solver }
    }
}

impl<'a, T, U> SolverDual for AdvancedSimplifySolver<'a, T, U>
where
    T: IDataRepo,
    U: SolverSingle,
{
    fn solve_unordered(&self, limit: &SolveLimit, demands: &[i8], workers: u8) -> Vec<Batches> {
        let mut info = self.info.clone();
        info.workers = workers;
        
        // 使用最大worker计算在最坏叠箱发生时（即各队列在各种组合下的理论最小值）的最大值
        let max_batch = self.solver.solve_best(limit, demands, workers);
        let max_batch_val = match limit.with_cost {
            true => max_batch.value - max_batch.cost,
            false => max_batch.value,
        };

        // 计算非叠箱时的最大值（即各种组合中的理论最大值）
        let mut candidates = vec![];
        let batches = self.solver.solve_unordered(limit, demands, 1);
        for b in batches {
            // 补偿多个worker的干劲加成
            let factor = 1f32 + (0.01 * ((b.seq as u8 - 1) * workers) as f32);
            // let factor = (100 + (b.seq as u8 - 1) * workers) as f32 / (100 + b.seq as u8 - 1) as f32;
            let value = (b.value as f32 * factor) as u16;
            let value = match limit.with_cost {
                true => value - b.cost,
                false => value,
            };
            if value >= max_batch_val {
                let mut recipe: [Option<RecipeState>; 6] = [None, None, None, None, None, None];
                for i in 0..b.seq {
                    let id = b.steps[i] as usize;
                    recipe[i] = Some(self.data.state(id, demands[id]));
                }
                candidates.push(recipe);
            }
        }

        // println!("{} {:?} {:?}", self.data.recipe_len(), self.info, limit);
        // println!("{} {} {} {}", self.data.popular(1) as u8, self.data.popular(2) as u8, self.data.popular(3) as u8, self.data.popular(4) as u8);
        println!(
            "candidates: {}, max_val: {}",
            candidates.len(),
            max_batch_val
        );

        let mut result = vec![];

        // 模拟计算
        for i in 0..candidates.len() {
            for j in i + 1..candidates.len() {
                let mut k = workers - 1;
                while k > 0 {
                    let recipes = [(k, candidates[i]), (workers - k, candidates[j])];
                    let (val, _) = simulate_multi_batch(&self.info, &recipes);
                    result.push(Batches::from_batch(&val));
                    k -= 1;
                }
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
            result.push(Batches::from_batch(&batches));
        }

        result
    }
}
