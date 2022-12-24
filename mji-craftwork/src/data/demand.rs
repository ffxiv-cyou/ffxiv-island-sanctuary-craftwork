/// 市场需求
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Demand {
    /// 严重供少于求
    VeryHigh,
    /// 供少于求
    High,
    /// 供求增长
    Average,
    /// 供过于求
    Low,
    /// 严重供过于求
    VeryLow
}

impl Demand {
    /// 真实需求值上限，包括本数
    pub fn upper_bound(&self) -> i16 {
        match self {
            Demand::VeryHigh => i16::MAX,
            Demand::High => 17,
            Demand::Average => 9,
            Demand::Low => 1,
            Demand::VeryLow => -7,
        }
    }
    /// 真实需求值下限，包括本数
    pub fn lower_bound(&self) -> i16 {
        match self {
            Demand::VeryHigh => 18,
            Demand::High => 10,
            Demand::Average => 2,
            Demand::Low => -6,
            Demand::VeryLow => -999,
        }
    }

    /// 根据真实需求值映射到区间
    pub fn from_val(val: i16) -> Self {
        match val {
            18..=i16::MAX => Self::VeryHigh,
            10..=17 => Self::High,
            2..=9 => Self::Average,
            -6..=1 => Self::Low,
            i16::MIN..=-7 => Self::VeryLow
        }
    }

    /// 系数，按百分比计算
    pub fn factor(&self) -> u8 {
        match self {
            Demand::VeryHigh => 160,
            Demand::High => 130,
            Demand::Average => 100,
            Demand::Low => 80,
            Demand::VeryLow => 60,
        }
    }
}

impl From<u8> for Demand {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::VeryHigh,
            1 => Self::High,
            2 => Self::Average,
            3 => Self::Low,
            4 => Self::VeryLow,
            _ => panic!()
        }
    }
}

impl From<Demand> for u8 {
    fn from(val: Demand) -> Self {
        match val {
            Demand::VeryHigh => 0,
            Demand::High => 1,
            Demand::Average => 2,
            Demand::Low => 3,
            Demand::VeryLow => 4,
        }
    }
}

/// 需求变化量
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum DemandChange {
    /// 需求大幅降低
    MassiveDecrease,
    /// 需求降低
    Decerease,
    /// 无变化
    Equal,
    /// 需求增加
    Increase,
    /// 需求大幅增加
    MassiveIncrease,
}

impl From<u8> for DemandChange {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::MassiveDecrease,
            1 => Self::Decerease,
            2 => Self::Equal,
            3 => Self::Increase,
            4 => Self::MassiveIncrease,
            _ => panic!()
        }
    }
}

impl From<DemandChange> for u8 {
    fn from(val: DemandChange) -> Self {
        match val {
            DemandChange::MassiveDecrease => 0,
            DemandChange::Decerease => 1,
            DemandChange::Equal => 2,
            DemandChange::Increase => 3,
            DemandChange::MassiveIncrease => 4,
        }
    }
}