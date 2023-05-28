use super::{simulate, Batch};
use crate::data::{CraftworkInfo, RecipeState};

/// 模拟同时工作的多个工坊
///
/// - info: 工坊信息
/// - recipes: 工坊工作队列, (数量，队列)
///
/// 返回求解的值和新的状态
pub fn simulate_multi_batch(
    info: &CraftworkInfo,
    recipes: &[(u8, [Option<RecipeState>; 6])],
) -> (Vec<(u8, Batch)>, CraftworkInfo) {
    let mut batches = vec![];
    let mut states = vec![]; // index, begin_time, end_time
    let mut info = info.clone();

    let mut max_recipe = 0;
    for i in 0..recipes.len() {
        batches.push((recipes[i].0, Batch::new()));
        let time = match &recipes[i].1[0] {
            Some(r) => r.craft_time(),
            None => 0,
        };
        states.push((0, 0, time));

        // 计算产量要的数组长度
        for r in &recipes[i].1 {
            if let Some(r) = r {
                max_recipe = max_recipe.max(r.id());
            }
        }
    }
    // 产量，用于计算需求变动值
    let mut produces = vec![0; max_recipe as usize + 1];

    let mut current_time = 0;
    loop {
        // 查找最早结束的配方
        let mut first_end = 24;
        for i in 0..recipes.len() {
            let (j, beg, end) = states[i];
            // 长度为0，空配方
            if beg == end {
                continue;
            }
            // 更新干劲
            if beg == current_time && j != 0 {
                let workers = recipes[i].0;
                info = info.add_tension(workers)
            }
            // 计算首个结束的时间
            first_end = first_end.min(end)
        }
        // 如果当前时间已经过了，那么说明所有配方都结束了
        if first_end <= current_time {
            break;
        }

        // 计算配方值
        for i in 0..recipes.len() {
            let (j, beg, _) = states[i];
            if beg != current_time {
                continue;
            }
            let recipe = &recipes[i].1[j];
            if let Some(r) = recipe {
                let val = simulate(&info, r, produces[r.id() as usize]);
                batches[i].1 = batches[i]
                    .1
                    .push(r.id(), val, r.cost(), r.craft_time() as u16);
            }
        }

        // 更新产量状态
        current_time = first_end;
        for i in 0..recipes.len() {
            let (j, beg, end) = states[i];
            if end != first_end || end == beg {
                continue;
            }
            // 计算产量，更新需求值
            if let Some(r) = &recipes[i].1[j] {
                let workers = recipes[i].0;
                produces[r.id() as usize] += match j {
                    0 => 1,
                    _ => 2,
                } * workers;
            }

            // 计算新状态
            let recipe = match j + 1 < recipes[i].1.len() {
                true => &recipes[i].1[j + 1],
                false => &None,
            };
            let craft_time = match recipe {
                Some(r) => r.craft_time(),
                None => 0,
            };
            states[i] = (j + 1, end, end + craft_time);
        }
    }

    (batches, info)
}
