use crate::{
    data::{CraftworkInfo, IDataRepo},
    simulator::simulate,
};

use super::{Batch, SolveLimit, SolverSingle};

/// Simplify 简易剪枝
///
/// 此方法在暴力求解的过程中，动态对部分可能解做剪枝。
///
/// 当一个配方大于等于12小时后，计算其每小时收益，并与最高收益对比。
/// 若其收益小于最高收益的69%，则将其剪枝。
///
/// 此方法不保证所有解一定正确，仅能尽量保证最高几个的解是正确的。
///
/// 69%是通过实测得出的，对于其他配方可能不适用。
pub struct SimplifySolver<'a, T>
where
    T: IDataRepo,
{
    info: CraftworkInfo,
    data: &'a T,
}

impl<'a, T> SolverSingle for SimplifySolver<'a, T>
where
    T: IDataRepo,
{
    fn solve_unordered(&self, limit: &SolveLimit, demands: &[i8], workers: u8) -> Vec<Batch> {
        let mut ret: Vec<Batch> = vec![];
        let mut avg = 0;
        let mut info = self.info.clone();
        if workers != 0 {
            info.workers = workers;
        }

        self.solve_sub(
            limit,
            demands,
            &mut ret,
            Batch::new(),
            info,
            &mut avg,
        );
        ret
    }

    fn update_info(&mut self, info: CraftworkInfo) {
        self.info = info
    }
}

impl<'a, T> SimplifySolver<'a, T>
where
    T: IDataRepo,
{
    /// 新建一个暴力搜索
    pub fn new(data: &'a T, info: CraftworkInfo) -> Self {
        Self {
            info: info,
            data: data,
        }
    }

    fn solve_sub(
        &self,
        limit: &SolveLimit,
        demands: &[i8],
        vec: &mut Vec<Batch>,
        current: Batch,
        mut info: CraftworkInfo,
        max: &mut u16,
    ) {
        let last = current.last();
        let first_batch = last == 0;
        let last = self.data.recipe(last as usize);
        let remain = limit.time - current.get_time();

        // 连击开始后即增加干劲
        if !first_batch {
            info = info.next()
        }

        self.data.foreach(|r| {
            if r.craft_time == 0 || remain < r.craft_time {
                return;
            }
            if !limit.check(r) || last.id == r.id {
                return;
            }
            // 连击过滤
            let batch_check = first_batch
                || (last.theme1 != 0 && (last.theme1 == r.theme1 || last.theme1 == r.theme2))
                || (last.theme2 != 0 && (last.theme2 == r.theme1 || last.theme2 == r.theme2))
                || (last.theme1 == 0 && last.theme2 == 0);
            if !batch_check {
                return;
            }

            let id = r.id as usize;
            let s = self.data.state(id, demands[id]);
            let demand_sub = current.get_produce(s.id()) * info.workers;
            let val = simulate(&info, &s, demand_sub);
            let current = current.push(r.id, val, r.cost, r.craft_time);

            if current.get_time() >= 12 {
                let avg = current.value / current.get_time() as u16;

                // 平均收益不达标，剪枝
                if avg < (*max as f32 * 0.69) as u16 {
                    return;
                }
            }

            if current.get_time() > limit.time - 4 {
                // 当前工序结束
                vec.push(current);

                let avg = current.value / limit.time as u16;
                if avg > *max {
                    *max = avg;
                }
            } else {
                self.solve_sub(limit, demands, vec, current, info, max)
            }
        });
    }
}
