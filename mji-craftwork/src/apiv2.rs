use std::convert::TryInto;

use crate::{
    data::{CraftworkInfo, GameDataRepo},
    gsolver::{GMultiSolver, GSolver, MildMulitSolver, MildSolver},
    init_repo,
    predition::DemandPattern,
    simulate_multi,
    solver::{
        AdvancedSimplifySolver, BFSolver, SolveLimit, SolverCtx, SolverDual, SolverWithBatch,
    },
};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct APIv2 {
    repo: GameDataRepo,
    day_solver_single: BFSolver,
    day_solver_dual: AdvancedSimplifySolver<BFSolver>,
    week_solver_single: MildSolver,
    week_solver_dual: MildMulitSolver<AdvancedSimplifySolver<BFSolver>>,
}

/// 初始化 V2 版本的 API
///
/// 传入数据格式如下：
/// - recipes: 配方，以 id theme1 theme2 level time value cost 格式排列
/// - pop_pattern: 欢迎度模式，一维数组形式的二维数组
/// - pop_pattern_row: 欢迎度模式单行长度
#[wasm_bindgen]
pub fn init_api_v2(recipes: Vec<u16>, pop_pattern: Vec<u8>, pop_pattern_row: usize) -> APIv2 {
    let repo = init_repo(recipes, pop_pattern, pop_pattern_row);

    APIv2 {
        repo,
        day_solver_single: BFSolver::new(),
        day_solver_dual: AdvancedSimplifySolver::new(BFSolver::new()),
        week_solver_single: MildSolver::new(),
        week_solver_dual: MildMulitSolver::new(AdvancedSimplifySolver::new(BFSolver::new())),
    }
}

#[wasm_bindgen]
impl APIv2 {
    /// 设置当前人气状况
    ///
    /// 传入由数据包解析的人气状况模式ID
    pub fn set_pattern(&mut self, pattern: usize) {
        self.repo.set_popular_pattern(pattern)
    }

    /// 模拟排班表
    ///
    /// 传入排班序列 [(workers, steps[6])]
    ///
    /// 传出总成本和各个步骤的价格: [(cost, value[6])]
    pub fn simulate(&self, state: &CraftworkInfo, seq: &[u8], demands: &[i8]) -> Vec<u16> {
        simulate_multi(&self.repo, state, seq, demands)
    }

    /// 在已配置工房的情况下，搜索此状态下的单工房最优解
    ///
    /// - set: 已设置的工房，结构为 [工坊数量, 序列[6]]
    ///
    /// 返回所有可能的解的数组，数组结构如下：
    /// - value 总价
    /// - cost 成本
    /// - prev_value 之前工坊的总价
    /// - prev_cost 之前工坊的成本
    /// - step_count 步骤数目
    /// - steps[6] 每一步的物品ID
    /// - values[6] 每一步的价格
    pub fn solve_day_with_batch(
        &mut self,
        state: &CraftworkInfo,
        level: u8,
        ban_list: &[u8],
        set: &[u8],
        demands: &[i8],
        worker: u8,
        time: u8,
        with_cost: bool,
    ) -> Vec<u16> {
        let limit = SolveLimit::new(level, ban_list, time, with_cost);
        let ctx = SolverCtx::new(&self.repo, state.clone(), limit);

        let mut sets = vec![];
        for i in (0..set.len()).step_by(7) {
            sets.push((set[i], set[i + 1..i + 7].try_into().unwrap()));
        }

        let batches =
            SolverWithBatch::solve(&mut self.day_solver_single, &ctx, &sets, demands, worker);

        let mut ret = vec![];
        for b in batches {
            ret.push(b.value);
            ret.push(b.cost);
            ret.push(b.batch.get_val());
            ret.push(b.batch.get_cost());
            ret.push(b.batch.seq as u16);
            for i in b.batch.get_steps() {
                ret.push(*i as u16);
            }
            ret.extend_from_slice(b.batch.get_values());
        }
        ret
    }

