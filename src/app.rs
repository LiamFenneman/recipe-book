use crate::{components::Page::*, pages::Home::*, pages::NewRecipe::*, recipe::Recipes};
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

#[derive(Clone)]
pub struct RecipesContext {
    pub recipes: ReadSignal<Recipes>,
    pub set_recipes: WriteSignal<Recipes>,
}

pub fn provide_recipes_context(cx: Scope) {
    let (recipes, set_recipes) = create_signal(cx, Recipes::new(cx));

    provide_context(
        cx,
        RecipesContext {
            recipes,
            set_recipes,
        },
    );
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);
    provide_dark_mode_context(cx);
    provide_recipes_context(cx);

    let dark_mode = use_context::<DarkModeContext>(cx).unwrap().dark_mode;

    view! {
        cx,
        <Link rel="preconnect" href="https://fonts.bunny.net"/>
        <Link href=r#"https://fonts.bunny.net/css?family=alata:400"# rel="stylesheet"/>
        <Link href=r#"https://fonts.bunny.net/css?family=black-han-sans:400"# rel="stylesheet"/>

        <Stylesheet id="leptos" href="/pkg/recipe_book.css"/>

        <Title text="Recipe Book"/>

        <Body class=move || format!(
            "{} min-h-screen bg-gradient-to-br from-cyan-600 to-blue-600 px-8 sm:px-0",
            match dark_mode() {
                true => "dark",
                false => "light",
        })/>

        <Router>
            <Page>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <Home/> }/>
                    <Route path="/new" view=|cx| view! { cx, <NewRecipe/> }/>
                </Routes>
            </Page>
        </Router>
    }
}
