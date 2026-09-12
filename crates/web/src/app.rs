use leptos::prelude::*;

use crate::{components::PageShell, pages::{HomePage, RegisterPage}};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Page {
    Register,
    Home,
}

#[component]
pub fn App() -> impl IntoView {
    let (page, set_page) = signal(Page::Register);

    view! {
        <PageShell>
            {move || match page.get() {
                Page::Register => view! {
                    <RegisterPage on_registered=move || set_page.set(Page::Home) />
                }.into_any(),
                Page::Home => view! {
                    <HomePage />
                }.into_any(),
            }}
        </PageShell>
    }
}
