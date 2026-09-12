use leptos::prelude::*;

#[component]
pub fn PageShell(children: Children) -> impl IntoView {
    view! {
        <main style="max-width: 420px; margin: 48px auto; font-family: sans-serif;">
            {children()}
        </main>
    }
}
