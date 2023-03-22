use leptos::{Scope, SignalGet};
use serde::{Deserialize, Serialize};

use crate::recipe::Recipe;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeSerialized {
    pub id : uuid::Uuid,
    pub name : String,
    pub ingredients : Vec<String>,
    pub steps : Vec<String>,
}

impl RecipeSerialized {
    pub fn into_recipe(self, cx: Scope) -> Recipe {
        Recipe::new(
            cx,
            self.id,
            self.name,
            self.ingredients,
            self.steps,
        )
    }
}

impl From<&Recipe> for RecipeSerialized {
    fn from(value: &Recipe) -> Self {
        Self {
            id: value.id,
            name: value.name.get(),
            ingredients: value.ingredients.get(),
            steps: value.steps.get(),
        }
    }
}
