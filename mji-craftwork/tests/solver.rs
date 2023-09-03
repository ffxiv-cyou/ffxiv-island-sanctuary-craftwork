use mji_craftwork::{
    data::CraftworkInfo,
    init_repo,
    predition::get_demands,
    simulate, solve_singleday,
    solver::{
        AdvancedSimplifySolver, BFSolver, SimplifySolver, SolverDual, SolverSingle, SolverWithBatch,
    },
};
use rand::prelude::Distribution;

use test_data::{empty_demands, from_pattern_code, make_config, make_limit, new_repo};

/// 库初始化测试
#[test]
fn init_test() {
    use test_data::{CRAFT_OBJECTS, POPULARITY_LIST};
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
    let demands = empty_demands();

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
    let demands = empty_demands();
    let (repo, info, limit) = make_config(1, &empty);
    let solver = BFSolver::new(&repo, info);

    let result = SolverSingle::solve_unordered(&solver, &limit, &demands, 0);
    println!("solve space: {}", result.len());

    let result = SolverSingle::solve(&solver, &limit, &demands, 0);
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
    let demands = empty_demands();
    let (repo, info, limit) = make_config(1, &empty);
    let solver = SimplifySolver::new(&repo, info);

    let result = SolverSingle::solve_unordered(&solver, &limit, &demands, 0);
    println!("solve space: {}", result.len());
}

/// 测试BF算法运行状况
#[test]
fn predict_best() {
    let empty = vec![];
    let demands = empty_demands();
    let (repo, info, limit) = make_config(1, &empty);
    let solver = BFSolver::new(&repo, info);

    let result = SolverSingle::solve_best(&solver, &limit, &demands, 0);
    println!("{:?}", result);
}

/// 测试Simplify算法运行状况
#[test]
fn predict_simplify_best() {
    let empty = vec![];
    let demands = empty_demands();
    let (repo, info, limit) = make_config(1, &empty);
    let solver = SimplifySolver::new(&repo, info);
    let result = SolverSingle::solve_best(&solver, &limit, &demands, 0);
    println!("{:?}", result);
}

#[test]
fn simulator() {
    let repo = new_repo(1);
    let demands = empty_demands();

    let state = CraftworkInfo::new(0, 35, 1, 1);
    let arr = vec![13, 23, 13, 23];
    let result = simulate(&repo, &state, arr, &demands);
    assert_eq!(result, [0, 38, 240, 78, 246, 0, 0]);
}

/// 对比BF算法和Simplify算法
#[test]
fn compare_bf_simplify() {
    let empty = vec![];
    let mut demands = empty_demands();
    let (mut repo, info, limit) = make_config(1, &empty);

    let demand_range = rand::distributions::Uniform::new(2, 24u8);
    let pop_range = rand::distributions::Uniform::new(1, 100);

    let mut rng = rand::thread_rng();

    for i in 0..1000 {
        let pop = pop_range.sample(&mut rng);
        repo.set_popular_pattern(pop);
        for i in 0..demands.len() {
            demands[i] = demand_range.sample(&mut rng) as i8;
        }

        let simplify = SimplifySolver::new(&repo, info);
        let bf = BFSolver::new(&repo, info);

        let sim_result = SolverSingle::solve_best(&simplify, &limit, &demands, 0);
        let bf_result = SolverSingle::solve_best(&bf, &limit, &demands, 0);

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

/// 测试已设定一个排班的情况下求解
#[test]
fn test_solver_multi() {
    let empty = vec![];
    let demands = empty_demands();
    let (repo, info, limit) = make_config(1, &empty);
    let solver = BFSolver::new(&repo, info);

    let set = [(3, [14, 49, 14, 49, 0, 0])];
    let result = SolverWithBatch::solve_unordered(&solver, &limit, &set, &demands, 1);
    println!("solve space: {}", result.len());
    let mut max_len = 0;
    let mut max_time = 0;
    for r in result {
        max_len = max_len.max(r.batch.seq);
        max_time = max_time.max(r.batch.time);
    }
    assert_eq!(max_len, 6);
    assert_eq!(max_time, 24);

    let result = SolverWithBatch::solve(&solver, &limit, &set, &demands, 1);
    assert_eq!(result.len(), limit.max_result);
    for i in 0..10 {
        println!(
            "val: {}, {:?}, {:?} | {}",
            result[i].batch.get_val(),
            result[i].batch.get_steps(),
            result[i].batch.get_values(),
            result[i].value
        );
    }
}

#[test]
fn test_solver_simp_adv() {
    let empty = vec![49];
    let (repo, info, demand_pat) =
        from_pattern_code(b"AWCJtKVXcyvBnLQ7EzyKZ4sckoYqRlIaR5xjg7kqxUF3SoyVIwYAAAAA");
    let mut limit = make_limit(&empty);
    let demands = get_demands(&demand_pat, 1);

    println!("demands: {:?}", demands);

    let solver = BFSolver::new(&repo, info);
    let solver = AdvancedSimplifySolver::new(&repo, &solver, info);

    // let result = SolverDual::solve_unordered(&solver, &limit, &demands, 4);
    // println!("solve space: {}", result.len());

    limit.max_result = 100;
    let result = SolverDual::solve(&solver, &limit, &demands, 4);
    for i in 0..result.len() {
        println!("{}", result[i]);
    }
}

#[test]
fn test_solver_simp_loop() {
    let empty = vec![];
    let demands = empty_demands();
    for pop in 1..99 {
        let (repo, info, limit) = make_config(pop, &empty);
        let solver = BFSolver::new(&repo, info);
        let solver = AdvancedSimplifySolver::new(&repo, &solver, info);

        let result = SolverDual::solve_unordered(&solver, &limit, &demands, 4);
        println!("pop {} solve space: {}", pop, result.len());
    }
}
