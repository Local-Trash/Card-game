use leptos::{component, IntoView, view};



#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Header />
    }
}

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <div id="header">
            <a href=""><p>"Home"</p></a>
            <a href="/play"><p>"Play"</p></a>
            <a href="/cards"><p>"Cards"</p></a>
            <a href="/leaderboard"><p>"Leader Board"</p></a>
        </div>
    }
}