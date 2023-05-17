mod pattern;
mod pred;
pub use pattern::DemandPattern;
pub use pred::{get_demands, predict, predict_adv, predict_all};
