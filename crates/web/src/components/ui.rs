use leptos::prelude::*;

#[component]
pub fn TextInput(
    value: ReadSignal<String>,
    set_value: WriteSignal<String>,
    placeholder: &'static str,
) -> impl IntoView {
    view! {
        <input
            placeholder=placeholder
            prop:value=move || value.get()
            on:input=move |ev| set_value.set(event_target_value(&ev))
        />
    }
}

#[component]
pub fn ErrorMessage(message: ReadSignal<Option<String>>) -> impl IntoView {
    view! {
        <p style="color: red;">{move || message.get().unwrap_or_default()}</p>
    }
}
