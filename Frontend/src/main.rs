mod game;
mod style;

use leptos::{component, document, ev, HtmlElement, IntoView, mount_to_body, on_cleanup, view, window_event_listener};
use leptos::html::Div;

fn main() {
    let handler = window_event_listener(ev::keypress, |ev| {
        let card: &HtmlElement<Div> = document().get_element_by_id("card").unwrap().as_ref();

        card.style((), ()).set_property("grid-row", "5").expect("Failed to set property");

    });

    mount_to_body(|| view! { <App />});

    on_cleanup(move || {
        handler.remove();
    })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="grid_container" id="grid_container">
            <div class="card" id="card"></div>
        </div>
    }
}
