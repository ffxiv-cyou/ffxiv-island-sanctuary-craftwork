use mji_craftwork::{
    data::{CraftworkInfo, GameDataRepo},
    init_repo, simulate, solve_singleday,
    solver::{BFSolver, SimplifySolver, SolveLimit, Solver},
};
use rand::prelude::Distribution;

mod data;
use crate::data::new_repo;

fn make_demands() -> Vec<i8> {
    vec![9; 82]
}

fn make_limit(ban: &[u16]) -> SolveLimit {
    SolveLimit::new(11, ban, 24, false)
}

fn make_config(pop: usize, ban: &[u16]) -> (GameDataRepo, Vec<i8>, CraftworkInfo, SolveLimit) {
    let repo = new_repo(pop);
    let demands = make_demands();
    let info = CraftworkInfo::new(0, 35, 1, 1);
    let limit = make_limit(&ban);

    (repo, demands, info, limit)
}

/// 库初始化测试
#[test]
fn init_test() {
    use data::{CRAFT_OBJECTS, POPULARITY_LIST};
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
    let demands = make_demands();

    solve_singleday(
        &repo,
        &CraftworkInfo::new(0, 30, 1, 1),
        10,
        vec![],
        &demands,
        24,
        false,
    );
}

/// 测试 BF算法
#[test]
fn predict() {
    let empty = vec![];
    let (repo, demands, info, limit) = make_config(1, &empty);
    let solver = BFSolver::new(&repo, info);

    let result = solver.solve_unordered(&limit, &demands);
    println!("solve space: {}", result.len());

    let result = solver.solve(&limit, &demands);
    assert_eq!(result.len(), limit.max_result);
    for i in 0..1 {
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
    let empty = vec![];
    let (repo, demands, info, limit) = make_config(1, &empty);
    let solver = SimplifySolver::new(&repo, info);

    let result = solver.solve_unordered(&limit, &demands);
    println!("solve space: {}", result.len());
}

/// 测试BF算法运行状况
#[test]
fn predict_best() {
    let empty = vec![];
    let (repo, demands, info, limit) = make_config(1, &empty);
    let solver = BFSolver::new(&repo, info);

    let result = solver.solve_best(&limit, &demands);
    println!("{:?}", result);
}

/// 测试Simplify算法运行状况
#[test]
fn predict_simplify_best() {
    let empty = vec![];
    let (repo, demands, info, limit) = make_config(1, &empty);
    let solver = SimplifySolver::new(&repo, info);
    let result = solver.solve_best(&limit, &demands);
    println!("{:?}", result);
}

#[test]
fn simulator() {
    let repo = new_repo(1);
    let demands = make_demands();

    let state = CraftworkInfo::new(0, 35, 1, 1);
    let arr = vec![13, 23, 13, 23];
    let result = simulate(&repo, &state, arr, &demands);
    assert_eq!(result, [0, 38, 240, 78, 246, 0, 0]);
}

/// 对比BF算法和Simplify算法
#[test]
fn compare_bf_simplify() {
    let empty = vec![];
    let (mut repo, mut demands, info, limit) = make_config(1, &empty);

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

        assert_eq!(
            bf_result.value, sim_result.value,
            "iter {}, pop {}, demands {:?}, bf {:?}, simple {:?},",
            i, pop, demands, bf_result, sim_result
        );

        if i % 100 == 0 {
            println!("{}", i)
        }
    }
}
