use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;
mod router;
mod context;
mod content;

use components::layout::Layout;
use context::TerminalProvider;
use pages::home::Home;
use pages::about::About;
use pages::blog::Blog;
use pages::blog_post::BlogPost;
use pages::project::Project;
use pages::portfolio::Portfolio;
use router::Route;

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::About => html! { <About /> },
        Route::Blog => html! { <Blog /> },
        Route::BlogTag { tag } => html! { <Blog tag={Some(tag)} /> },
        Route::BlogPost { id } => html! { <BlogPost id={id} /> },
        Route::Portfolio => html! { <Portfolio /> },
        Route::Project { id } => html! { <Project id={id} /> },
        Route::NotFound => html! { <h1>{ "404 Not Found" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <TerminalProvider>
            <BrowserRouter>
                <Layout>
                    <Switch<Route> render={switch} />
                </Layout>
            </BrowserRouter>
        </TerminalProvider>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
