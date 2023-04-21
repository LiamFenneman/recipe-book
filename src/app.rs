use crate::{pages::Home::*, components::Page::*};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[derive(Clone)]
pub struct DarkModeContext {
    pub dark_mode: ReadSignal<bool>,
    pub set_dark_mode: WriteSignal<bool>,
}

pub fn provide_dark_mode_context(cx: Scope) {
    let (dark_mode, set_dark_mode) = create_signal(cx, false);

    provide_context(
        cx,
        DarkModeContext {
            dark_mode,
            set_dark_mode,
        },
    );
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);
    provide_dark_mode_context(cx);

    view! {
        cx,
        <Link rel="preconnect" href="https://fonts.bunny.net"/>
        <Link href=r#"https://fonts.bunny.net/css?family=alata:400"# rel="stylesheet"/>
        <Link href=r#"https://fonts.bunny.net/css?family=black-han-sans:400"# rel="stylesheet"/>

        <Stylesheet id="leptos" href="/pkg/recipe_book.css"/>

        <Title text="Welcome to Leptos"/>

        <Router>
            <Page>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <Home/> }/>
                </Routes>
            </Page>
        </Router>
    }
}
