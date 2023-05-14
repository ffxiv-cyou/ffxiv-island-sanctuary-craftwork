use mji_craftwork::{data::{Demand, DemandChange}, pattern_demand, pattern_predict, predition::{DemandPattern, predict_adv}, pattern_predict_adv};

#[test]
pub fn pattern() {
    let vec = vec![
        0x10, 0x00, 0x24, // 2F
        0x22, 0x10, 0x00, // 3F
        0x22, 0x11, 0x11, // 3P
    ];
    let result = pattern_predict(&vec, 3);
    assert_eq!(result.len(), 3);
    assert_eq!(result[0], DemandPattern::Day2Strong.into());
    assert_eq!(result[1], DemandPattern::Day3Strong.into());
    assert_eq!(result[2], DemandPattern::Day3Weak.into());
}

#[test]
pub fn predict_adv_test() {
    let seq0 = vec![
        9, 0x10, 0, // 上周 9，本周第一天1/2, 2强
        9, 0x11, 0, // 上周 9，本周第一天1/1, 2弱
        17, 0x13, 0, // 上周 17, 本周第一天 1/-1, 是2的任意一个
        24, 0x14, 0, // 
    ];
    let result = pattern_predict_adv(&seq0, 1);
    assert_eq!(result[0], DemandPattern::Day2Strong.into());
    assert_eq!(result[1], DemandPattern::Unknown.into());
    assert_eq!(result[2], DemandPattern::Day2Weak.into());
    assert_eq!(result[3], DemandPattern::Unknown.into());

    println!("{:?}", result);
}

#[test]
pub fn demand() {
    let vec = vec![
        DemandPattern::Day3Strong.into(),
        DemandPattern::Day3Weak.into(),
    ];
    let result = pattern_demand(&vec, 2);
    assert_eq!(result.len(), 2);
    assert_eq!(Demand::from_val(result[0] as i16), Demand::VeryHigh.into());
    assert_eq!(Demand::from_val(result[1] as i16), Demand::High.into());
}