    /// 搜索单日多工坊下的可能最优解
    ///
    /// 返回所有可能解的数组，数组结构如下
    /// - value 总价
    /// - cost 成本
    /// - count 不同种类工坊个数
    ///   - worker 工坊个数
    ///   - value 此工坊单价
    ///   - cost 此工坊成本
    ///   - step_count 步骤数目
    ///   - steps[6] 每一步的物品ID
    ///   - values[6] 每一步的价格
    pub fn solve_day_dual(
        &mut self,
        state: &CraftworkInfo,
        level: u8,
        ban_list: &[u8],
        demands: &[i8],
        worker: u8,
        time: u8,
        with_cost: bool,
    ) -> Vec<u16> {
        let limit = SolveLimit::new(level, ban_list, time, with_cost);
        let ctx = SolverCtx::new(&self.repo, state.clone(), limit);

        let results = self.day_solver_dual.solve(&ctx, demands, worker);
        let mut ret = vec![];
        for b in results {
            ret.push(b.value);
            ret.push(b.cost);

            let index = ret.len();
            ret.push(b.batches.len() as u16);

            for (worker, batch) in b.batches {
                // 跳过空的
                if worker == 0 {
                    ret[index] -= 1;
                    continue;
                }

                ret.push(worker as u16);
                ret.push(batch.value as u16);
                ret.push(batch.cost as u16);
                ret.push(batch.seq as u16);
                for i in batch.get_steps() {
                    ret.push(*i as u16);
                }
                ret.extend_from_slice(batch.get_values());
            }
        }
        ret
    }

    /// 在每日工房使用相同的物品的情况下，尝试求解整周
    ///
    /// 传入的是需求趋势模式数组
    pub fn solve_week_single(
        &mut self,
        state: &CraftworkInfo,
        level: u8,
        ban_list: &[u8],
        time: u8,
        with_cost: bool,
        pattern: &[u8],
    ) -> Vec<u16> {
        let limit = SolveLimit::new(level, ban_list, time, with_cost);
        let ctx = SolverCtx::new(&self.repo, state.clone(), limit);

        let vec = DemandPattern::from_u8(pattern);
        let batches = self.week_solver_single.solve(&ctx, &vec);

        let mut ret = vec![];
        for b in batches {
            match b {
                None => {
                    ret.push(0); // val
                    ret.push(0); // cost
                    ret.push(0); // seq
                    ret.extend_from_slice(&[0, 0, 0, 0, 0, 0]); // steps
                    ret.extend_from_slice(&[0, 0, 0, 0, 0, 0]); // values
                }
                Some(batch) => {
                    ret.push(batch.get_val());
                    ret.push(batch.get_cost());
                    ret.push(batch.seq as u16);
                    for i in batch.get_steps() {
                        ret.push(*i as u16);
                    }
                    ret.extend_from_slice(batch.get_values());
                }
            }
        }
        ret
    }

    /// 清理求解器的缓存
    pub fn solver_clear_cache(&mut self) {
        self.week_solver_dual.clear_cache();
    }

    /// 在每日工房使用至多两种物品的情况下，尝试求解整周
    ///
    /// - pattern: 需求趋势模式数组
    /// - part_id: 当前解分块Index
    ///
    /// 返回值：
    /// - val: 用于比较的值
    /// - batches: []
    ///   - value: 收益
    ///   - cost: 成本
    ///   - workers: 有多少不同种的worker
    ///     - worker
    ///     - seq
    ///     - steps[]
    ///     - values[]
    pub fn solve_week_part(
        &mut self,
        state: &CraftworkInfo,
        level: u8,
        ban_list: &[u8],
        time: u8,
        with_cost: bool,
        pattern: &[u8],
        part_id: u16,
    ) -> Vec<u16> {
        let limit = SolveLimit::new(level, ban_list, time, with_cost);
        let ctx = SolverCtx::new(&self.repo, state.clone(), limit);

        let vec = DemandPattern::from_u8(pattern);
        let (batches, val) = self
            .week_solver_dual
            .solve_part(&ctx, &vec, part_id as usize);

        let mut ret = vec![val];
        for b in batches {
            match b {
                None => {
                    ret.push(0); // value
                    ret.push(0); // cost
                    ret.push(0); // worker types
                }
                Some(batch) => {
                    ret.push(batch.value);
                    ret.push(batch.cost);

                    let index = ret.len();
                    ret.push(batch.batches.len() as u16);

                    for (worker, batch) in batch.batches {
                        // 跳过空的
                        if worker == 0 {
                            ret[index] -= 1;
                            continue;
                        }
                        ret.push(worker as u16);
                        ret.push(batch.seq as u16);
                        for i in batch.get_steps() {
                            ret.push(*i as u16);
                        }
                        ret.extend_from_slice(batch.get_values());
                    }
                }
            }
        }
        ret
    }
}
