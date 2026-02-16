use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;

#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <nav class="navbar">
            <div class="nav-brand">{ "Liam Bury" }</div>
            <div class="nav-links">
                <Link<Route> to={Route::Home} classes="nav-link">{ "Home" }</Link<Route>>
                <Link<Route> to={Route::About} classes="nav-link">{ "About" }</Link<Route>>
                <Link<Route> to={Route::Blog} classes="nav-link">{ "Blog" }</Link<Route>>
            </div>
        </nav>
    }
}
