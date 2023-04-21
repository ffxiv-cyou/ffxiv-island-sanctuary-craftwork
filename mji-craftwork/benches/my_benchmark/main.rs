use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use mji_craftwork::{
    data::{CraftworkInfo, GameDataRepo, Popularity, Recipe},
    solver::{BFSolver, SolveLimit, Solver},
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
            cost: 0
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
    let mut repo = GameDataRepo::new(recpies, pop_vec);

    repo.set_popular_pattern(32);

    repo
}

fn predition_benchmark(c: &mut Criterion) {
    let repo = load_data::<51>();
    let solver = BFSolver::new(&repo, CraftworkInfo::new(0, 35, 1, 1));
    
    let empty = vec![];
    let mut limit = SolveLimit::new(10, &empty, 24, false);
    limit.max_result = 100;

    let demands = vec![9; 62];
    let mut group = c.benchmark_group("predition");
    group.bench_function(BenchmarkId::new("BFSolver", 1), 
        |b| b.iter(|| solver.solve(black_box(&limit), black_box(&demands))));
    group.finish();
}

criterion_group!{
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().with_profiler(FlamegraphProfiler::new(100));
    targets = predition_benchmark
}
criterion_main!(benches);
