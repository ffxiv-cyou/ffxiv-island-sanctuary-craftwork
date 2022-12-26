use mji_craftwork::{
    data::{Demand,},
    pattern_demand, pattern_predict,
    predition::DemandPattern,
};

#[test]
pub fn pattern() {
    let vec = vec![
        0x22, 0x14, 0x04, // 3F
        0x22, 0x13, 0x13, // 3P
    ];
    let result = pattern_predict(&vec, 3);
    assert_eq!(result.len(), 2);
    assert_eq!(result[0], DemandPattern::Day3Strong.into());
    assert_eq!(result[1], DemandPattern::Day3Weak.into());
}

#[test]
pub fn demand() {
    let vec = vec![
        DemandPattern::Day3Strong.into(),
        DemandPattern::Day3Weak.into(),
    ];
    let result = pattern_demand(&vec, 2);
    assert_eq!(result.len(), 2);
    assert_eq!(result[0], Demand::VeryHigh.into());
    assert_eq!(result[1], Demand::High.into());
}
