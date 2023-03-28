use leptos::*;
use uuid::Uuid;

use crate::storage::RecipeSerialized;

pub const STORAGE_KEY: &str = "recipe-book";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Recipes(pub Vec<Recipe>);

impl Recipes {
    pub fn new(cx: Scope) -> Self {
        let starting_recipes = if let Ok(Some(storage)) = window().local_storage() {
            storage
                .get_item(STORAGE_KEY)
                .ok()
                .flatten()
                .and_then(|value| serde_json::from_str::<Vec<RecipeSerialized>>(&value).ok())
                .map(|values| {
                    values
                        .into_iter()
                        .map(|stored| stored.into_recipe(cx))
                        .collect()
                })
                .unwrap_or_else(|| Self::example(cx))
        } else {
            Self::example(cx)
        };
        Self(starting_recipes)
    }

    fn example(cx: Scope) -> Vec<Recipe> {
        vec![
            Recipe::new(
                cx,
                Uuid::new_v4(),
                "Pancakes".into(),
                vec![
                    "1 cup of flour".into(),
                    "2 eggs".into(),
                    "1 cup of milk".into(),
                    "2 tablespoons of sugar".into(),
                    "1 teaspoon of baking powder".into(),
                    "1/2 teaspoon of salt".into(),
                    "1 tablespoon of butter".into(),
                ],
                vec![
                    "1. Mix the dry ingredients together in a bowl.".into(),
                    "2. Mix the wet ingredients together in a separate bowl.".into(),
                    "3. Combine the wet and dry ingredients.".into(),
                    "4. Heat a pan over medium heat.".into(),
                    "5. Pour the batter into the pan.".into(),
                    "6. Cook until the edges start to bubble.".into(),
                    "7. Flip and cook until golden brown.".into(),
                ],
            ),
            Recipe::new(
                cx,
                Uuid::new_v4(),
                "Omelette".into(),
                vec![
                    "2 eggs".into(),
                    "1 tablespoon of butter".into(),
                    "1/2 cup of cheese".into(),
                    "1/2 cup of vegetables".into(),
                ],
                vec![
                    "1. Crack the eggs into a bowl.".into(),
                    "2. Whisk the eggs.".into(),
                    "3. Heat a pan over medium heat.".into(),
                    "4. Add the butter to the pan.".into(),
                    "5. Add the eggs to the pan.".into(),
                    "6. Add the cheese to the eggs.".into(),
                    "7. Add the vegetables to the eggs.".into(),
                    "8. Cook until the edges start to bubble.".into(),
                    "9. Flip and cook until golden brown.".into(),
                ],
            ),
        ]
    }
}

impl IntoIterator for Recipes {
    type Item = Recipe;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Recipe {
    pub id: Uuid,
    pub name: RwSignal<String>,
    pub ingredients: RwSignal<Vec<String>>,
    pub steps: RwSignal<Vec<String>>,
}

impl Recipe {
    pub fn new(
        cx: Scope,
        id: Uuid,
        name: String,
        ingredients: Vec<String>,
        steps: Vec<String>,
    ) -> Self {
        let name = create_rw_signal(cx, name);
        let ingredients = create_rw_signal(cx, ingredients);
        let steps = create_rw_signal(cx, steps);

        Self {
            id,
            name,
            ingredients,
            steps,
        }
    }
}

#[component]
pub fn RecipeCard(cx: Scope, recipe: Recipe) -> impl IntoView {
    let (expand, set_expand) = create_signal(cx, false);

    view! {
        cx,
        <div class="w-full border-2 rounded-xl drop-shadow bg-black bg-opacity-5">
            <div
                class="flex flex-row justify-between cursor-pointer select-none p-4 hover:bg-black hover:bg-opacity-5 focus:bg-black focus:bg-opacity-5"
                on:click=move |_| set_expand(!expand())
            >
                <h3 class="font-bold drop-shadow-sm">{recipe.name}</h3>
                <div class="flex flex-row gap-4">
                    <p class="text-base">{format!("{} ingredients", recipe.ingredients.get().len())}</p>
                    <p class="text-base">{format!("{} steps", recipe.steps.get().len())}</p>
                </div>
            </div>
            <Show
                when=expand
                fallback=move |_| view! { cx, <></> }
            >
                <div class="border-t-2 p-4 space-y-2">
                    <h5 class="">"Ingredients"</h5>
                    <ul class="ml-2 list-disc list-inside space-y-1">
                    { recipe.ingredients.get().iter().map(|ing| {
                        view! {
                            cx,
                            <li class="text-base">{ing}</li>
                        }
                    }).collect::<Vec<_>>() }
                    </ul>
                    <h5 class="">"Steps"</h5>
                    <ol class="ml-2 list-none list-outside space-y-1">
                    { recipe.steps.get().iter().map(|step| {
                        view! {
                            cx,
                            <li class="text-base">{step}</li>
                        }
                    }).collect::<Vec<_>>() }
                    </ol>
                </div>
            </Show>
        </div>
    }
}
