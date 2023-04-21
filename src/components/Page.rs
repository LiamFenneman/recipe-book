use leptos::*;

#[component]
pub fn Page(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <main>
            {children(cx)}
        </main>
    }
}
