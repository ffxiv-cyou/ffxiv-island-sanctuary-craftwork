use std::collections::BinaryHeap;

use crate::{
    data::{CraftworkInfo, IDataRepo},
    simulator::simulate,
};

use super::{Batch, SolveLimit, Solver};

pub struct BFSolver<'a, T>
where
    T: IDataRepo,
{
    info: CraftworkInfo,
    data: &'a T,
}

impl<'a, T> Solver for BFSolver<'a, T>
where
    T: IDataRepo,
{
    fn solve(&self, limit: &SolveLimit, demands: &[i8]) -> Vec<Batch> {
        let mut ret: Vec<Batch> = vec![];
        self.solve_sub(limit, demands, &mut ret, Batch::new(), self.info.clone());
        // 结果排序
        let mut heap = BinaryHeap::new();
        for mut item in ret {
            item.set_cmp_value(limit.with_cost);
            if heap.len() >= limit.max_result {
                heap.pop();
            }
            heap.push(item)
        }
        heap.into_sorted_vec()
    }

    fn update_info(&mut self, info: CraftworkInfo) {
        self.info = info
    }
}

impl<'a, T> BFSolver<'a, T>
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
    ) {
        let last = current.last();
        let first_batch = last == 0;
        let last = self.data.recipe(last as usize);
        let remain = 24 - current.get_time();

        // 连击开始后即增加干劲
        if !first_batch {
            info = info.next()
        }

        self.data.foreach(|r| {
            if r.craft_time == 0 || remain < r.craft_time as u16 {
                return;
            }
            if !limit.check(r) || last.id == r.id {
                return;
            }
            // 连击过滤
            let batch_check = first_batch
                || (last.theme1 != 0 && last.theme1 == r.theme1)
                || (last.theme2 != 0 && last.theme2 == r.theme2)
                || (last.theme1 == 0 && last.theme2 == 0);
            if !batch_check {
                return;
            }

            let id = r.id as usize;
            let s = self.data.state(id, demands[id]);
            let demand_sub = current.get_produce(s.id()) * info.workers;
            let val = simulate(&info, &s, demand_sub);
            let current = current.push(r.id, val, r.cost, r.craft_time as u16);

            if current.get_time() > 20 {
                // 当前工序结束
                vec.push(current)
            } else {
                self.solve_sub(limit, demands, vec, current, info)
            }
        });
    }
}
