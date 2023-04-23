use mji_craftwork::{
    data::{CraftworkInfo},
    init_repo, simulate, solve_singleday,
    solver::{BFSolver, SolveLimit, Solver, SimplifySolver},
};
use rand::prelude::Distribution;

mod data;
use crate::data::new_repo;

/// 库初始化测试
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

/// 测试 BF算法
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

/// 测试Simplify算法
#[test]
fn predict_simplify() {
    let repo = new_repo(1);
    let demands = vec![9; 62];
    let solver = SimplifySolver::new(&repo, CraftworkInfo::new(0, 35, 1, 1));
    let empty = vec![];
    let limit = SolveLimit::new(10, &empty, 24, false);
    let result = solver.solve_unordered(&limit, &demands);
    println!("length: {}", result.len());
}

/// 测试BF算法运行状况
#[test]
fn predict_best() {
    let repo = new_repo(1);
    let demands = vec![9; 62];
    let solver = BFSolver::new(&repo, CraftworkInfo::new(0, 35, 1, 1));
    let empty = vec![];
    let limit = SolveLimit::new(10, &empty, 24, false);
    let result = solver.solve_best(&limit, &demands);
    println!("{:?}", result);
}

/// 测试Simplify算法运行状况
#[test]
fn predict_simplify_best() {
    let repo = new_repo(1);
    let demands = vec![9; 62];
    let solver = SimplifySolver::new(&repo, CraftworkInfo::new(0, 35, 1, 1));
    let empty = vec![];
    let limit = SolveLimit::new(10, &empty, 24, false);
    let result = solver.solve_best(&limit, &demands);
    println!("{:?}", result);
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

/// 对比BF算法和Simplify算法
#[test]
fn compare_bf_simplify() {
    let mut repo = new_repo(1);
    let mut demands = vec![9; 62];

    let empty = vec![];
    let limit = SolveLimit::new(10, &empty, 24, false);
    let info = CraftworkInfo::new(0, 35, 2, 3);
    
    let demand_range = rand::distributions::Uniform::new(2, 24u8);
    let pop_range = rand::distributions::Uniform::new(1, 100);
    
    let mut rng = rand::thread_rng();

    for i in 0..100000 {
        let pop = pop_range.sample(&mut rng);
        repo.set_popular_pattern(pop);
        for i in 0..demands.len() {
            demands[i] = demand_range.sample(&mut rng) as i8;
        }

        let simplify = SimplifySolver::new(&repo, info);
        let bf = BFSolver::new(&repo, info);
        
        let sim_result = simplify.solve_best(&limit, &demands);
        let bf_result = bf.solve_best(&limit, &demands);

        assert_eq!(bf_result.value, sim_result.value, "iter {}, pop {}, demands {:?}, bf {:?}, simple {:?},", i, pop, demands, bf_result, sim_result);
    
        if i % 100 == 0 {
            println!("{}", i)
        }
    }
}