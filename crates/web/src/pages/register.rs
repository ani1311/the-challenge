use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::{api::users::register_user, components::{ErrorMessage, TextInput}};

#[component]
pub fn RegisterPage(on_registered: impl Fn() + Copy + Send + Sync + 'static) -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (error, set_error) = signal(None::<String>);
    let (is_loading, set_is_loading) = signal(false);

    let submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        let name_value = name.get();
        if name_value.trim().is_empty() {
            set_error.set(Some("Name is required".to_string()));
            return;
        }

        set_error.set(None);
        set_is_loading.set(true);

        spawn_local(async move {
            let result = register_user(name_value).await;
            set_is_loading.set(false);

            match result {
                Ok(_) => on_registered(),
                Err(message) => set_error.set(Some(message)),
            }
        });
    };

    view! {
        <h1>"Register"</h1>
        <form on:submit=submit>
            <TextInput value=name set_value=set_name placeholder="Name" />
            <button type="submit" disabled=move || is_loading.get()>
                {move || if is_loading.get() { "Registering..." } else { "Register" }}
            </button>
        </form>
        <ErrorMessage message=error />
    }
}
