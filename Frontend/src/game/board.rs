use leptos::{component, IntoView, view};

use super::cards::CardCell;

// This is the individually cell on the board. There are currently 4.
#[component]
fn Cell(card: CardCell) -> impl IntoView {
    view! {
        <div class=move || {
            format!(
                "cell {}", 
                match card { 
                    CardCell::Empty => "",
                    CardCell::Card(class) => class, 
                }
            )
        }></div>
    }
}

#[component]
pub fn Board() -> impl IntoView {
    view! {
        
    }
}