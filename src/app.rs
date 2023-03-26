use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::add_recipe::*;
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
                <Route path="/new" view=move |cx| view! { cx, <AddRecipe /> } />
            </Routes>
        </Router>
    }
}
