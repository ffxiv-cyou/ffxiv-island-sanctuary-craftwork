use mji_craftwork::{data::{Demand}, pattern_demand, pattern_predict, predition::{DemandPattern, predict_adv}, pattern_predict_adv};

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
        9, 0x10, // 上周 9，本周第一天1/2, 2强
        9, 0x11, // 上周 9，本周第一天1/1, 2弱
        17, 0x13, // 上周 17, 本周第一天 1/-1, 是2的任意一个
        24, 0x14, // 上周 24, 本周第一天 1/-2, 是2的任意一个
    ];
    let result = pattern_predict_adv(&seq0, 1);
    assert_eq!(result[0], 0b0000_0000_0001);
    assert_eq!(result[1], 0b0000_0000_0010);
    assert_eq!(result[2], 0b0000_0000_0011);
    assert_eq!(result[3], 0b0000_0000_0011);

    let seq = vec![
        24, 0x24, 0x22, 0x32, // 24, 2/-2, 2/0, 3/0, 5强/弱
        7, 0x21, 0x11, 0x34, // 7, 2/1, 1/1, 3/-2, 67强/弱
        9, 0x22, 0x11, 0x21, // 9, 2/0, 1/1, 2/1, 3弱
        17, 0x24, 0x11, 0x23, // 17, 2/-2, 1/1, 2/-1, 6弱
        7, 0x21, 0x22, 0x21, // 7, 2/1, 2/0, 2/1, 4弱
    ];
    let result = pattern_predict_adv(&seq, 3);
    assert_eq!(result[0], 0b0000_1100_0000);
    assert_eq!(result[1], 0b1101_0000_0000);
    assert_eq!(result[2], 0b0000_0000_1000);
    assert_eq!(result[3], 0b0010_0000_0000);
    assert_eq!(result[4], 0b0000_0010_0000);

    let seq = vec![
        7, 0x21, 0x22, 0x22, 0x21, // 7, 2/1, 2/0, 2/0, 2/1, 5弱
        17, 0x24, 0x11, 0x34, 0x20, // 17, 2/-2, 1/1, 3/-2, 2/2, 6强
        7, 0x21, 0x11, 0x34, 0x32, // 7, 2/1, 1/1, 3/-2, 3/0, 7强
        17, 0x24, 0x11, 0x44, 0x42, // 17, 2/-2, 1/1, 4/-2, 4/0, 7强
        24, 0x24, 0x22, 0x32, 0x20, // 24, 2/-2, 2/0, 3/0, 2/2, 5强
    ];
    let result = pattern_predict_adv(&seq, 4);
    assert_eq!(result[0], 0b0000_1000_0000);
    assert_eq!(result[1], 0b0001_0000_0000);
    assert_eq!(result[2], 0b0100_0000_0000);
    assert_eq!(result[3], 0b0100_0000_0000);
    assert_eq!(result[4], 0b0000_0100_0000);
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
