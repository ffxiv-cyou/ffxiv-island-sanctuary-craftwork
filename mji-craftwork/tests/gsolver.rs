use mji_craftwork::{
    data::CraftworkInfo,
    gsolver::{GSolver, MildSolver, RadicalSolver, MildMulitSolver, GMultiSolver, print_week_result},
    predition::DemandPattern,
    simulator::Batch,
    solver::SolveLimit,
};
use test_data::{new_repo,from_pattern_code, to_pattern_code, to_plan_code, make_limit};
use rand::prelude::Distribution;
use rand::seq::SliceRandom;

fn new_limit<'a>() -> SolveLimit<'a> {
    let mut limit = SolveLimit::new(10, &[], 24, false);
    limit.max_result = 1000;

    limit
}

fn batch_value(batches: &[Option<Batch>; 6]) -> u16 {
    let mut sum_val = 0;
    for i in batches {
        match i {
            None => (),
            Some(b) => sum_val += b.value,
        }
    }
    sum_val
}

#[test]
fn test_gsolver_mild() {
    let limit = new_limit();

    let code = b"DTCyaMw1lSFijBrKtXSxNppEaKkXeXuMSzUCAAAAAAAAAAAAAA";
    let (repo, info, pat) = from_pattern_code(code);

    let solver = MildSolver::new(&repo, info);

    let result = solver.solve(&limit, &pat);
    let sum_val = batch_value(&result);
    println!("total: {}, {:?}", sum_val * info.workers as u16, result)
}

#[test]
fn test_gsolver_radical() {
    let limit = new_limit();

    let code = b"SJByzIRRxbqHJzZmScUxUrO5qTSBS2ocorcIAAAAAAA";
    let (repo, info, pat) = from_pattern_code(code);

    let solver = RadicalSolver::new(&repo, info);

    let result = solver.solve(&limit, &pat);
    let sum_val = batch_value(&result);

    println!("total: {}, {:?}", sum_val * info.workers as u16, result)
}

#[test]
fn test_gsolver_compare() {
    let limit = new_limit();
    let mut repo = new_repo(1);
    let info = CraftworkInfo::new(0, 35, 2, 3);

    let pop_range = rand::distributions::Uniform::new(1, 100);
    let mut rng = rand::thread_rng();

    let mut rand = [
        0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7, 8,
        8, 8, 8, 9, 9, 9, 9, 10, 10, 10, 10, 11, 11, 11, 11, 11, 12, 12, 12, 12, 12, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ];

    let mut cnt_mild = 0;
    let mut cnt_radical = 0;
    let mut cnt_equal = 0;

    for i in 0..100 {
        let pop = pop_range.sample(&mut rng);
        repo.set_popular_pattern(pop);

        let sli = &mut rand[1..51];
        sli.shuffle(&mut rng);
        let pat = DemandPattern::from_u8(&rand);

        let mild = MildSolver::new(&repo, info);
        let radical = RadicalSolver::new(&repo, info);

        let mild_batch = mild.solve(&limit, &pat);
        let radical_batch = radical.solve(&limit, &pat);

        let mild_val = batch_value(&mild_batch);
        let radical_val = batch_value(&radical_batch);

        println!("{}: mild {}, radical {}", i, mild_val, radical_val);
        if mild_val > radical_val {
            cnt_mild += 1;
            println!("code: {}", to_pattern_code(pop as u8, &pat));
        } else if mild_val < radical_val {
            cnt_radical += 1;
        } else {
            cnt_equal += 1;
        }
    }

    println!(
        "mild {}, rad {}, equal {}",
        cnt_mild, cnt_radical, cnt_equal
    );
}

#[test]
fn test_gsolver_mild_multi() {
    let limit = make_limit(&[]);
    // let limit = SolveLimit::new(12, &[], 24, false);

    let (repo, mut info, pat) = from_pattern_code(b"QVBGh7oanMZyE7uYaTJSIXq3xFZKyVg8QYtTmshkMhu3qFnBIwcAAAAA");
    // let (repo, mut info, pat) = from_pattern_code(b"AQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
    info.workers = 4;
    let mut solver = MildMulitSolver::new(&repo, info);

    let result = solver.solve(&limit, &pat);
    println!("code: {}", to_plan_code(&result));
    print_week_result(&result);
}