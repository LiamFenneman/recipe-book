use crate::{recipe::*, storage::RecipeSerialized};
use leptos::*;

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    let recipes = create_rw_signal::<Recipes>(cx, Recipes::new(cx));

    create_effect(cx, move |_| {
        if let Ok(Some(storage)) = window().local_storage() {
            let objs = recipes
                .get()
                .0
                .iter()
                .map(RecipeSerialized::from)
                .collect::<Vec<_>>();
            let json = serde_json::to_string(&objs).expect("couldn't serialize recipes");
            if storage.set_item(STORAGE_KEY, &json).is_err() {
                log::error!("error while trying to set item in localStorage");
            }
        }
    });

    view! {
        cx,
        <main class="max-w-2xl mx-auto py-4">
            <h1 class="mt-8 text-6xl font-normal font-black text-white drop-shadow">"Recipe Book"</h1>
            <h2 class="text-xs font-normal text-white drop-shadow">"by Liam Fenneman"</h2>
            <div class="mt-[-1rem] flex flex-row justify-end">
                <a
                    class="px-4 py-2 text-white bg-black bg-opacity-5 drop-shadow border-2 rounded-xl"
                    href="/new"
                >
                    "New Recipe"
                </a>
            </div>
            <div class="mt-4 text-2xl font-normal text-white drop-shadow flex flex-col gap-4">
                <For
                    each=recipes
                    key=|recipe| recipe.id
                    view=move |cx, recipe: Recipe| view! { cx, <RecipeCard recipe=recipe /> }
                />
            </div>
        </main>
    }
}
