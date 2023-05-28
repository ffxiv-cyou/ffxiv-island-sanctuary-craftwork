use mji_craftwork::data::{
    CraftworkInfo, GameDataRepo, IDataRepo, Popularity, Recipe, RecipeState,
};
use mji_craftwork::simulator::{simulate, simulate_batch_seq, simulate_multi_batch, Batch};

mod data;
use data::new_repo;

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
        ((32.0f32 * 1.1 * 1.2).floor() * 1.2 * 1.3).floor()
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
            2,
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
            2,
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

    let (batch, info) = simulate_multi_batch(&info, &recipes);
    // println!("batch: {:?}", batch);
    // println!("craftwork: {:?}", info);

    assert_eq!(
        Batch {
            seq: 4,
            steps: [14, 49, 14, 49, 0, 0],
            values: [72, 416, 158, 450, 0, 0],
            cmp_value: 0,
            value: 1096,
            cost: 0,
            time: 24
        },
        batch[0].1
    );
    assert_eq!(
        Batch {
            seq: 5,
            steps: [59, 58, 57, 58, 57, 0],
            values: [86, 152, 272, 162, 294, 0],
            cmp_value: 0,
            value: 966,
            cost: 0,
            time: 24
        },
        batch[1].1
    );
    assert_eq!(14, info.tension);
}
