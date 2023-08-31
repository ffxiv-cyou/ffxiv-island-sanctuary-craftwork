/// 步骤的可能解
#[derive(Debug, Clone, Copy)]
pub struct Batch {
    /// 步骤总数
    pub seq: u8,
    /// 步骤
    pub steps: [u8; 6],
    /// 单价
    pub values: [u16; 6],
    /// 用于比较的总价
    pub cmp_value: u16,
    /// 总价
    pub value: u16,
    /// 总成本
    pub cost: u16,
    /// 总耗时
    pub time: u8,
}

impl Batch {
    pub fn new() -> Self {
        Batch {
            seq: 0,
            steps: [0; 6],
            values: [0; 6],
            value: 0,
            cost: 0,
            time: 0,
            cmp_value: 0,
        }
    }

    /// 获取上一个步骤的ID
    pub fn last(&self) -> u8 {
        if self.seq == 0 {
            0
        } else {
            self.steps[self.seq as usize - 1]
        }
    }

    /// 添加一个新步骤
    pub fn push(&self, id: u8, mut val: u16, cost: u16, time: u8) -> Self {
        if self.seq as usize >= self.steps.len() {
            panic!("Step overflow! {:?}", self.steps)
        }

        // 非第一批次时有连击加成
        if self.seq != 0 {
            val *= 2;
        }
        let mut ret = Self {
            seq: self.seq + 1,
            steps: self.steps,
            values: self.values,
            value: self.value + val,
            cost: self.cost + cost,
            time: self.time + time,
            cmp_value: self.cmp_value,
        };
        ret.steps[self.seq as usize] = id;
        ret.values[self.seq as usize] = val;
        ret
    }

    /// 获取指定物品的产量
    pub fn get_produce(&self, id: u8) -> u8 {
        let mut count = 0;
        for i in 0..self.seq as usize {
            if self.steps[i] == id {
                count += match i {
                    0 => 1,
                    _ => 2,
                }
            }
        }
        return count;
    }

    pub fn get_time(&self) -> u8 {
        self.time
    }
    pub fn get_val(&self) -> u16 {
        self.value
    }
    pub fn get_cost(&self) -> u16 {
        self.cost
    }
    /// 设置比较用的值
    pub fn set_cmp_value(&mut self, with_cost: bool) {
        if with_cost {
            self.cmp_value = self.value - self.cost
        } else {
            self.cmp_value = self.value
        }
    }
    pub fn get_steps(&self) -> &[u8] {
        &self.steps
    }
    pub fn get_values(&self) -> &[u16] {
        &self.values
    }
}

impl PartialOrd for Batch {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let a = self.cmp_value as usize * 10 + self.seq as usize;
        let b = other.cmp_value as usize * 10 + other.seq as usize;
        b.partial_cmp(&a)
    }
}

impl PartialEq for Batch {
    fn eq(&self, other: &Self) -> bool {
        self.cmp_value == other.cmp_value && self.seq == other.seq
    }
}

impl Ord for Batch {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a = self.cmp_value as usize * 10 + self.seq as usize;
        let b = other.cmp_value as usize * 10 + other.seq as usize;
        b.cmp(&a)
    }
}
impl Eq for Batch {}
