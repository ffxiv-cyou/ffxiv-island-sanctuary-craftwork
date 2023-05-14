mod pattern;
mod pred;
pub use pattern::DemandPattern;
pub use pred::{get_demands, predict_all, predict, predict_adv};
