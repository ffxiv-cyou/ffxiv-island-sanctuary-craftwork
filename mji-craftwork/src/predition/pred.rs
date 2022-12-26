use crate::data::{Demand, DemandChange};

use super::DemandPattern;

/// 各种Pattern的需求表
const DEMAND_TABLE: [[u8; 7]; 12] = [
    [1, 0, 2, 2, 2, 2, 2],
    [1, 1, 2, 2, 2, 2, 2],
    [2, 1, 0, 2, 2, 2, 2],
    [2, 1, 1, 2, 2, 2, 2],
    [2, 2, 1, 0, 2, 2, 2],
    [2, 2, 1, 1, 2, 2, 2],
    [2, 2, 2, 1, 0, 2, 2],
    [2, 2, 2, 1, 1, 2, 2],
    [2, 1, 2, 2, 1, 0, 2],
    [2, 1, 2, 2, 1, 1, 2],
    [2, 1, 2, 2, 2, 1, 0],
    [2, 1, 2, 2, 2, 1, 1],
];
/// 各种Pattern的变化表
const CHANGE_TABLE: [[u8; 7]; 12] = [
    [2, 4, 0, 2, 2, 2, 2],
    [2, 3, 0, 2, 2, 2, 2],
    [2, 4, 4, 0, 2, 2, 2],
    [2, 3, 3, 0, 2, 2, 2],
    [2, 2, 4, 4, 0, 2, 2],
    [2, 2, 3, 3, 0, 2, 2],
    [2, 2, 2, 4, 4, 0, 2],
    [2, 2, 2, 3, 3, 0, 2],
    [2, 2, 0, 4, 4, 4, 0],
    [2, 2, 1, 3, 3, 3, 0],
    [2, 2, 0, 2, 4, 4, 4],
    [2, 2, 0, 3, 3, 3, 3],
];

/// 根据指定的pattern获取需求
pub fn get_demand(pattern: DemandPattern, day: u8, _offset: u8) -> Demand {
    if day >= 7 || pattern == DemandPattern::Unknown {
        return Demand::Average;
    }

    let index: u8 = pattern.into();
    let index = (index as usize) - 1;
    DEMAND_TABLE[index][day as usize].into()
}

/// 根据变动Pattern获取需求
pub fn get_demands(pattern: &[DemandPattern], day: u8) -> Vec<Demand> {
    let mut demands = vec![];
    for p in pattern {
        demands.push(get_demand(*p, day, 0));
    }
    demands
}

/// 根据已有的需求和需求变动表猜测变动的Pattern
pub fn predict(seq: &[(Demand, DemandChange)]) -> DemandPattern {
    if seq.len() > 7 {
        return DemandPattern::Unknown;
    }

    let mut candidates = [true; 12];

    for i in 0..seq.len() {
        let (demand, change) = seq[i];
        if i == 0 && demand == Demand::High {
            match change {
                DemandChange::Decerease => return DemandPattern::Day2Weak,
                DemandChange::Equal => return DemandPattern::Day2Strong,
                DemandChange::Increase => return DemandPattern::Day2Weak,
                // 2F or 2P
                DemandChange::MassiveDecrease | 
                DemandChange::MassiveIncrease => {
                    for i in 2..candidates.len() {
                        candidates[i] = false;
                    }
                }
            }
            continue;
        }

        // 筛选当前
        for j in 0..candidates.len() {
            if !candidates[j] {
                continue;
            }
            if DEMAND_TABLE[j][i] != demand.into() || CHANGE_TABLE[j][i] != change.into() {
                candidates[j] = false;
            }
        }

        // 计算剩余可能的数量
        let mut candidate_count = 0;
        let mut last_candidate = 0;
        for j in 0..candidates.len() {
            if candidates[j] {
                candidate_count += 1;
                last_candidate = j;
            }
        }

        // 唯一的可能，返回
        if candidate_count == 1 {
            return ((last_candidate as u8) + 1).into();
        }
        // 没有可能，返回未知
        if candidate_count == 0 {
            return DemandPattern::Unknown;
        }
    }

    DemandPattern::Unknown
}

/// 批量预测
/// 
/// days 指单个物品有多少预测天数
/// seqs 是需求和需求变化的数组，按单个物品的各天的变动优先排序。
pub fn predict_all(seqs: &[(Demand, DemandChange)], days: usize) -> Vec<DemandPattern> {
    let mut result = vec![];
    for i in (0..seqs.len()).step_by(days) {
        result.push(predict(&seqs[i..i+days]));
    }
    result
}