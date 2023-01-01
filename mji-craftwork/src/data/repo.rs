use wasm_bindgen::prelude::wasm_bindgen;

use super::{Popularity, Recipe, RecipeState};

pub trait IDataRepo {
    fn recipe(&self, id: usize) -> &Recipe;
    fn popular(&self, id: usize) -> Popularity;
    fn demand(&self, id: usize) -> i8;
    fn recipe_len(&self) -> usize;

    fn state(&self, id: usize) -> RecipeState {
        RecipeState::new(self.recipe(id), self.demand(id), self.popular(id))
    }

    fn foreach<U, V>(&self, pred: U, mut callback: V)
    where
        U: Fn(&Recipe) -> bool,
        V: FnMut(&Recipe, RecipeState),
    {
        let len = self.recipe_len();
        for i in 0..len {
            let recipe = self.recipe(i);
            if pred(recipe) {
                callback(recipe, self.state(i))
            }
        }
    }
}

#[wasm_bindgen]
pub struct GameDataRepo {
    recipes: Vec<Recipe>,
    demands: Vec<i8>,
    popularity: Vec<Vec<Popularity>>,

    popular_pattern: usize
}

impl GameDataRepo {
    pub fn new(recipes: Vec<Recipe>, pops: Vec<Vec<Popularity>>) -> Self {
        Self {
            recipes: recipes,
            demands: vec![9; 62],
            popularity: pops,
            popular_pattern: 0
        }
    }
}
impl GameDataRepo {
    pub fn set_popular_pattern(&mut self, pat: usize) {
        self.popular_pattern = pat;
    }

    pub fn set_demands(&mut self, demands: &[i8]) {
        self.demands.clear();
        self.demands.extend_from_slice(demands);
    }
}

impl IDataRepo for GameDataRepo {
    fn recipe(&self, id: usize) -> &Recipe {
        &self.recipes[id]
    }

    fn popular(&self, id: usize) -> Popularity {
        self.popularity[self.popular_pattern][id]
    }

    fn demand(&self, id: usize) -> i8 {
        self.demands[id]
    }

    fn recipe_len(&self) -> usize {
        self.recipes.len()
    }
}