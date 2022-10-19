use crate::diet::Diet;

#[derive(Debug)]
pub struct Cookbook {
    recipes: Vec<Recipe>,
}

impl Cookbook {
    pub fn new() -> Cookbook {
        let chowder = Recipe::new(String::from("chowder"), Diet::Shellfish);
        let cioppino = Recipe::new(String::from("cioppino"), Diet::Fish);

        Cookbook {
            recipes: vec![chowder, cioppino],
        }
    }

    pub fn recipes(&self) -> std::slice::Iter<Recipe> {
        self.recipes.iter()
    }
}

#[derive(Debug)]
pub struct Recipe {
    name: String,
    diet: Diet,
}

impl Recipe {
    pub fn new(name: String, diet: Diet) -> Recipe {
        Recipe { name, diet }
    }

    pub fn diet(&self) -> Diet {
        self.diet
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}
