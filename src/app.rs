use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <Title text="Recipe Book" />
        <Body
            class="min-h-screen bg-gradient-to-br from-cyan-600 to-blue-600 pr-[20px]"
        />
        <Router>
            <Routes>
                <Route path="/" view=move |cx| view! { cx, <HomePage /> } />
            </Routes>
        </Router>
    }
}

#[derive(Debug, Clone)]
pub struct Recipe {
    name: String,
    ingredients: Vec<String>,
    steps: Vec<String>,
}

#[component]
pub fn RecipeCard(cx: Scope, recipe: Recipe) -> impl IntoView {
    let (expand, set_expand) = create_signal(cx, false);
    view! {
        cx,
        <div class="w-full border-2 rounded-xl drop-shadow">
            <div
                class="flex flex-row justify-between cursor-pointer select-none p-4"
                on:click=move |_| set_expand(!expand())
            >
                <h3 class="font-bold drop-shadow-sm">{recipe.name}</h3>
                <div class="flex flex-row gap-4">
                    <p class="text-base drop-shadow-sm">{format!("{} ingredients", recipe.ingredients.len())}</p>
                    <p class="text-base drop-shadow-sm">{format!("{} steps", recipe.steps.len())}</p>
                </div>
            </div>
            <Show
                when=move || expand()
                fallback=move |_| view! { cx, <></> }
            >
                <div class="border-t-2 p-4 space-y-2">
                    <h5 class="drop-shadow-sm">"Ingredients"</h5>
                    <ul class="ml-4 list-disc list-inside space-y-1">
                    { recipe.ingredients.iter().map(|ing| {
                        view! {
                            cx,
                            <li class="text-base drop-shadow-sm">{ing}</li>
                        }
                    }).collect::<Vec<_>>() }
                    </ul>
                    <h5 class="drop-shadow-sm">"Steps"</h5>
                    <ol class="ml-8 list-decimal list-outside space-y-1">
                    { recipe.steps.iter().map(|step| {
                        view! {
                            cx,
                            <li class="text-base drop-shadow-sm">{step}</li>
                        }
                    }).collect::<Vec<_>>() }
                    </ol>
                </div>
            </Show>
        </div>
    }
}

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    let (recipes, _set_recipes) = create_signal(
        cx,
        vec![
            Recipe {
                name: "Pancakes".to_string(),
                ingredients: vec![
                    "1 cup of flour".to_string(),
                    "2 eggs".to_string(),
                    "1 cup of milk".to_string(),
                    "2 tablespoons of sugar".to_string(),
                    "1 teaspoon of baking powder".to_string(),
                    "1/2 teaspoon of salt".to_string(),
                    "1 tablespoon of butter".to_string(),
                ],
                steps: vec![
                    "Mix the dry ingredients together in a bowl.".to_string(),
                    "Mix the wet ingredients together in a separate bowl.".to_string(),
                    "Combine the wet and dry ingredients.".to_string(),
                    "Heat a pan over medium heat.".to_string(),
                    "Pour the batter into the pan.".to_string(),
                    "Cook until the edges start to bubble.".to_string(),
                    "Flip and cook until golden brown.".to_string(),
                ],
            },
            Recipe {
                name: "Omelette".to_string(),
                ingredients: vec![
                    "2 eggs".to_string(),
                    "1 tablespoon of butter".to_string(),
                    "1/2 cup of cheese".to_string(),
                    "1/2 cup of vegetables".to_string(),
                ],
                steps: vec![
                    "Crack the eggs into a bowl.".to_string(),
                    "Whisk the eggs.".to_string(),
                    "Heat a pan over medium heat.".to_string(),
                    "Add the butter to the pan.".to_string(),
                    "Add the eggs to the pan.".to_string(),
                    "Add the cheese to the eggs.".to_string(),
                    "Add the vegetables to the eggs.".to_string(),
                    "Cook until the edges start to bubble.".to_string(),
                    "Flip and cook until golden brown.".to_string(),
                ],
            },
        ],
    );

    view! {
        cx,
        <main class="max-w-3xl mx-auto py-4">
            <h1 class="mt-8 text-6xl font-normal font-black text-white drop-shadow">"Recipe Book"</h1>
            <div class="mt-8 text-2xl font-normal text-white drop-shadow flex flex-col gap-4">
                <For
                    each=recipes
                    key=|recipe| recipe.name.clone()
                    view=move |cx, recipe: Recipe| view! { cx, <RecipeCard recipe=recipe /> }
                />
            </div>
        </main>
    }
}
