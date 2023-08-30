use base64::{engine::general_purpose, Engine as _};
use mji_craftwork::{
    data::{CraftworkInfo, GameDataRepo},
    predition::DemandPattern,
};

use super::new_repo;

pub fn from_pattern_code(code: &[u8]) -> (GameDataRepo, CraftworkInfo, Vec<DemandPattern>) {
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
