use std::cmp::min;

use wasm_bindgen::prelude::wasm_bindgen;

use crate::simulator::Batch;

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

    pub fn add_seq(&mut self, batch: &Batch, workers: u8) {
        for i in 0..batch.seq {
            self.add(batch.steps[i as usize], match i == 0 {
                true => 1 * workers,
                false => 2 * workers,
            })
        }
    }

    pub fn value(&self) -> u8 {
        let mut sum = 0;
        for i in 0..min(T, self.items.len()) {
            sum += min(self.sum[i], self.items[i].num);
        }
        sum <<= 2;
        // 优先使用正好填满的东西
        for i in 0..min(T, self.items.len()) {
            sum += match self.items[i].num > 0 && self.sum[i] >= self.items[i].num {
                true => 1,
                false => 0,
            };
        }
        sum
    }

    pub fn to_favors(&self) -> [Favor; T] {
        let mut arr = [Favor::new(0, 0); T];
        let len = min(T, self.items.len());
        arr[0..len].clone_from_slice(self.items);

        for i in 0..len {
            arr[i].num = match arr[i].num > self.sum[i] {
                true => arr[i].num - self.sum[i],
                false => 0,
            };
        }

        arr
    }
}