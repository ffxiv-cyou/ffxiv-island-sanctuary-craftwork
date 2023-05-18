use crate::data::{Demand, DemandChange};

use super::DemandPattern;

/// 真实需求表
const REAL_DEMAND_TABLE: [[i8; 7]; 12] = [
    [16, 24, 9, 9, 9, 9, 9],
    [13, 17, 7, 7, 7, 7, 7],
    [9, 16, 24, 9, 9, 9, 9],
    [9, 13, 17, 7, 7, 7, 7],
    [9, 9, 16, 24, 9, 9, 9],
    [9, 9, 13, 17, 7, 7, 7],
    [9, 9, 9, 16, 24, 9, 9],
    [9, 9, 9, 13, 17, 7, 7],
    [9, 10, 2, 9, 16, 24, 9],
    [9, 10, 5, 8, 13, 17, 7],
    [9, 10, 2, 2, 9, 16, 24],
    [9, 10, 2, 5, 8, 13, 17],
];

/// 根据指定的pattern获取需求
///
/// day 从0开始
pub fn get_demand(pattern: DemandPattern, day: u8, _offset: u8) -> i8 {
    if day >= 7 || pattern == DemandPattern::Unknown {
        return 9;
    }

    let index: u8 = pattern.into();
    let index = (index as usize) - 1;
    REAL_DEMAND_TABLE[index][day as usize].into()
}

/// 根据变动Pattern获取需求
pub fn get_demands(pattern: &[DemandPattern], day: u8) -> Vec<i8> {
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
                DemandChange::Equal => return DemandPattern::Day2Strong,
                DemandChange::Increase => return DemandPattern::Day2Weak,
                // 2F or 2P
                DemandChange::MassiveDecrease
                | DemandChange::MassiveIncrease
                | DemandChange::Decerease => {
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
            if Demand::from_val(REAL_DEMAND_TABLE[j][i] as i16) != demand {
                candidates[j] = false;
            } else if i > 0 {
                let delta = REAL_DEMAND_TABLE[j][i] - REAL_DEMAND_TABLE[j][i - 1];
                if DemandChange::from_val(delta as i16) != change {
                    candidates[j] = false;
                }
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

/// （高级）预测物品需求，返回所有可能的Pattern
///
/// - seq 为当周从第一天开始的需求和变动
/// - last_demand 为上周最后一天的实际需求
///
/// 返回值为可能的Pattern的Bitmap
pub fn predict_adv(seq: &[(Demand, DemandChange)], last_demand: i8) -> u16 {
    if seq.len() > 7 {
        return 0;
    }

    let mut candidates: u16 = 0b1111_1111_1111;
    for i in 0..seq.len() {
        let (demand, change) = seq[i];

        // 筛选当前
        for j in 0..12 {
            let mask = 1 << j;
            if candidates & mask == 0 {
                continue;
            }
            let pred_demand = REAL_DEMAND_TABLE[j][i];
            let pred_dem = Demand::from_val(pred_demand as i16);

            // 检查变动后的需求值是否匹配指定模式
            if pred_dem != demand {
                // 第一和第二天使用本值，其他时间可能受做的东西影响导致出现变化
                if pred_dem < demand || i < 2 {
                    candidates &= !mask;
                    continue;
                }
            }

            // 前一天的需求值
            let last_demand = match i {
                0 => last_demand,
                _ => REAL_DEMAND_TABLE[j][i - 1],
            };

            // 计算需求变动量
            let delta = pred_demand - last_demand;
            if DemandChange::from_val(delta as i16) != change {
                candidates &= !mask;
                continue;
            }
        }

        // 唯一解或者无解都直接返回
        if candidates.count_ones() <= 1 {
            return candidates;
        }
    }
    // 信息量不够，返回已有的预测
    return candidates;
}

/// 批量预测
///
/// days 指单个物品有多少预测天数
/// seqs 是需求和需求变化的数组，按单个物品的各天的变动优先排序。
pub fn predict_all(seqs: &[(Demand, DemandChange)], days: usize) -> Vec<DemandPattern> {
    let mut result = vec![];
    for i in (0..seqs.len()).step_by(days) {
        result.push(predict(&seqs[i..i + days]));
    }
    result
}
