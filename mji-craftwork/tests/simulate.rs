use mji_craftwork::data::{
    CraftworkInfo, GameDataRepo, IDataRepo, Popularity, Recipe, RecipeState,
};
use mji_craftwork::simulator::{simulate, simulate_batch_seq, simulate_multi_batch, Batch};

use test_data::new_repo;

fn get_recipe_state(repo: &GameDataRepo, id: u8) -> RecipeState {
    repo.state(id as usize, 9)
}

fn get_recipe_states(repo: &GameDataRepo, seq: &[u8]) -> Vec<RecipeState> {
    let mut recp = vec![];
    for s in seq {
        recp.push(get_recipe_state(repo, s.clone()));
    }
    recp
}

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
        ((32.0f32 * 1.1 * 1.2).floor() * 1.3 * 1.2).floor()
    )
}

#[test]
fn test_simulate2() {
    let info = CraftworkInfo::new(25, 35, 3, 4);
    let recipe = &Recipe {
        id: 1,
        theme1: 0,
        theme2: 0,
        level: 6,
        craft_time: 8,
        value: 136,
        cost: 0,
    };
    let state = RecipeState::new(recipe, 25, Popularity::High);

    let val = simulate(&info, &state, 0);

    // 计算公式：v = floor(floor(val * f_tension * f_level) * f_demand * f_pop)
    assert_eq!(
        val as f32,
        ((136.0f32 * 1.3f32 * 1.25f32).floor() * 1.6f32 * 1.2f32).floor()
    )
}

#[test]
fn test_simulate3() {
    let info = CraftworkInfo::new(12, 35, 3, 4);
    let recipe = &Recipe {
        id: 1,
        theme1: 0,
        theme2: 0,
        level: 6,
        craft_time: 8,
        value: 52,
        cost: 0,
    };
    let state = RecipeState::new(recipe, 17, Popularity::High);

    let val = simulate(&info, &state, 0);

    // 计算公式：v = floor(floor(val * f_tension * f_level) * f_demand * f_pop)
    assert_eq!(
        val as f32,
        ((52.0f32 * 1.3f32 * 1.12f32).floor() * 1.3f32 * 1.2f32).floor()
    )
}
#[test]
fn test_simulate_batch() {
    let repo = new_repo(1);
    let info = CraftworkInfo::new(0, 35, 2, 3);
    let seq = [14, 49, 14, 49];
    let recipe = get_recipe_states(&repo, &seq);

    let (batch, info) = simulate_batch_seq(&info, &recipe);

    // println!("batch: {:?}", batch);
    // println!("craftwork: {:?}", info);

    assert_eq!(
        Batch {
            seq: 4,
            steps: [14, 49, 14, 49, 0, 0],
            values: [72, 414, 154, 436, 0, 0],
            cmp_value: 0,
            value: 1076,
            cost: 0,
            time: 24
        },
        batch
    );
    assert_eq!(9, info.tension);
}

#[test]
fn test_simulate_multi_batch() {
    let repo = new_repo(1);
    let info = CraftworkInfo::new(0, 35, 2, 3);
    let recipes = [
        (
            3,
            [
                Some(get_recipe_state(&repo, 14)),
                Some(get_recipe_state(&repo, 49)),
                Some(get_recipe_state(&repo, 14)),
                Some(get_recipe_state(&repo, 49)),
                None,
                None,
            ],
        ),
        (
            1,
            [
                Some(get_recipe_state(&repo, 59)),
                Some(get_recipe_state(&repo, 58)),
                Some(get_recipe_state(&repo, 57)),
                Some(get_recipe_state(&repo, 58)),
                Some(get_recipe_state(&repo, 57)),
                None,
            ],
        ),
    ];

    let (batch, new_info) = simulate_multi_batch(&info, &recipes);
    println!("batch: {:?}", batch);
    println!("craftwork: {:?}", info);

    assert_eq!([72, 422, 158, 452, 0, 0], batch[0].1.values);
    assert_eq!([86, 152, 278, 164, 292, 0], batch[1].1.values);
    assert_eq!(13, new_info.tension);
}
