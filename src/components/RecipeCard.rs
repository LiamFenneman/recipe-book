use crate::recipe::Recipe;
use leptos::*;

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
