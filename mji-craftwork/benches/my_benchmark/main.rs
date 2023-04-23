use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use mji_craftwork::{
    data::{CraftworkInfo, GameDataRepo, Popularity, Recipe},
    solver::{BFSolver, SolveLimit, Solver, SimplifySolver}, gsolver::{MildSolver, GSolver}, predition::DemandPattern,
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

fn predition_benchmark(c: &mut Criterion) {
    let mut repo = load_data::<51>();
    repo.set_popular_pattern(1);

    let info = CraftworkInfo::new(0, 35, 1, 1);
    let solver = BFSolver::new(&repo, info);
    let sim_solver = SimplifySolver::new(&repo, info);

    let empty = vec![];
    let mut limit = SolveLimit::new(10, &empty, 24, false);
    limit.max_result = 100;
    let mut limit1 = SolveLimit::new(10, &empty, 24, false);
    limit1.max_result = 1;

    let demands = vec![9; 62];
    let mut group = c.benchmark_group("predition");
    group.bench_function(BenchmarkId::new("BFSolver", 100), |b| {
        b.iter(|| solver.solve(black_box(&limit), black_box(&demands)))
    });
    group.bench_function(BenchmarkId::new("BFSolver", 1), |b| {
        b.iter(|| solver.solve(black_box(&limit1), black_box(&demands)))
    });
    group.bench_function(BenchmarkId::new("BFSolverUnorder", 1), |b| {
        b.iter(|| solver.solve_unordered(black_box(&limit), black_box(&demands)))
    });
    group.bench_function(BenchmarkId::new("BFSolverBest", 1), |b| {
        b.iter(|| solver.solve_best(black_box(&limit), black_box(&demands)))
    });
    group.bench_function(BenchmarkId::new("SimplifySolverUnorder", 1), |b| {
        b.iter(|| sim_solver.solve_unordered(black_box(&limit), black_box(&demands)))
    });
    group.finish();
}

fn gsolver_benchmark(c: &mut Criterion) {
    let mut repo = load_data::<51>();
    repo.set_popular_pattern(19);

    let solver = MildSolver::new(&repo, CraftworkInfo::new(0, 35, 1, 1));
    let ban = vec![];
    let limit = SolveLimit::new(10, &ban, 24, false);

    let data = vec![
        0, 11, 12, 3, 3, 4, 1, 11, 12, 5, 2, 4, 7, 7, 12, 2, 1, 9, 1, 6, 2, 6, 11, 6, 8, 4, 9, 4,
        11, 7, 3, 3, 6, 2, 7, 10, 1, 10, 12, 12, 8, 10, 5, 5, 8, 5, 11, 9, 10, 9, 8, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ];
    let pat = DemandPattern::from_u8(&data);

    let mut group: criterion::BenchmarkGroup<criterion::measurement::WallTime> = c.benchmark_group("gpred");
    group.bench_function(BenchmarkId::new("MildSolver", 1), |b| {
        b.iter(|| solver.solve(black_box(&limit), black_box(&pat)))
    });
    group.finish()
}

criterion_group! {
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().with_profiler(FlamegraphProfiler::new(100));
    targets = predition_benchmark, gsolver_benchmark
}
criterion_main!(benches);
