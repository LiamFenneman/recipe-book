use crate::recipe::AddRecipe;
use leptos::*;
use leptos_router::*;

#[component]
pub fn NewRecipe(cx: Scope) -> impl IntoView {
    let add_recipe = create_server_action::<AddRecipe>(cx);

    let (rows_ingrs, set_rows_ingrs) = create_signal(cx, 1);
    let (rows_steps, set_rows_steps) = create_signal(cx, 1);

    view! { cx,
        <h1 class="mt-8 text-6xl font-normal font-black text-white drop-shadow">"New Recipe"</h1>
        <ActionForm
            action=add_recipe
            class="flex flex-col gap-4 mt-8 p-4 text-white bg-black bg-opacity-5 rounded-xl shadow-outline"
        >
            <div class="flex flex-col gap-1">
                <label for="name">"Name"</label>
                <label class="text-xs" for="name">"Example: Homemade pizza"</label>
                <input
                    class="text-black px-3 py-2 rounded-xl"
                    name="name"
                    placeholder="Name"
                />
            </div>

            <div class="flex flex-col gap-1">
                <label for="ingredients">"Ingredients"</label>
                <label class="text-xs" for="ingredients">"Each line is a separate ingredient."</label>
                <textarea
                    class="text-black resize-none px-3 py-2 rounded-xl"
                    name="ingredients"
                    placeholder="Ingredients"
                    on:input=move |e| set_rows_ingrs(event_target_value(&e).lines().count() + 1)
                    rows=rows_ingrs
                />
            </div>

            <div class="flex flex-col gap-1">
                <label for="steps">"Steps"</label>
                <label class="text-xs" for="steps">"Each line is a separate step."</label>
                <textarea
                    class="text-black resize-none px-3 py-2 rounded-xl"
                    name="steps"
                    placeholder="Steps"
                    on:input=move |e| set_rows_steps(event_target_value(&e).lines().count() + 1)
                    rows=rows_steps
                />
            </div>

            <div class="flex flex-row gap-4 text-center">
                <A href="/" class="text-white w-1/3 p-2 mx-auto cursor-pointer rounded-xl border-2 border-transparent hover:border-white">"Back"</A>
                <button type="submit"
                    class="text-white flex-1 p-2 mx-auto cursor-pointer rounded-xl border-2 border-white
                    hover:bg-black hover:bg-opacity-10"
                >"Add Recipe"</button>
            </div>
        </ActionForm>
    }
}
