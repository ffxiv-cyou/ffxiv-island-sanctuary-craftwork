use wasm_bindgen::prelude::wasm_bindgen;

use super::{Demand, Popularity, Recipe, RecipeState};

pub struct DataRepo<const T: usize> {
    recipes: [Recipe; T],
    demand: [Demand; T],
    popularity: [Popularity; T],
}

pub trait IDataRepo {
    fn recipe(&self, id: usize) -> &Recipe;
    fn popular(&self, id: usize) -> Popularity;
    fn demand(&self, id: usize) -> Demand;
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

impl<const T: usize> DataRepo<T> {
    pub fn new(recipes: [Recipe; T], demand: [Demand; T], popularity: [Popularity; T]) -> Self {
        Self {
            recipes,
            demand,
            popularity,
        }
    }
}

impl<const T: usize> IDataRepo for DataRepo<T> {
    fn recipe_len(&self) -> usize {
        T
    }
    fn recipe(&self, id: usize) -> &Recipe {
        &self.recipes[id]
    }
    fn popular(&self, id: usize) -> Popularity {
        self.popularity[id]
    }
    fn demand(&self, id: usize) -> Demand {
        self.demand[id]
    }
}

#[wasm_bindgen]
pub struct GameDataRepo {
    recipes: Vec<Recipe>,
    demands: Vec<Demand>,
    popularity: Vec<Vec<Popularity>>,

    popular_pattern: usize
}

impl GameDataRepo {
    pub fn new(recipes: Vec<Recipe>, pops: Vec<Vec<Popularity>>) -> Self {
        Self {
            recipes: recipes,
            demands: vec![Demand::Average; 62],
            popularity: pops,
            popular_pattern: 0
        }
    }
}
impl GameDataRepo {
    pub fn set_popular_pattern(&mut self, pat: usize) {
        self.popular_pattern = pat;
    }

    pub fn set_demands(&mut self, demands: &[Demand]) {
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

    fn demand(&self, id: usize) -> Demand {
        self.demands[id]
    }

    fn recipe_len(&self) -> usize {
        self.recipes.len()
    }
}