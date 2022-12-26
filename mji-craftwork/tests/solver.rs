use mji_craftwork::{data::{GameDataRepo, Demand, Popularity, Recipe, CraftworkInfo}, solver::{BFSolver, Solver, SolveLimit}, init_repo, set_demands, solve_singleday, simulate, };

mod test_data;
use test_data::{CRAFT_OBJECTS, POPULARITY_LIST, DEMAND_LIST};

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
        };
        recpies.push(rec);
    }

    let mut pop_vec = vec![];
    for r in POPULARITY_LIST {
        let mut pop: Vec<Popularity> = vec![];
        for i in 1..usize::min(r.len(), T+1) {
            pop.push(r[i].into());
        }
        pop_vec.push(pop);
    }
    let mut repo = GameDataRepo::new(recpies, pop_vec);

    let current_pop = DEMAND_LIST[0];
    repo.set_popular_pattern(current_pop as usize);
    // let next_pop = DEMAND_LIST[1];

    let demands = vec![9; T];
    repo.set_demands(&demands);
    repo
}

fn load_data_init() -> GameDataRepo {
    let mut recipes = vec![];

    for i in 0..CRAFT_OBJECTS.len() {
        recipes.push(CRAFT_OBJECTS[i][0] as u16);
        recipes.push(CRAFT_OBJECTS[i][2] as u16);
        recipes.push(CRAFT_OBJECTS[i][3] as u16);
        recipes.push(CRAFT_OBJECTS[i][13] as u16);
        recipes.push(CRAFT_OBJECTS[i][14] as u16);
        recipes.push(CRAFT_OBJECTS[i][15] as u16);
    }
    let mut pop_vec = vec![];
    for r in POPULARITY_LIST {
        for i in 1..r.len() {
            pop_vec.push(r[i] as u8);
        }
    }

    init_repo(recipes, pop_vec, 62)
}

fn load_test_demand(repo: &mut GameDataRepo) {
    let demands = vec![9; 62];
    set_demands(repo, &demands);
}

#[test]
fn init_test() {
    let mut repo = load_data_init();
    load_test_demand(&mut repo);
    
    solve_singleday(&repo, &CraftworkInfo::new(0, 30, 1, 1), 10, vec![]);
}

#[test]
fn predict() {
    let repo = load_data::<51>();
    let solver = BFSolver::new(&repo, CraftworkInfo::new(0, 35, 1, 1));
    let empty= vec![];
    let limit = SolveLimit::new(10, &empty);
    let result = solver.solve(&limit);
    println!("length: {}", result.len());
    for i in 0..20 {
        println!("val: {},{:?}, {:?}", result[i].get_val(), result[i].get_steps(), result[i].get_values());
    }
}

#[test]
fn simulator() {
    let mut repo = load_data_init();
    load_test_demand(&mut repo);

    let state = CraftworkInfo::new(0, 35, 1, 1);
    let arr = vec![13, 23, 13, 23];
    let result = simulate(&repo, &state, arr);
    println!("{:?}", result);
    assert_ne!(result[0], 0);
    assert_ne!(result[1], 0);
    assert_ne!(result[2], 0);
    assert_ne!(result[3], 0);
}