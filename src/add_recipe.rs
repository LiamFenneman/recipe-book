use leptos::*;

#[component]
pub fn AddRecipe(cx: Scope) -> impl IntoView {
    let (err_name, set_err_name) = create_signal::<Option<String>>(cx, None);
    let (err_ingrs, set_err_ingrs) = create_signal::<Option<String>>(cx, None);
    let (err_steps, set_err_steps) = create_signal::<Option<String>>(cx, None);

    view! {
        cx,
        <main class="max-w-md mx-auto py-8">
            <h1 class="mt-8 text-6xl font-normal font-black text-white drop-shadow">"New Recipe"</h1>
            <form
                class="flex flex-col mt-8 p-4 text-white bg-black bg-opacity-5 rounded-xl shadow-outline"
                on:submit=move |e| {
                    e.prevent_default();
                    // TODO: submit form
                }
            >
                <label class="mb-1" for="name">"Name"</label>
                <input class="text-black mb-4 rounded-xl" name="name" type="text" />
                <Show
                    when=move || err_name().is_some()
                    fallback=move |_| view! { cx, <></> }
                >
                    <label class="mt-[-1rem] mb-2 text-red-500 text-xs" for="name">{ err_name().unwrap() }</label>
                </Show>

                <label class="mb-1" for="ingredients">"Ingredients"</label>
                <label class="mb-1 text-xs" for="ingredients">"Each line is a separate ingredient."</label>
                <GrowableInput class="text-black mb-4 resize-none rounded-xl" name="ingredients" />
                <Show
                    when=move || err_ingrs().is_some()
                    fallback=move |_| view! { cx, <></> }
                >
                    <label class="mt-[-1rem] mb-2 text-red-500 text-xs" for="name">{ err_ingrs().unwrap() }</label>
                </Show>

                <label class="mb-1" for="steps">"Steps"</label>
                <label class="mb-1 text-xs" for="steps">"Each line is a separate step. Numbers are automatically added."</label>
                <GrowableInput class="text-black mb-4 resize-none rounded-xl" name="steps" />
                <Show
                    when=move || err_steps().is_some()
                    fallback=move |_| view! { cx, <></> }
                >
                    <label class="mt-[-1rem] mb-2 text-red-500 text-xs" for="name">{ err_steps().unwrap() }</label>
                </Show>

                <input class="text-white p-2 mx-auto w-2/3 cursor-pointer rounded-xl border-2 border-transparent hover:border-white" type="submit" />
            </form>
        </main>
    }
}

#[component]
pub fn GrowableInput(
    cx: Scope,
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] name: String,
) -> impl IntoView {
    let (rows, set_rows) = create_signal(cx, 1);
    let on_input = move |e| set_rows(event_target_value(&e).lines().count() + 1);
    view!{
        cx,
        <textarea class=class name=name rows=rows on:input=on_input />
    }
}
