use wasm_bindgen::prelude::wasm_bindgen;

use super::{Popularity, Recipe, RecipeState};

pub trait IDataRepo {
    fn recipe(&self, id: usize) -> &Recipe;
    fn popular(&self, id: usize) -> Popularity;
    fn recipe_len(&self) -> usize;

    fn state(&self, id: usize, demand: i8) -> RecipeState {
        RecipeState::new(self.recipe(id), demand, self.popular(id))
    }

    fn foreach<V>(&self, mut callback: V)
    where
        V: FnMut(&Recipe),
    {
        let len = self.recipe_len();
        for i in 0..len {
            let recipe = self.recipe(i);
            callback(recipe)
        }
    }
}

#[wasm_bindgen]
pub struct GameDataRepo {
    recipes: Vec<Recipe>,
    popularity: Vec<Vec<Popularity>>,

    popular_pattern: usize,
}

impl GameDataRepo {
    pub fn new(recipes: Vec<Recipe>, pops: Vec<Vec<Popularity>>) -> Self {
        Self {
            recipes: recipes,
            popularity: pops,
            popular_pattern: 0,
        }
    }
}
impl GameDataRepo {
    pub fn set_popular_pattern(&mut self, pat: usize) {
        self.popular_pattern = pat;
    }
}

impl IDataRepo for GameDataRepo {
    fn recipe(&self, id: usize) -> &Recipe {
        &self.recipes[id]
    }

    fn popular(&self, id: usize) -> Popularity {
        self.popularity[self.popular_pattern][id]
    }

    fn recipe_len(&self) -> usize {
        self.recipes.len()
    }
}
