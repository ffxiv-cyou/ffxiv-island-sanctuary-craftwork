use mji_craftwork::{
    data::{CraftworkInfo},
    init_repo, simulate, solve_singleday,
    solver::{BFSolver, SolveLimit, Solver},
};

mod data;
use crate::data::new_repo;

#[test]
fn init_test() {
    use data::{ CRAFT_OBJECTS, POPULARITY_LIST};
    let mut recipes = vec![];
  
    for i in 0..CRAFT_OBJECTS.len() {
        recipes.push(CRAFT_OBJECTS[i][0] as u16);
        recipes.push(CRAFT_OBJECTS[i][2] as u16);
        recipes.push(CRAFT_OBJECTS[i][3] as u16);
        recipes.push(CRAFT_OBJECTS[i][13] as u16);
        recipes.push(CRAFT_OBJECTS[i][14] as u16);
        recipes.push(CRAFT_OBJECTS[i][15] as u16);
        recipes.push(0);
    }
    let mut pop_vec = vec![];
    for r in POPULARITY_LIST {
        for i in 1..r.len() {
            pop_vec.push(r[i] as u8);
        }
    }
  
    let repo = init_repo(recipes, pop_vec, 62);
    let demands = vec![9; 62];

    solve_singleday(
        &repo,
        &CraftworkInfo::new(0, 30, 1, 1),
        10,
        vec![],
        &demands,
        24,
        false
    );
}

#[test]
fn predict() {
    let repo = new_repo(1);
    let demands = vec![9; 62];
    let solver = BFSolver::new(&repo, CraftworkInfo::new(0, 35, 1, 1));
    let empty = vec![];
    let limit = SolveLimit::new(10, &empty, 24, false);
    let result = solver.solve(&limit, &demands);
    println!("length: {}", result.len());
    assert_eq!(result.len(), limit.max_result);
    for i in 0..20 {
        println!(
            "val: {},{:?}, {:?}",
            result[i].get_val(),
            result[i].get_steps(),
            result[i].get_values()
        );
    }
}

#[test]
fn simulator() {
    let repo = new_repo(1);
    let demands = vec![9; 62];

    let state = CraftworkInfo::new(0, 35, 1, 1);
    let arr = vec![13, 23, 13, 23];
    let result = simulate(&repo, &state, arr, &demands);
    assert_eq!(result, [0, 38, 240, 78, 246, 0, 0]);
}
