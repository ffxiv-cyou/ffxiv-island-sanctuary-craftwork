mod batch;
pub use batch::Batch;

use crate::data::{CraftworkInfo, RecipeState};

/// 模拟一个操作
/// 
/// dup: 需求变动值
pub fn simulate(info: &CraftworkInfo, recipe: &RecipeState, dup: u8) -> u16 {
    let val = recipe.value() as f32 * info.factor();
    let val = val.floor();
    let val = val * recipe.factor(dup as i16);
    val.floor() as u16
}

/// 模拟一系列的工序
///
/// 注意，这里只考虑了连击的情况。
pub fn simulate_batch(info: &CraftworkInfo, recipe: &[RecipeState]) -> Batch {
    simulate_batch_seq(info, recipe).0
}

/// 模拟一系列的工序，返回工序Batch和对应变动后的工坊信息
/// 
/// 注意，这里只考虑了连击的情况。
pub fn simulate_batch_seq(info: &CraftworkInfo, recipe: &[RecipeState]) -> (Batch, CraftworkInfo) {
    let mut batch = Batch::new();
    let mut info = info.clone();
    for i in 0..recipe.len() {
        let r = &recipe[i];
        let demand_sub = batch.get_produce(r.id()) * info.workers;
        if i != 0 {
            info = info.next();
        }
        let val = simulate(&info, r, demand_sub);
        batch = batch.push(r.id(), val, r.cost(), r.craft_time() as u16);
    }
    (batch, info)
}

#[cfg(test)]
mod tests {
    use crate::data::{CraftworkInfo, Popularity, Recipe, RecipeState};

    use super::simulate;

    #[test]
    fn test_simulate() {
        let info = CraftworkInfo::new(10, 10, 2, 1);
        let recipe = &Recipe {
            id: 1,
            theme1: 0,
            theme2: 0,
            level: 6,
            craft_time: 4,
            value: 32,
            cost: 0,
        };
        let state = RecipeState::new(recipe, 17, Popularity::High);

        let val = simulate(&info, &state, 6);

        // 计算公式：v = floor(floor(val * f_tension * f_level) * f_demand * f_pop)
        assert_eq!(
            val as f32,
            ((32.0f32 * 1.1 * 1.2).floor() * 1.2 * 1.3).floor()
        )
    }
}
