#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::PetApp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct Pet {
    pub name: String,
    pub age: i8,
    pub hunger: i8,
    pub fitness: i8,
    pub children: Vec<String>,
}

impl Default for Pet {
    fn default() -> Self {
        Self {
            name: "Rusty".to_owned(),
            age: 0,
            hunger: 0,
            fitness: 10,
            children: Vec::new(),
        }
    }
}

impl Pet {
    pub fn is_alive(&self) -> bool {
        self.hunger < 10 && self.fitness >= 0 && self.age < 30
    }

    pub fn adopt_child(&mut self, child_name: String) {
        self.children.push(child_name);
    }
    pub fn status(&self) -> String {
        format!(
            "Age: {} Hunger: {} Fitness: {}",
            self.age, self.hunger, self.fitness
        )
    }
    pub fn feed(&mut self) {
        if self.hunger - 3 < 0 {
            self.hunger = 0;
        } else {
            self.hunger -= 3
        }
    }
    pub fn walk(&mut self) {
        if self.fitness + 3 > 10 {
            self.fitness = 10
        } else {
            self.fitness += 3
        }
    }
    pub fn grow_up(&mut self) {
        self.age += 1;
        self.hunger += 3;
        self.fitness -= 3;
    }
}
