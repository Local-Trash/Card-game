use leptos::{component, IntoView, view};

// This is the individually cell on the board. There are 16.
#[component]
fn Cell() -> impl IntoView {
    view! {
        <div class=move || {
            format!("cell")
        }></div>
    }
}

#[component]
fn Board() -> IntoView {
    view! {}
}