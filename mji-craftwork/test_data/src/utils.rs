use mji_craftwork::{
    data::{CraftworkInfo, GameDataRepo, Popularity, Recipe},
    solver::SolveLimit,
};

use super::{CRAFT_OBJECTS, POPULARITY_LIST};

/// 获取配方列表
pub fn recipes() -> Vec<Recipe> {
    let mut recpies = vec![];
    for i in 0..CRAFT_OBJECTS.len() {
        let rec = Recipe {
            id: CRAFT_OBJECTS[i][0] as u8,
            theme1: CRAFT_OBJECTS[i][2] as u8,
            theme2: CRAFT_OBJECTS[i][3] as u8,
            level: CRAFT_OBJECTS[i][13] as u8,
            craft_time: CRAFT_OBJECTS[i][14] as u8,
            value: CRAFT_OBJECTS[i][15],
            cost: 0,
        };
        recpies.push(rec);
    }
    recpies
}

/// 获取欢迎度列表
pub fn popularity() -> Vec<Vec<Popularity>> {
    let mut pop_vec = vec![vec![]];
    for r in POPULARITY_LIST {
        let mut v = vec![];
        for i in 1..r.len() {
            v.push(Popularity::from(r[i]));
        }
        pop_vec.push(v)
    }
    pop_vec
}

/// 新建一个repo
pub fn new_repo(pop: usize) -> GameDataRepo {
    let recpies = recipes();
    let pop_vec = popularity();
    let mut repo = GameDataRepo::new(recpies, pop_vec);
    repo.set_popular_pattern(pop);
    repo
}

/// 默认全9的需求
pub fn empty_demands() -> Vec<i8> {
    vec![9; 82]
}

/// 新建一个通用求解限制
pub fn make_limit(ban: &[u8]) -> SolveLimit {
    SolveLimit::new(16, ban, 24, false)
}

/// 新建一个通用工房信息
pub fn make_info() -> CraftworkInfo {
    CraftworkInfo::new(0, 35, 2, 3)
}

/// 使用最常用配置创建库
pub fn make_config(pop: usize, ban: &[u8]) -> (GameDataRepo, CraftworkInfo, SolveLimit) {
    let repo = new_repo(pop);
    let info = make_info();
    let limit = make_limit(&ban);

    (repo, info, limit)
}
