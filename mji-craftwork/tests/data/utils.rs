use mji_craftwork::{
    data::{GameDataRepo, Popularity, Recipe},
    predition::DemandPattern,
};

use super::{CRAFT_OBJECTS, POPULARITY_LIST};

/// 获取配方列表
pub fn recipes() -> Vec<Recipe> {
    let mut recpies = vec![];
    for i in 0..CRAFT_OBJECTS.len() {
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

pub fn new_repo(pop: usize) -> GameDataRepo {
    let recpies = recipes();
    let pop_vec = popularity();
    let mut repo = GameDataRepo::new(recpies, pop_vec);
    repo.set_popular_pattern(pop);
    repo
}
