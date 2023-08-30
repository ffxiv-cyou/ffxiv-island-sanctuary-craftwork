use super::{Demand, Popularity};

/// 单个配方
#[derive(Clone, Copy)]
pub struct Recipe {
    /// 配方ID
    pub id: u16,
    /// 主题1
    pub theme1: u8,
    /// 主题2
    pub theme2: u8,
    /// 配方等级
    pub level: u8,
    /// 制造时长
    pub craft_time: u8,
    /// 产物基础价格
    pub value: u16,
    /// 原料价格
    pub cost: u16,
}

impl Recipe {
    pub fn new() -> Self {
        Recipe {
            id: 0,
            theme1: 0,
            theme2: 0,
            level: 0,
            craft_time: 0,
            value: 0,
            cost: 0,
        }
    }

    pub fn from_array(arr: &[u16]) -> Self {
        Recipe {
            id: arr[0],
            theme1: arr[1] as u8,
            theme2: arr[2] as u8,
            level: arr[3] as u8,
            craft_time: arr[4] as u8,
            value: arr[5],
            cost: arr[6],
        }
    }
}

/// 配方状态
#[derive(Debug, Clone, Copy)]
pub struct RecipeState {
    /// 配方ID
    id: u16,
    /// 产品收益
    value: u16,
    /// 原料成本
    cost: u16,
    /// 工作时间
    time: u8,
    /// 需求值
    demand: i8,
    /// 欢迎度
    popularity: Popularity,
}

impl RecipeState {
    pub fn new(recipe: &Recipe, demand: i8, popularity: Popularity) -> Self {
        Self {
            id: recipe.id,
            value: recipe.value,
            cost: recipe.cost,
            time: recipe.craft_time,
            demand,
            popularity,
        }
    }
    /// 空值
    pub fn empty() -> Self {
        Self {
            id: 0,
            value: 0,
            cost: 0,
            time: 0,
            demand: 0,
            popularity: Popularity::Average,
        }
    }
    /// 配方系数
    ///
    /// 系数 = 欢迎度系数 * 需求系数
    pub fn factor(&self, demand_sub: i16) -> f32 {
        (Demand::from_val(self.demand as i16 - demand_sub).factor() as u16
            * self.popularity.factor() as u16) as f32
            / 10000f32
    }
    pub fn value(&self) -> u16 {
        self.value
    }
    pub fn cost(&self) -> u16 {
        self.cost
    }
    pub fn id(&self) -> u16 {
        self.id
    }
    pub fn craft_time(&self) -> u8 {
        self.time
    }
}
