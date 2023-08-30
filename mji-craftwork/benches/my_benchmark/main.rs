use base64::{engine::general_purpose, Engine as _};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use mji_craftwork::{
    data::{CraftworkInfo, GameDataRepo, IDataRepo, Popularity, Recipe},
    gsolver::{GSolver, MildSolver, RadicalSolver},
    predition::DemandPattern,
    simulator::{simulate_batch_seq, simulate_multi_batch},
    solver::{BFSolver, SimplifySolver, SolveLimit, SolverSingle, SolverWithBatch, AdvancedSimplifySolver, SolverDual},
};

mod test_data;
use test_data::{CRAFT_OBJECTS, POPULARITY_LIST};

fn load_data<const T: usize>() -> GameDataRepo {
    let mut recpies = vec![];

    for i in 0..usize::min(CRAFT_OBJECTS.len(), T) {
        let rec = Recipe {
            id: CRAFT_OBJECTS[i][0],
            theme1: CRAFT_OBJECTS[i][2] as u8,
            theme2: CRAFT_OBJECTS[i][3] as u8,
            level: CRAFT_OBJECTS[i][13] as u8,
            craft_time: CRAFT_OBJECTS[i][14] as u8,
            value: CRAFT_OBJECTS[i][15],
            cost: 0,
        };
        recpies.push(rec);
    }

    let mut pop_vec = vec![vec![]];
    for r in POPULARITY_LIST {
        let mut pop: Vec<Popularity> = vec![];
        for i in 1..usize::min(r.len(), T + 1) {
            pop.push(r[i].into());
        }
        pop_vec.push(pop);
    }
    GameDataRepo::new(recpies, pop_vec)
}

fn make_config(ban: &[u16]) -> (GameDataRepo, CraftworkInfo, SolveLimit) {
    let mut repo = load_data::<82>();
    repo.set_popular_pattern(1);
    let info = CraftworkInfo::new(0, 35, 2, 3);
    let limit = SolveLimit::new(16, ban, 24, false);
    (repo, info, limit)
}

/// 测试单日单种类求解器
fn predition_benchmark(c: &mut Criterion) {
    let empty = vec![];
    let (repo, info, limit) = make_config(&empty);
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
    let (repo, info, mut limit) = make_config(&empty);
    limit.max_result = 200;

    let mild_solver = MildSolver::new(&repo, info);
    let radical_solver = RadicalSolver::new(&repo, info);

    let code = &general_purpose::URL_SAFE_NO_PAD
        .decode("DTCyaMw1lSFijBrKtXSxNppEaKkXeXuMSzUCAAAAAAAAAAAAAAA")
        .unwrap();
    let mut pat = vec![];
    for b in &code[1..] {
        pat.push(DemandPattern::from(b & 0x0f));
        pat.push(DemandPattern::from(b >> 4));
    }

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
    let (repo, info, _) = make_config(&ban);

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
    let (repo, info, limit) = make_config(&ban);

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
