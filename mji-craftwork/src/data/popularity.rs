/// 产物受欢迎程度
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Popularity {
    /// 未知
    Unknown,
    /// 人气火爆
    VeryHigh,
    /// 略受瞩目
    High,
    /// 普普通通
    Average,
    /// 无人问津
    Low
}

impl Popularity {
    /// 系数，按百分比计算
    pub fn factor(&self) -> u8 {
        match self {
            Popularity::Unknown => 0,
            Popularity::VeryHigh => 140,
            Popularity::High => 120,
            Popularity::Average => 100,
            Popularity::Low => 80,
        }
    }
}

impl From<u8> for Popularity {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Unknown,
            1 => Self::VeryHigh,
            2 => Self::High,
            3 => Self::Average,
            4 => Self::Low,
            _ => Self::Unknown
        }
    }
}

impl From<Popularity> for u8 {
    fn from(p: Popularity) -> Self {
        match p {
            Popularity::Unknown => 0,
            Popularity::VeryHigh => 1,
            Popularity::High => 2,
            Popularity::Average => 3,
            Popularity::Low => 4,
        }
    }
}
