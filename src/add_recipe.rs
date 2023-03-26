use leptos::{
    html::{Input, Textarea},
    *,
};
use leptos_router::{use_navigate, NavigateOptions};
use web_sys::SubmitEvent;

use crate::{recipe::{Recipe, Recipes, STORAGE_KEY}, storage::RecipeSerialized};

#[component]
pub fn AddRecipe(cx: Scope) -> impl IntoView {
    let (err_name, set_err_name) = create_signal::<Option<&'static str>>(cx, None);
    let (err_ingrs, set_err_ingrs) = create_signal::<Option<&'static str>>(cx, None);
    let (err_steps, set_err_steps) = create_signal::<Option<&'static str>>(cx, None);

    let el_name: NodeRef<Input> = create_node_ref(cx);
    let el_ingrs: NodeRef<Textarea> = create_node_ref(cx);
    let el_steps: NodeRef<Textarea> = create_node_ref(cx);

    let (rows_ingrs, set_rows_ingrs) = create_signal(cx, 1);
    let (rows_steps, set_rows_steps) = create_signal(cx, 1);

    let on_submit = move |e: SubmitEvent| {
        e.prevent_default();

        let name = el_name().expect("<input> exists").value();
        let ingrs = el_ingrs()
            .expect("<textarea> exists")
            .value()
            .lines()
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty() && s != "")
            .collect::<Vec<_>>();
        let steps = el_steps()
            .expect("<textarea> exists")
            .value()
            .lines()
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty() && s != "")
            .collect::<Vec<_>>();

        set_err_name(if name.is_empty() {
            Some("Name cannot be empty.")
        } else {
            None
        });

        set_err_ingrs(if ingrs.is_empty() {
            Some("Ingredients cannot be empty.")
        } else {
            None
        });

        set_err_steps(if steps.is_empty() {
            Some("Steps cannot be empty.")
        } else {
            None
        });

        if err_name().is_none() && err_ingrs().is_none() && err_steps().is_none() {
            // TODO: update the recipes signal
            let mut recipes = Recipes::new(cx);
            recipes.0.push(Recipe::new(cx, uuid::Uuid::new_v4(), name, ingrs, steps));
            if let Ok(Some(storage)) = window().local_storage() {
                let objs = recipes
                    .0
                    .iter()
                    .map(RecipeSerialized::from)
                    .collect::<Vec<_>>();
                let json = serde_json::to_string(&objs).expect("couldn't serialize recipes");
                if storage.set_item(STORAGE_KEY, &json).is_err() {
                    log::error!("error while trying to set item in localStorage");
                }
            }

            // navigate to home page
            let _ = use_navigate(cx)(
                "/",
                NavigateOptions {
                    replace: true,
                    ..Default::default()
                },
            );
        }
    };

    view! {
        cx,
        <main class="max-w-md mx-auto py-4">
            <h1 class="mt-8 text-6xl font-normal font-black text-white drop-shadow">"New Recipe"</h1>
            <form
                class="flex flex-col mt-8 p-4 text-white bg-black bg-opacity-5 rounded-xl shadow-outline"
                on:submit=on_submit
            >
                <label class="mb-1" for="name">"Name"</label>
                <input class="text-black mb-4 rounded-xl" name="name" type="text" node_ref=el_name />
                <Show
                    when=move || err_name().is_some()
                    fallback=move |_| view! { cx, <></> }
                >
                    <label class="mt-[-1rem] mb-2 text-red-500 text-xs" for="name">{ err_name().unwrap() }</label>
                </Show>

                <label class="mb-1" for="ingredients">"Ingredients"</label>
                <label class="mb-1 text-xs" for="ingredients">"Each line is a separate ingredient."</label>
                <textarea
                    class="text-black mb-4 resize-none rounded-xl"
                    name="ingredients"
                    on:input=move |e| set_rows_ingrs(event_target_value(&e).lines().count() + 1)
                    rows=rows_ingrs
                    node_ref=el_ingrs
                />
                <Show
                    when=move || err_ingrs().is_some()
                    fallback=move |_| view! { cx, <></> }
                >
                    <label class="mt-[-1rem] mb-2 text-red-500 text-xs" for="name">{ err_ingrs().unwrap() }</label>
                </Show>

                <label class="mb-1" for="steps">"Steps"</label>
                <label class="mb-1 text-xs" for="steps">"Each line is a separate step. Numbers are automatically added."</label>
                <textarea
                    class="text-black mb-4 resize-none rounded-xl"
                    name="steps"
                    on:input=move |e| set_rows_steps(event_target_value(&e).lines().count() + 1)
                    rows=rows_steps
                    node_ref=el_steps
                />
                <Show
                    when=move || err_steps().is_some()
                    fallback=move |_| view! { cx, <></> }
                >
                    <label class="mt-[-1rem] mb-2 text-red-500 text-xs" for="name">{ err_steps().unwrap() }</label>
                </Show>

                <div class="flex flex-row gap-4 text-center">
                    <a href="/" class="text-white w-1/3 p-2 mx-auto cursor-pointer rounded-xl border-2 border-transparent hover:border-white" type="submit">"Back"</a>
                    <input class="text-white flex-1 p-2 mx-auto cursor-pointer rounded-xl border-2 border-transparent hover:border-white" type="submit" />
                </div>
            </form>
        </main>
    }
}
