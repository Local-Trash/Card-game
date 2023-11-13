mod game;
mod style;

use leptos::{component, document, ev, HtmlElement, IntoView, mount_to_body, on_cleanup, view, window_event_listener};
use leptos::html::Div;
use game::Game;

fn main() {
    mount_to_body(|| view! { <App />});
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Game />
    }
}
