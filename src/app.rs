use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::add_recipe::*;
use crate::home::*;

pub fn path_with_prefix(path: &str) -> String {
    cfg_if::cfg_if! {
        if #[cfg(debug_assertions)] {
            let path_prefix = "";
        } else {
            let path_prefix = "/recipe-book";
        }
    }

    format!("{}{}", path_prefix, path)
}

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
                <Route path={path_with_prefix("/")} view=move |cx| view! { cx, <HomePage /> } />
                <Route path={path_with_prefix("/new")} view=move |cx| view! { cx, <AddRecipe /> } />
            </Routes>
        </Router>
    }
}
