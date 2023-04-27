use data::new_repo;
use mji_craftwork::{
    data::{CraftworkInfo, GameDataRepo},
    gsolver::{GSolver, MildSolver, RadicalSolver},
    predition::DemandPattern,
    solver::SolveLimit, simulator::Batch,
};
use base64::{Engine as _, engine::general_purpose};
use rand::prelude::Distribution;
use rand::seq::SliceRandom;

mod data;

fn new_limit<'a>() -> SolveLimit<'a> {
    let mut limit = SolveLimit::new(10, &[], 24, false);
    limit.max_result = 1000;

    limit
}

fn generate_from(code: &[u8]) -> (GameDataRepo, CraftworkInfo, Vec<DemandPattern>) {
    let arr = &general_purpose::URL_SAFE_NO_PAD.decode(code).unwrap();

    let repo = new_repo(arr[0] as usize);
    let info = CraftworkInfo::new(0, 35, 2, 3);

    let mut pat = vec![];
    for b in &arr[1..] {
        pat.push(DemandPattern::from(b & 0x0f));
        pat.push(DemandPattern::from(b >> 4));
    }

    (repo, info, pat)
}

fn generate_to(pop: u8, pat: &[DemandPattern]) -> String {
    let mut arr = vec![];
    arr.push(pop);

    for i in (0..pat.len()).step_by(2) {
        let v = pat[i] as u8;
        let v2 = pat[i+1] as u8;
        arr.push((v2 << 4) + v);
    }

    general_purpose::URL_SAFE_NO_PAD.encode(arr)
}

fn batch_value(batches: &[Option<Batch>;6]) -> u16 {
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
    let (repo, info, pat) = generate_from(code);
    
    let solver = MildSolver::new(&repo, info);

    let result = solver.solve(&limit, &pat);
    let sum_val = batch_value(&result);
    println!("total: {}, {:?}", sum_val * info.workers as u16, result)
}

#[test]
fn test_gsolver_radical() {
    let limit = new_limit();

    let code = b"SJByzIRRxbqHJzZmScUxUrO5qTSBS2ocorcIAAAAAAA";
    let (repo, info, pat) = generate_from(code);
    
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

    let mut rand = [0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7, 8, 8, 8, 8, 9, 9, 9, 9, 10, 10, 10, 10, 11, 11, 11, 11, 11, 12, 12, 12, 12, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

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
            println!("code: {}", generate_to(pop as u8, &pat));
        } else if mild_val < radical_val {
            cnt_radical += 1;
        } else {
            cnt_equal += 1;
        }
    }

    println!("mild {}, rad {}, equal {}", cnt_mild, cnt_radical, cnt_equal);

}