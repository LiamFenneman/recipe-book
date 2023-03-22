use crate::recipe::*;
use leptos::*;

const STORAGE_KEY: &str = "recipe-book";

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    let (recipes, _set_recipes) = create_signal(cx, Vec::from(Recipe::examples()));

    view! {
        cx,
        <main class="max-w-2xl mx-auto py-4">
            <h1 class="mt-8 text-6xl font-normal font-black text-white drop-shadow">"Recipe Book"</h1>
            <h2 class="text-xs font-normal text-white drop-shadow">"by Liam Fenneman"</h2>
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
