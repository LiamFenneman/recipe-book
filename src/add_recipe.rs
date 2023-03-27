use leptos::{
    html::{Input, Textarea},
    *,
};
use leptos_router::{use_navigate, NavigateOptions};
use web_sys::SubmitEvent;

use crate::{
    recipe::{Recipe, Recipes, STORAGE_KEY},
    storage::RecipeSerialized,
};

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

    let is_loading_chat = create_rw_signal(cx, false);
    let send_chatgpt_request = move |e: ev::MouseEvent| {
        e.prevent_default();

        let name = el_name().expect("<input> exists").value();
        if name.is_empty() {
            is_loading_chat.set(false);
            set_err_name(Some("Name cannot be empty."));
            return;
        } else {
            is_loading_chat.set(true);
            set_err_name(None);
        }

        el_ingrs()
            .expect("<textarea> exists")
            .set_value("Loading...");
        el_steps()
            .expect("<textarea> exists")
            .set_value("Loading...");

        spawn_local(async move {
            let res = crate::chatgpt::fetch(name).await;
            let (i, s) = crate::chatgpt::transform(&res);
            
            set_rows_ingrs(i.len());
            set_rows_steps(s.len());

            el_ingrs().expect("<textarea> exists").set_value(&i.join("\n"));
            el_steps().expect("<textarea> exists").set_value(&s.join("\n"));

            is_loading_chat.set(false);
        });
    };

    let on_submit = move |e: SubmitEvent| {
        e.prevent_default();

        if is_loading_chat.get() {
            set_err_ingrs(Some("Wait for ChatGPT to finish."));
            set_err_steps(Some("Wait for ChatGPT to finish."));
            return;
        }

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
            let mut recipes = Recipes::new(cx);
            recipes
                .0
                .push(Recipe::new(cx, uuid::Uuid::new_v4(), name, ingrs, steps));
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
                <label class="mb-1 text-xs" for="name">"Example: Homemade pizza"</label>
                <input class="text-black mb-4 rounded-xl" name="name" type="text" node_ref=el_name />
                <Show
                    when=move || err_name().is_some()
                    fallback=move |_| view! { cx, <></> }
                >
                    <label class="mt-[-1rem] mb-2 text-red-500 text-xs" for="name">{ err_name().unwrap() }</label>
                </Show>

                <Show
                    when=move || !is_loading_chat()
                    fallback=move |_| view! {
                        cx,
                        <div class="text-white text-center w-2/3 p-2 mb-4 mx-auto cursor-pointer rounded-xl
                            border-2 border-white hover:bg-black hover:bg-opacity-5 select-none">
                            "Loading..."
                        </div>
                    }
                >
                    <button
                        class="text-white w-2/3 p-2 mb-4 mx-auto cursor-pointer rounded-xl border-2 border-white hover:bg-black hover:bg-opacity-5"
                        on:click=send_chatgpt_request
                    >
                        "Ask ChatGPT"
                    </button>
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
                    <input class="text-white flex-1 p-2 mx-auto cursor-pointer rounded-xl border-2 border-white hover:bg-black hover:bg-opacity-10" type="submit" />
                </div>
            </form>
        </main>
    }
}
