use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DemandPattern {
    Unknown,
    Day2Strong,
    Day2Weak,
    Day3Strong,
    Day3Weak,
    Day4Strong,
    Day4Weak,
    Day5Strong,
    Day5Weak,
    Day6Strong,
    Day6Weak,
    Day7Strong,
    Day7Weak,
}

impl From<u8> for DemandPattern {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Unknown,
            1 => Self::Day2Strong,
            2 => Self::Day2Weak,
            3 => Self::Day3Strong,
            4 => Self::Day3Weak,
            5 => Self::Day4Strong,
            6 => Self::Day4Weak,
            7 => Self::Day5Strong,
            8 => Self::Day5Weak,
            9 => Self::Day6Strong,
            10 => Self::Day6Weak,
            11 => Self::Day7Strong,
            12 => Self::Day7Weak,
            _ => Self::Unknown,
        }
    }
}

impl From<DemandPattern> for u8 {
    fn from(val: DemandPattern) -> Self {
        match val {
            DemandPattern::Unknown => 0,
            DemandPattern::Day2Strong => 1,
            DemandPattern::Day2Weak => 2,
            DemandPattern::Day3Strong => 3,
            DemandPattern::Day3Weak => 4,
            DemandPattern::Day4Strong => 5,
            DemandPattern::Day4Weak => 6,
            DemandPattern::Day5Strong => 7,
            DemandPattern::Day5Weak => 8,
            DemandPattern::Day6Strong => 9,
            DemandPattern::Day6Weak => 10,
            DemandPattern::Day7Strong => 11,
            DemandPattern::Day7Weak => 12,
        }
    }
}

impl DemandPattern {
    pub fn from_u8(data: &[u8]) -> Vec<DemandPattern> {
        let mut result = vec![];
        for d in data {
            result.push(DemandPattern::from(*d))
        }
        result
    }
}