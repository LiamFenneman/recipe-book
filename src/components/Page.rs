use leptos::*;
use crate::app::DarkModeContext;

#[component]
pub fn Page(cx: Scope, children: Children) -> impl IntoView {
    let dark_mode = use_context::<DarkModeContext>(cx).unwrap().dark_mode;

    view! { cx,
        <main class=move || match dark_mode() {
            true => "bg-black dark overflow-x-hidden",
            false => "bg-white light overflow-x-hidden",
        }>
            {children(cx)}
        </main>
    }
}
