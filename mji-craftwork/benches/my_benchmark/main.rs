use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use mji_craftwork::{
    data::IDataRepo,
    gsolver::{GSolver, MildSolver, RadicalSolver},
    simulator::{simulate_batch_seq, simulate_multi_batch},
    solver::{BFSolver, SimplifySolver, SolverSingle, SolverWithBatch, AdvancedSimplifySolver, SolverDual},
};
use test_data::{make_config, make_limit, from_pattern_code};

/// 测试单日单种类求解器
fn predition_benchmark(c: &mut Criterion) {
    let empty = vec![];
    let (repo, info, limit) = make_config(1, &empty);
    let solver = BFSolver::new(&repo, info);
    let sim_solver = SimplifySolver::new(&repo, info);
    let adv_solver = AdvancedSimplifySolver::new(&repo, &solver, info);

    // 求解单个
    let mut limit1 = limit.clone();
    limit1.max_result = 1;

    let demands = vec![9; 82];
    let mut group = c.benchmark_group("predition");
    // 暴力搜索，排序前100
    group.bench_function(BenchmarkId::new("BFSolver", 100), |b| {
        b.iter(|| SolverSingle::solve(&solver, black_box(&limit), black_box(&demands), 0))
    });
    // 暴力搜索，排序前1
    group.bench_function(BenchmarkId::new("BFSolver", 1), |b| {
        b.iter(|| SolverSingle::solve(&solver, black_box(&limit1), black_box(&demands), 0))
    });
    // 暴力搜索，不排序
    group.bench_function(BenchmarkId::new("BFSolverUnorder", 1), |b| {
        b.iter(|| SolverSingle::solve_unordered(&solver, black_box(&limit), black_box(&demands), 0))
    });
    // 暴力搜索，取最优
    group.bench_function(BenchmarkId::new("BFSolverBest", 1), |b| {
        b.iter(|| SolverSingle::solve_best(&solver, black_box(&limit), black_box(&demands), 0))
    });
    // 剪枝搜索，不排序
    group.bench_function(BenchmarkId::new("SimplifySolverUnorder", 1), |b| {
        b.iter(|| {
            SolverSingle::solve_unordered(&sim_solver, black_box(&limit), black_box(&demands), 0)
        })
    });
    // 剪枝多工坊搜索，不排序
    group.bench_function(BenchmarkId::new("AdvSimSolverUnorder", 1), |b| {
        b.iter(|| {
            SolverDual::solve_unordered(&adv_solver, black_box(&limit), black_box(&demands), 4)
        })
    });
    // 剪枝多工坊搜索，不排序
    group.bench_function(BenchmarkId::new("AdvSimSolverBest", 1), |b| {
        b.iter(|| {
            SolverDual::solve_best(&adv_solver, black_box(&limit), black_box(&demands), 4)
        })
    });
    group.finish();
}

/// 测试整周求解器
fn gsolver_benchmark(c: &mut Criterion) {
    let empty = vec![];
    let mut limit = make_limit(&empty);
    limit.max_result = 100;

    let (repo, info, pat) = from_pattern_code(b"DTCyaMw1lSFijBrKtXSxNppEaKkXeXuMSzWCertGSRcyVHaYugwAAAAA");
    let mild_solver = MildSolver::new(&repo, info);
    let radical_solver = RadicalSolver::new(&repo, info);

    let mut group: criterion::BenchmarkGroup<criterion::measurement::WallTime> =
        c.benchmark_group("gpred");
    // Mild 版本，剪枝+求解排列组合
    group.bench_function(BenchmarkId::new("Mild_GSolver", 1), |b| {
        b.iter(|| GSolver::solve(&mild_solver, black_box(&limit), black_box(&pat)))
    });
    // Radical 版本，对最高可能队列排列组合
    group.bench_function(BenchmarkId::new("Radical_GSolver", 1), |b| {
        b.iter(|| GSolver::solve(&radical_solver, black_box(&limit), black_box(&pat)))
    });
    group.finish()
}

/// 测试模拟器性能
fn simulate_benchmark(c: &mut Criterion) {
    let ban = [];
    let (repo, info, _) = make_config(1, &ban);

    let mut recipe = vec![];
    for s in [14, 49, 14, 49] {
        recipe.push(repo.state(s, 9));
    }

    let recipes = [
        (
            2,
            [
                Some(repo.state(14, 9)),
                Some(repo.state(49, 9)),
                Some(repo.state(14, 9)),
                Some(repo.state(49, 9)),
                None,
                None,
            ],
        ),
        (
            2,
            [
                Some(repo.state(59, 9)),
                Some(repo.state(58, 9)),
                Some(repo.state(57, 9)),
                Some(repo.state(58, 9)),
                Some(repo.state(57, 9)),
                None,
            ],
        ),
    ];

    let mut group: criterion::BenchmarkGroup<criterion::measurement::WallTime> =
        c.benchmark_group("simulate");

    // 单作业模拟器
    group.bench_function(BenchmarkId::new("Single", 4), |b| {
        b.iter(|| simulate_batch_seq(&info, black_box(&recipe)))
    });
    // 多作业模拟器
    group.bench_function(BenchmarkId::new("Multi", 4), |b| {
        b.iter(|| simulate_multi_batch(&info, black_box(&recipes)))
    });
}

/// 测试单日多种类模拟器
fn solver_multi_benchmark(c: &mut Criterion) {
    let ban = [];
    let (repo, info, limit) = make_config(1, &ban);

    let demands = vec![9; 82];
    let mut recipe = vec![];
    for s in [14, 49, 14, 49] {
        recipe.push(repo.state(s, 9));
    }
    let solver = BFSolver::new(&repo, info);
    let set = [(3, [14, 49, 14, 49, 0, 0])];

    let mut group: criterion::BenchmarkGroup<criterion::measurement::WallTime> =
        c.benchmark_group("solver_multi");

    // 多种类，不排序
    group.bench_function(BenchmarkId::new("solver", 1), |b| {
        b.iter(|| {
            SolverWithBatch::solve_unordered(
                &solver,
                &limit,
                black_box(&set),
                black_box(&demands),
                2,
            )
        })
    });
}

#[cfg(unix)]
mod pref;

#[cfg(unix)]
fn get_criterion() -> Criterion {
    use pref::FlamegraphProfiler;
    Criterion::default().with_profiler(FlamegraphProfiler::new(100))
}

#[cfg(windows)]
fn get_criterion() -> Criterion {
    Criterion::default()
}

criterion_group! {
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = get_criterion();
    targets = predition_benchmark, gsolver_benchmark, simulate_benchmark, solver_multi_benchmark
}
criterion_main!(benches);
