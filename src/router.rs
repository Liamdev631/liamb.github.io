use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/blog")]
    Blog,
    #[at("/blog/:id")]
    BlogPost { id: String },
    #[at("/blog/tag/:tag")]
    BlogTag { tag: String },
    #[at("/portfolio")]
    Portfolio,
    #[at("/portfolio/:id")]
    Project { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}
