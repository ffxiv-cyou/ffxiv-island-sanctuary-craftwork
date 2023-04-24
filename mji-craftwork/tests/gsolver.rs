use data::new_repo;
use mji_craftwork::{
    data::CraftworkInfo,
    gsolver::{GSolver, MildSolver},
    predition::DemandPattern,
    solver::SolveLimit,
};
use base64::{Engine as _, engine::general_purpose};

mod data;

#[test]
fn test_gsolver() {
    let code = b"DTCyaMw1lSFijBrKtXSxNppEaKkXeXuMSzUCAAAAAAAAAAAAAA";
    let arr = &general_purpose::URL_SAFE_NO_PAD.decode(code).unwrap();

    let repo = new_repo(arr[0] as usize);
    let info = CraftworkInfo::new(0, 35, 2, 3);
    let solver = MildSolver::new(&repo, info);
    
    let ban = vec![];
    let mut limit = SolveLimit::new(10, &ban, 24, false);
    limit.max_result = 500;


    let mut pat = vec![];
    for b in &arr[1..] {
        pat.push(DemandPattern::from(b & 0x0f));
        pat.push(DemandPattern::from(b >> 4));
    }

    let result = solver.solve(&limit, &pat);
    let mut sum_val = 0;
    for i in result {
        match i {
            None => (),
            Some(b) => sum_val += b.value,
        }
    }
    
    println!("total: {}, {:?}", sum_val * info.workers as u16, result)
}
