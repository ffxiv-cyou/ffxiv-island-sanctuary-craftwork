use data::new_repo;
use mji_craftwork::{
    data::CraftworkInfo,
    gsolver::{GSolver, MildSolver},
    predition::DemandPattern,
    solver::SolveLimit,
};

mod data;

#[test]
fn test_gsolver() {
    let repo = new_repo(19);
    let info = CraftworkInfo::new(0, 35, 2, 3);
    let solver = MildSolver::new(&repo, info);
    
    let ban = vec![];
    let mut limit = SolveLimit::new(10, &ban, 24, false);
    limit.max_result = 500;

    let data = vec![
        0, 11, 12, 3, 3, 4, 1, 11, 12, 5, 2, 4, 7, 7, 12, 2, 1, 9, 1, 6, 2, 6, 11, 6, 8, 4, 9, 4,
        11, 7, 3, 3, 6, 2, 7, 10, 1, 10, 12, 12, 8, 10, 5, 5, 8, 5, 11, 9, 10, 9, 8, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ];
    let pat = DemandPattern::from_u8(&data);
    let result = solver.solve(&limit, &pat);
    println!("{:?}", result)
}
