use leptos::*;

#[component]
pub fn Page(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <main class="max-w-2xl mx-auto py-4">
            {children(cx)}
        </main>
    }
}
