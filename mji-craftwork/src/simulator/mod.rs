mod batch;
pub use batch::Batch;
mod single;

pub use single::{simulate, simulate_batch, simulate_batch_seq};

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
