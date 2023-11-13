mod board;
mod cards;

use leptos::{component, IntoView, view};
use board::Board;

#[component]
pub fn Game() -> impl IntoView {
    view! {
        <Board />
    }
}