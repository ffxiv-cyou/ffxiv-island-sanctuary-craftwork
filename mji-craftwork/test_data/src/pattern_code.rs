use base64::{engine::general_purpose, Engine as _};
use mji_craftwork::{
    data::{CraftworkInfo, GameDataRepo},
    predition::DemandPattern,
    solver::Batches,
};

use super::{make_info, new_repo};

pub fn from_pattern_code(code: &[u8]) -> (GameDataRepo, CraftworkInfo, Vec<DemandPattern>) {
    let arr = &general_purpose::URL_SAFE_NO_PAD.decode(code).unwrap();

    let repo = new_repo(arr[0] as usize);
    let info = make_info();

    let mut pat = vec![];
    for b in &arr[1..] {
        pat.push(DemandPattern::from(b & 0x0f));
        pat.push(DemandPattern::from(b >> 4));
    }

    (repo, info, pat)
}

pub fn to_pattern_code(pop: u8, pat: &[DemandPattern]) -> String {
    let mut arr = vec![];
    arr.push(pop);

    for i in (0..pat.len()).step_by(2) {
        let v = pat[i] as u8;
        let v2 = pat[i + 1] as u8;
        arr.push((v2 << 4) + v);
    }

    general_purpose::URL_SAFE_NO_PAD.encode(arr)
}

pub fn to_plan_code(plan: &[Option<Batches>]) -> String {
    let mut arr = vec![0x80];
    arr.push(0);

    for day in plan {
        match day {
            Some(batch) => {
                let index = arr.len();
                arr.push(batch.batches.len() as u8);

                for (worker, batch) in batch.batches {
                    if worker == 0 {
                        arr[index] -= 1;
                    } else {
                        arr.push((worker << 4) + batch.seq);
                        for i in 0..batch.seq as usize {
                            arr.push(batch.steps[i]);
                        }
                    }
                }
            }
            None => arr.push(0),
        };
    }
    general_purpose::URL_SAFE_NO_PAD.encode(arr)
}
