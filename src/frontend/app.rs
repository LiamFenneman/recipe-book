use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <Link rel="preconnect" href="https://fonts.bunny.net"/>
        <Link href=r#"https://fonts.bunny.net/css?family=alata:400"# rel="stylesheet"/>
        <Link href=r#"https://fonts.bunny.net/css?family=black-han-sans:400"# rel="stylesheet"/>

        <Stylesheet id="leptos" href="/pkg/recipe_book.css"/>

        <Title text="Welcome to Leptos"/>

        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { cx,
        <h1 class="bg-red-500">"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
