pub mod data;
pub mod gsolver;
pub mod predition;
pub mod simulator;
pub mod solver;
pub mod utils;

use data::{Demand, DemandChange, GameDataRepo, IDataRepo, Recipe, RecipeState};
use gsolver::{MildSolver, GSolver};
use predition::{get_demands, predict_all, DemandPattern, predict_adv};
use solver::{BFSolver, SolveLimit, Solver};
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
/// - recipes: 配方，以 id theme1 theme2 level time value cost 格式排列
/// - pop_pattern: 欢迎度模式，一维数组形式的二维数组
/// - pop_pattern_row: 欢迎度模式单行长度
#[wasm_bindgen]
pub fn init_repo(recipes: Vec<u16>, pop_pattern: Vec<u8>, pop_pattern_row: usize) -> GameDataRepo {
    console_error_panic_hook::set_once();

    let mut recp = vec![];
    let mut pops = vec![];

    for i in (0..recipes.len()).step_by(7) {
        recp.push(Recipe::from_array(&recipes[i..i + 7]));
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

/// 模拟排班表
///
/// 传入排班序列后，传出的是各阶段的收入。
///
/// 传出的数据结构如下：
/// cost value[6]
#[wasm_bindgen]
pub fn simulate(
    repo: &GameDataRepo,
    state: &CraftworkInfo,
    seq: Vec<u8>,
    demands: &[i8],
) -> Vec<u16> {
    let mut recp = Vec::<RecipeState>::new();
    for s in seq {
        recp.push(repo.state(s as usize, demands[s as usize]));
    }

    let batch = simulator::simulate_batch(state, &recp);
    let mut ret = Vec::<u16>::new();
    ret.push(batch.get_cost());
    ret.extend_from_slice(batch.get_values());

    ret
}

/// 搜索单日的最优解
///
/// 返回所有可能的解的数组，数组结构如下
/// - value 总价
/// - cost 成本
/// - step_count 步骤数目
/// - steps[6] 每一步的物品ID
/// - values[6] 每一步的价格
#[wasm_bindgen]
pub fn solve_singleday(
    repo: &GameDataRepo,
    state: &CraftworkInfo,
    level: u8,
    ban_list: Vec<u16>,
    demands: &[i8],
    time: u16,
    with_cost: bool,
) -> Vec<u16> {
    let solver = BFSolver::new(repo, state.clone());
    let limit = SolveLimit::new(level, &ban_list, time, with_cost);
    let batches = solver.solve(&limit, demands);

    let mut ret = vec![];
    for b in batches {
        ret.push(b.get_val());
        ret.push(b.get_cost());
        ret.push(b.seq as u16);
        ret.extend_from_slice(b.get_steps());
        ret.extend_from_slice(b.get_values());
    }
    ret
}

/// 预测需求曲线
///
/// 传入的 array 为需求变动的二维数组转一维表示，单个物品多日的变动优先。
/// 高4位为需求，低4位为需求变动（与数据包格式保持一致）。
///
/// 返回曲线的模式，其中：
/// - 0: 未知
/// - 1: 2强
/// - 2: 2弱
/// - 3: 3强
/// - 以此类推
#[wasm_bindgen]
pub fn pattern_predict(array: &[u8], days: usize) -> Vec<u8> {
    let mut seqs = vec![];
    for i in 0..array.len() {
        let byte = array[i];
        let demand: Demand = (byte >> 4).into();
        let change: DemandChange = (byte & 0x0F).into();
        seqs.push((demand, change))
    }

    let pat = predict_all(&seqs, days);
    let mut result = vec![];
    for i in pat {
        result.push(i.into());
    }

    result
}

/// 预测需求变动模式
/// 
/// 传入的Array按以下顺序排布
/// - 0: 上周第七天的真实需求值
/// - 1: 第一天的需求与变动
/// - 2: 第二天的需求与变动
/// - 4: ...
/// 
/// 单个物品Array的长度为 days+1
/// 
/// 返回需求变动模式的数组，每个物品占用2bytes，分别代表两种可能的需求模式
#[wasm_bindgen]
pub fn pattern_predict_adv(array: &[u8], days: usize) -> Vec<u16> {
    let mut result = vec![];

    for i in (0..array.len()).step_by(days + 1) {
        let last_demand = array[i] as i8;
        let mut seqs = vec![];
        for j in 0..days {
            let byte = array[i + j + 1];
            let demand: Demand = (byte >> 4).into();
            let change: DemandChange = (byte & 0x0F).into();
            seqs.push((demand, change));
        }

        let pats = predict_adv(&seqs, last_demand);
        result.push(pats)
    }
    result
}

/// 根据需求曲线预测指定日期的需求
///
/// day从0开始
#[wasm_bindgen]
pub fn pattern_demand(array: &[u8], day: u8) -> Vec<i8> {
    let mut pats = vec![];
    for &i in array {
        pats.push(i.into());
    }
    let demands = get_demands(&pats, day);
    let mut result = vec![];
    for i in demands {
        result.push(i.into());
    }
    result
}

/// 尝试自动解整周的数据
///
/// 传入的是需求趋势模式数组
#[wasm_bindgen]
pub fn solve_week(
    repo: &GameDataRepo,
    state: &CraftworkInfo,
    level: u8,
    ban_list: Vec<u16>,
    time: u16,
    with_cost: bool,
    pattern: &[u8],
) -> Vec<u16> {
    let limit = SolveLimit::new(level, &ban_list, time, with_cost);
    let solver = MildSolver::new(repo, state.clone());
    
    let vec = DemandPattern::from_u8(pattern);
    let batches = solver.solve(&limit, &vec);

    let mut ret = vec![];
    for b in batches {
        match b {
            None => {
                ret.push(0); // val
                ret.push(0); // cost
                ret.push(0); // seq
                ret.extend_from_slice(&[0,0,0,0,0,0]); // steps
                ret.extend_from_slice(&[0,0,0,0,0,0]); // values
            },
            Some(batch) => {
                ret.push(batch.get_val());
                ret.push(batch.get_cost());
                ret.push(batch.seq as u16);
                ret.extend_from_slice(batch.get_steps());
                ret.extend_from_slice(batch.get_values());
            },
        }
    }
    ret
}
