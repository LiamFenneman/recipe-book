use leptos::*;
use crate::app::DarkModeContext;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    let set_dark_mode = use_context::<DarkModeContext>(cx).unwrap().set_dark_mode;
    let on_click2 = move |_| {
        set_dark_mode.update(|dark_mode| *dark_mode = !*dark_mode);
    };
    
    view! { cx,
        <h1 class="bg-blue-500">"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        <button on:click=on_click2>"Toggle Dark"</button>
    }
}
