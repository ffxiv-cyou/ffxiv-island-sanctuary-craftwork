mod demand;
mod popularity;
mod recipe;
mod repo;

pub use demand::{Demand, DemandChange};
pub use popularity::Popularity;
pub use recipe::{Recipe, RecipeState};
pub use repo::{GameDataRepo, IDataRepo};
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
    /// 每次连击增加的干劲
    pub workers: u8,
}

#[wasm_bindgen]
impl CraftworkInfo {
    /// 工坊状态
    ///
    /// workers 表示同时运行多少个当前队列，一般为1或3。此参数影响连击干劲增加量
    #[wasm_bindgen(constructor)]
    pub fn new(tension: u8, max_tension: u8, level: u8, workers: u8) -> Self {
        CraftworkInfo {
            tension,
            max_tension,
            level,
            workers,
        }
    }
}
impl CraftworkInfo {
    /// 当前工坊系数
    pub fn factor(&self) -> f32 {
        let tension = u8::min(self.tension, self.max_tension);
        (1.0 + self.level as f32 * 0.1) * (1.0 + tension as f32 * 0.01)
    }
    /// 等级系数
    pub fn factor_level(&self) -> f32 {
        1.0 + self.level as f32 * 0.1
    }
    /// 干劲系数
    pub fn factor_tension(&self) -> f32 {
        let tension = u8::min(self.tension, self.max_tension);
        1.0 + tension as f32 * 0.01
    }

    /// 增加对应工坊数量的干劲
    pub fn next(self) -> Self {
        self.add_tension(self.workers)
    }

    /// 增加指定点数的干劲
    pub fn add_tension(self, n: u8) -> Self {
        CraftworkInfo {
            tension: u8::saturating_add(self.tension, n),
            max_tension: self.max_tension,
            level: self.level,
            workers: self.workers,
        }
    }
}
