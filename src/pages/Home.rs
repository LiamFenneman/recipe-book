use crate::{app::RecipesContext, components::RecipeCard::*, recipe::Recipe};
use leptos::*;
use leptos_router::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    let recipes = use_context::<RecipesContext>(cx).unwrap().recipes;

    view! { cx,
        <h1 class="mt-8 text-6xl font-normal font-black text-white drop-shadow">"Recipe Book"</h1>
        <h2 class="text-xs font-normal text-white drop-shadow">"by Liam Fenneman"</h2>
        <div class="mt-[-1rem] flex flex-row justify-end">
            <A
                class="px-4 py-2 text-white bg-black bg-opacity-5 drop-shadow border-2 rounded-xl"
                href="/new"
            >
                "New Recipe"
            </A>
        </div>
        <div class="mt-4 text-2xl font-normal text-white drop-shadow flex flex-col gap-4">
            <For
                each=recipes
                key=|recipe| recipe.id
                view=move |cx, recipe: Recipe| view! { cx, <RecipeCard recipe=recipe /> }
            />
        </div>
    }
}
