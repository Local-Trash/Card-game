use leptos::{component, IntoView, view};

use super::cards::CardCell;

#[component]
pub fn Board() -> impl IntoView {
    view! {
        (0..2).into
    }
}