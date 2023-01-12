/// 步骤的可能解
#[derive(Debug)]
pub struct Batch {
    /// 步骤总数
    pub seq: usize,
    /// 步骤
    pub steps: [u16; 6],
    /// 单价
    pub values: [u16; 6],
    /// 总价
    pub value: u16,
    /// 总耗时
    pub time: u16,
}

impl Batch {
    pub fn new() -> Self {
        Batch {
            seq: 0,
            steps: [0; 6],
            values: [0; 6],
            value: 0,
            time: 0,
        }
    }

    /// 获取上一个步骤的ID
    pub fn last(&self) -> u16 {
        if self.seq == 0 {
            0
        } else {
            self.steps[self.seq - 1]
        }
    }

    /// 添加一个新步骤
    pub fn push(&self, id: u16, mut val: u16, time: u16) -> Self {
        if self.seq >= self.steps.len() {
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
            time: self.time + time,
        };
        ret.steps[self.seq] = id;
        ret.values[self.seq] = val;
        ret
    }

    /// 获取指定物品的产量
    pub fn get_produce(&self, id: u16) -> u8 {
        let mut count = 0;
        for i in 0..self.seq {
            if self.steps[i] == id {
                count += match i {
                    0 => 1,
                    _ => 2,
                }
            }
        }
        return count;
    }

    pub fn get_time(&self) -> u16 {
        self.time
    }
    pub fn get_val(&self) -> u16 {
        self.value
    }
    pub fn get_steps(&self) -> &[u16] {
        &self.steps
    }
    pub fn get_values(&self) -> &[u16] {
        &self.values
    }
}

impl PartialOrd for Batch {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let a = self.value as usize * 10 + self.seq;
        let b = other.value as usize * 10 + other.seq;
        b.partial_cmp(&a)
    }
}

impl PartialEq for Batch {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.seq == other.seq
    }
}

impl Ord for Batch {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a = self.value as usize * 10 + self.seq;
        let b = other.value as usize * 10 + other.seq;
        b.cmp(&a)
    }
}
impl Eq for Batch {}
