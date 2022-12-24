mod demand;
mod popularity;
mod recipe;
mod repo;

pub use demand::{Demand, DemandChange};
pub use popularity::Popularity;
pub use recipe::{Recipe, RecipeState};
pub use repo::{DataRepo, GameDataRepo, IDataRepo};
use wasm_bindgen::prelude::wasm_bindgen;

/// 工坊当前状态
#[derive(Clone, Copy, Debug)]
#[wasm_bindgen]
pub struct CraftworkInfo {
    /// 当前干劲
    pub tension: u8,
    /// 最高干劲
    pub max_tension: u8,
    /// 当前工坊等级，取值为0-2
    pub level: u8,
}

#[wasm_bindgen]
impl CraftworkInfo {
    /// 工坊状态
    #[wasm_bindgen(constructor)]
    pub fn new(tension: u8, max_tension: u8, level: u8) -> Self {
        CraftworkInfo {
            tension,
            max_tension,
            level,
        }
    }
}
impl CraftworkInfo {
    /// 当前工坊系数
    pub fn factor(&self) -> f32 {
        let tension = u8::min(self.tension, self.max_tension);
        (1.0 + self.level as f32 * 0.1) * (1.0 + tension as f32 * 0.01)
    }

    /// 增加一点干劲
    pub fn next(self) -> Self {
        CraftworkInfo {
            tension: u8::saturating_add(self.tension, 1),
            max_tension: self.max_tension,
            level: self.level,
        }
    }
}
