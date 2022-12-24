mod batch;
pub use batch::Batch;

use crate::data::{CraftworkInfo, RecipeState};

/// 模拟一个操作
pub fn simulate(info: &CraftworkInfo, recipe: &RecipeState) -> u16 {
    let val = recipe.value() as f32 * info.factor();
    let val = val.floor();
    let val = val * recipe.factor();
    val.floor() as u16
}

/// 模拟一系列的工序
/// 
/// 注意，这里只考虑了连击的情况。
pub fn simulate_batch(info: &CraftworkInfo, recipe: &[RecipeState]) -> Batch {
    let mut batch = Batch::new();
    let mut info = info.clone();
    for r in recipe {
        let val = simulate(&info, r);
        info = info.next();
        batch = batch.push(r.id(), val, r.craft_time() as u16);
    }
    batch
}

#[cfg(test)]
mod tests {
    use crate::data::{CraftworkInfo, RecipeState, Recipe, Demand, Popularity};

    use super::simulate;

    #[test]
    fn test_simulate() {
        let info = CraftworkInfo::new(10, 10, 2);
        let recipe = &Recipe{
            id: 1,
            theme1: 0,
            theme2: 0,
            level: 6,
            craft_time: 4,
            value: 32
        };
        let state = RecipeState::new(recipe, Demand::High, Popularity::High);

        let val = simulate(&info, &state);

        // 计算公式：v = floor(floor(val * f_tension * f_level) * f_demand * f_pop)
        assert_eq!(val as f32, ((32.0f32 * 1.1 * 1.2).floor() * 1.2 * 1.3).floor())
    }
}
