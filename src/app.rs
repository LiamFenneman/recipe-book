use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::home::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <Title text="Recipe Book" />
        <Body
            class="min-h-screen bg-gradient-to-br from-cyan-600 to-blue-600 px-8 sm:px-0"
        />
        <Router>
            <Routes>
                <Route path="/" view=move |cx| view! { cx, <HomePage /> } />
                <Route path="/test" view=move |cx| view! { cx, <Test /> } />
            </Routes>
        </Router>
    }
}

#[component]
pub fn Test(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div class="max-w-md mx-auto mt-8">
            <form
                class="flex flex-col"
                on:submit=move |e| {
                    e.prevent_default();
                    log::info!("submitted");
                }
            >
                <label class="mb-1" for="name">"Name"</label>
                <input class="mb-4" name="name" type="text" />

                <label class="mb-1" for="ingredients">"Ingredients"</label>
                <input class="mb-4" name="ingredients" type="text" />

                <label class="mb-1" for="steps">"Steps"</label>
                <input class="mb-4" name="steps" type="text" />

                <input type="submit" />
            </form>
        </div>
    }
}
