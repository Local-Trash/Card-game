mod game;
mod style;
mod resource;
mod home;

use leptos::{component, mount_to_body, view, IntoView};
use game::Game;
use home::Header;

fn main() {
    mount_to_body(|| view! { <App />});
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Header />
        <Game />
    }
}
