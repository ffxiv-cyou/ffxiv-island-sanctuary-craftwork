use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use mji_craftwork::{
    data::IDataRepo,
    gsolver::{GSolver, MildSolver, RadicalSolver},
    simulator::{simulate_batch_seq, simulate_multi_batch},
    solver::{
        AdvancedSimplifySolver, BFSolver, SimplifySolver, SolverCtx, SolverDual, SolverSingle,
        SolverWithBatch,
    },
};
use test_data::{from_pattern_code, make_config, make_limit};

/// 测试单日单种类求解器
fn predition_benchmark(c: &mut Criterion) {
    let (repo, info, limit) = make_config(1, &[]);
    let ctx = SolverCtx::new(&repo, info, limit);

    let solver = BFSolver::new();
    let mut adv_solver = AdvancedSimplifySolver::new(solver);
    let mut sim_solver = SimplifySolver::new();
    let mut solver = BFSolver::new();

    // 求解单个
    let mut limit1 = limit.clone();
    limit1.max_result = 1;
    let ctx1 = SolverCtx::new(&repo, info, limit1);

    let demands = vec![9; 82];
    let mut group = c.benchmark_group("predition");
    // 暴力搜索，排序前100
    group.bench_function(BenchmarkId::new("BFSolver", 100), |b| {
        b.iter(|| SolverSingle::solve(&mut solver, black_box(&ctx), black_box(&demands), 0))
    });
    // 暴力搜索，排序前1
    group.bench_function(BenchmarkId::new("BFSolver", 1), |b| {
        b.iter(|| SolverSingle::solve(&mut solver, black_box(&ctx1), black_box(&demands), 0))
    });
    // 暴力搜索，不排序
    group.bench_function(BenchmarkId::new("BFSolverUnorder", 1), |b| {
        b.iter(|| {
            SolverSingle::solve_unordered(&mut solver, black_box(&ctx), black_box(&demands), 0)
        })
    });
    // 暴力搜索，取最优
    group.bench_function(BenchmarkId::new("BFSolverBest", 1), |b| {
        b.iter(|| SolverSingle::solve_best(&mut solver, black_box(&ctx), black_box(&demands), 0))
    });
    // 剪枝搜索，不排序
    group.bench_function(BenchmarkId::new("SimplifySolverUnorder", 1), |b| {
        b.iter(|| {
            SolverSingle::solve_unordered(&mut sim_solver, black_box(&ctx), black_box(&demands), 0)
        })
    });
    // 剪枝多工坊搜索，不排序
    group.bench_function(BenchmarkId::new("AdvSimSolverUnorder", 1), |b| {
        b.iter(|| {
            SolverDual::solve_unordered(&mut adv_solver, black_box(&ctx), black_box(&demands), 4)
        })
    });
    // 剪枝多工坊搜索，不排序
    group.bench_function(BenchmarkId::new("AdvSimSolverBest", 1), |b| {
        b.iter(|| SolverDual::solve_best(&mut adv_solver, black_box(&ctx), black_box(&demands), 4))
    });
    group.finish();
}

/// 测试整周求解器
fn gsolver_benchmark(c: &mut Criterion) {
    let mut limit = make_limit(&[]);
    limit.max_result = 100;

    let (repo, info, pat) =
        from_pattern_code(b"DTCyaMw1lSFijBrKtXSxNppEaKkXeXuMSzWCertGSRcyVHaYugwAAAAA");
    let ctx = SolverCtx::new(&repo, info, limit);
    let mut mild_solver = MildSolver::new();
    let mut radical_solver = RadicalSolver::new();

    let mut group: criterion::BenchmarkGroup<criterion::measurement::WallTime> =
        c.benchmark_group("gpred");
    // Mild 版本，剪枝+求解排列组合
    group.bench_function(BenchmarkId::new("Mild_GSolver", 1), |b| {
        b.iter(|| GSolver::solve(&mut mild_solver, black_box(&ctx), black_box(&pat)))
    });
    // Radical 版本，对最高可能队列排列组合
    group.bench_function(BenchmarkId::new("Radical_GSolver", 1), |b| {
        b.iter(|| GSolver::solve(&mut radical_solver, black_box(&ctx), black_box(&pat)))
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
    let (repo, info, limit) = make_config(1, &[]);
    let ctx = SolverCtx::new(&repo, info, limit);

    let demands = vec![9; 82];
    let mut recipe = vec![];
    for s in [14, 49, 14, 49] {
        recipe.push(repo.state(s, 9));
    }
    let mut solver = BFSolver::new();
    let set = [(3, [14, 49, 14, 49, 0, 0])];

    let mut group: criterion::BenchmarkGroup<criterion::measurement::WallTime> =
        c.benchmark_group("solver_multi");

    // 多种类，不排序
    group.bench_function(BenchmarkId::new("solver", 1), |b| {
        b.iter(|| {
            SolverWithBatch::solve_unordered(
                &mut solver,
                &ctx,
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
