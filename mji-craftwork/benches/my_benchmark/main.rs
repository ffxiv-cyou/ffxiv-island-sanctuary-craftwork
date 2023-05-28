use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use mji_craftwork::{
    data::{CraftworkInfo, GameDataRepo, IDataRepo, Popularity, Recipe},
    gsolver::{GSolver, MildSolver, RadicalSolver},
    predition::DemandPattern,
    simulator::{simulate, simulate_batch_seq, simulate_multi_batch},
    solver::{BFSolver, SimplifySolver, SolveLimit, Solver, SolverMulti},
};
mod test_data;
use test_data::{CRAFT_OBJECTS, POPULARITY_LIST};
mod pref;
use pref::FlamegraphProfiler;

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

    let mut pop_vec = vec![];
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
    let mut repo = load_data::<73>();
    repo.set_popular_pattern(1);
    let info = CraftworkInfo::new(0, 35, 1, 1);
    let limit = SolveLimit::new(16, ban, 24, false);
    (repo, info, limit)
}

fn predition_benchmark(c: &mut Criterion) {
    let empty = vec![];
    let (repo, info, limit) = make_config(&empty);
    let solver = BFSolver::new(&repo, info);
    let sim_solver = SimplifySolver::new(&repo, info);

    let mut limit1 = limit.clone();
    limit1.max_result = 1;

    let demands = vec![9; 82];
    let mut group = c.benchmark_group("predition");
    group.bench_function(BenchmarkId::new("BFSolver", 100), |b| {
        b.iter(|| Solver::solve(&solver, black_box(&limit), black_box(&demands)))
    });
    group.bench_function(BenchmarkId::new("BFSolver", 1), |b| {
        b.iter(|| Solver::solve(&solver, black_box(&limit1), black_box(&demands)))
    });
    group.bench_function(BenchmarkId::new("BFSolverUnorder", 1), |b| {
        b.iter(|| Solver::solve_unordered(&solver, black_box(&limit), black_box(&demands)))
    });
    group.bench_function(BenchmarkId::new("BFSolverBest", 1), |b| {
        b.iter(|| Solver::solve_best(&solver, black_box(&limit), black_box(&demands)))
    });
    group.bench_function(BenchmarkId::new("SimplifySolverUnorder", 1), |b| {
        b.iter(|| Solver::solve_unordered(&sim_solver, black_box(&limit), black_box(&demands)))
    });
    group.finish();
}

fn gsolver_benchmark(c: &mut Criterion) {
    let empty = vec![];
    let (repo, info, mut limit) = make_config(&empty);
    limit.max_result = 200;

    let mild_solver = MildSolver::new(&repo, info);
    let radical_solver = RadicalSolver::new(&repo, info);

    let data = vec![
        0, 11, 12, 3, 3, 4, 1, 11, 12, 5, 2, 4, 7, 7, 12, 2, 1, 9, 1, 6, 2, 6, 11, 6, 8, 4, 9, 4,
        11, 7, 3, 3, 6, 2, 7, 10, 1, 10, 12, 12, 8, 10, 5, 5, 8, 5, 11, 9, 10, 9, 8, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ];
    let pat = DemandPattern::from_u8(&data);

    let mut group: criterion::BenchmarkGroup<criterion::measurement::WallTime> =
        c.benchmark_group("gpred");
    group.bench_function(BenchmarkId::new("Mild_GSolver", 1), |b| {
        b.iter(|| GSolver::solve(&mild_solver, black_box(&limit), black_box(&pat)))
    });
    group.bench_function(BenchmarkId::new("Radical_GSolver", 1), |b| {
        b.iter(|| GSolver::solve(&radical_solver, black_box(&limit), black_box(&pat)))
    });
    group.finish()
}

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

    group.bench_function(BenchmarkId::new("Single", 4), |b| {
        b.iter(|| simulate_batch_seq(&info, black_box(&recipe)))
    });
    group.bench_function(BenchmarkId::new("Multi", 4), |b| {
        b.iter(|| simulate_multi_batch(&info, black_box(&recipes)))
    });
}

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

    group.bench_function(BenchmarkId::new("solver", 1), |b| {
        b.iter(|| {
            SolverMulti::solve_unordered(&solver, &limit, black_box(&set), black_box(&demands), 2)
        })
    });
}

criterion_group! {
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().with_profiler(FlamegraphProfiler::new(100));
    targets = predition_benchmark, gsolver_benchmark, simulate_benchmark, solver_multi_benchmark
}
criterion_main!(benches);
