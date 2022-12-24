pub mod data;
pub mod simulator;
pub mod solver;
pub mod utils;

use data::{Demand, GameDataRepo, IDataRepo, Recipe, RecipeState};
use solver::{BFSolver, Solver, SolveLimit};
use wasm_bindgen::prelude::*;

use crate::data::CraftworkInfo;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// 初始化数据
///
/// 传入数据格式如下：
/// recipes: id theme1 theme2 level time value
/// 
#[wasm_bindgen]
pub fn init_repo(recipes: Vec<u16>, pop_pattern: Vec<u8>, pop_pattern_row: usize) -> GameDataRepo {
    let mut recp = vec![];
    let mut pops = vec![];

    let recp_len = recipes.len() / 6;
    for i in 0..recp_len {
        recp.push(Recipe {
            id: recipes[i * 6 + 0],
            theme1: recipes[i * 6 + 1] as u8,
            theme2: recipes[i * 6 + 2] as u8,
            level: recipes[i * 6 + 3] as u8,
            craft_time: recipes[i * 6 + 4] as u8,
            value: recipes[i * 6 + 5],
        });
    }

    for i in 0..pop_pattern.len() {
        let r = i / pop_pattern_row;
        let c = i % pop_pattern_row;
        if c == 0 {
            pops.push(vec![]);
        }
        pops[r].push(pop_pattern[i].into())
    }

    GameDataRepo::new(recp, pops)
}

/// 设置当前人气状况
/// 
/// 传入由数据包解析的人气状况模式ID
#[wasm_bindgen]
pub fn set_pattern(repo: &mut GameDataRepo, pattern: usize) {
    repo.set_popular_pattern(pattern)
}

/// 设置需求状况
///
/// 传入一个uint8_t数组，表示各产物的需求状况
#[wasm_bindgen]
pub fn set_demands(repo: &mut GameDataRepo, data: &[u8]) {
    let mut array = Vec::<Demand>::new();
    for &d in data {
        array.push(d.into())
    }
    repo.set_demands(&array)
}

/// 模拟排班表
///
/// 传入排班序列后，传出的是各阶段的收入
#[wasm_bindgen]
pub fn simulate(repo: &GameDataRepo, state: &CraftworkInfo, seq: Vec<u8>) -> Vec<u16> {
    let mut recp = Vec::<RecipeState>::new();
    for s in seq {
        recp.push(repo.state(s as usize));
    }

    let batch = simulator::simulate_batch(state, &recp);
    let mut ret = Vec::<u16>::new();
    ret.extend_from_slice(batch.get_values());

    ret
}

/// 搜索最优解
///
/// 返回所有可能的解的数组，数组结构如下
/// value step_count steps[6]
#[wasm_bindgen]
pub fn solve_singleday(repo: &GameDataRepo, state: &CraftworkInfo, level: u8, ban_list: Vec<u16>) -> Vec<u16> {
    let solver = BFSolver::new(repo, state.clone());
    let limit = SolveLimit::new(level, &ban_list);
    let batches = solver.solve(&limit);

    let mut ret = vec![];
    for b in batches {
        ret.push(b.get_val());
        ret.push(b.seq as u16);
        ret.extend_from_slice(b.get_steps());
    }
    ret
}
