use std::cmp::min;

use wasm_bindgen::prelude::wasm_bindgen;

/// 猫耳小员委托Item
#[derive(Clone, Copy, Debug)]
#[wasm_bindgen]
pub struct Favor {
    /// 配方ID
    pub id: u8,
    /// 需求数量
    pub num: u8,
}

impl Favor {
    pub fn new(id: u8, num: u8) -> Self {
        Favor {
            id, num
        }
    }

    pub fn from_array(arr: &[u8]) -> Self {
        Self::new(arr[0], arr[1])
    }

    pub fn contains(arr: &[Self], id: u8) -> bool {
        for i in arr {
            if i.id == id {
                return true;
            }
        }
        return false;
    }
}

pub struct Favors<'a, const T: usize> {
    pub items: &'a [Favor],
    sum: [u8; T]
}

impl<'a, const T: usize> Favors<'a, T> {
    pub fn new(favors: &'a [Favor]) -> Self {
        Self{
            items: favors,
            sum: [0; T]
        }
    }

    pub fn add(&mut self, id: u8, count: u8) {
        for i in 0..min(T, self.items.len()) {
            if self.items[i].id == id {
                self.sum[i] += count
            }
        }
    }

    pub fn value(&self) -> u8 {
        let mut sum = 0;
        for i in 0..min(T, self.items.len()) {
            sum += min(self.sum[i], self.items[i].num);
        }
        sum
    }
}