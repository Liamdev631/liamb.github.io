use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;
use crate::context::{TerminalContext, TerminalAction};

struct ProjectItem {
    id: &'static str,
    title: &'static str,
    description: &'static str,
    image_url: &'static str,
}

#[function_component(Portfolio)]
pub fn portfolio() -> Html {
    let ctx = use_context::<TerminalContext>();

    use_effect_with((), {
        let ctx = ctx.clone();
        move |_| {
            if let Some(ctx) = ctx {
                ctx.dispatch(TerminalAction::Log("Accessing Portfolio Gallery...".into(), "cmd".into()));
                ctx.dispatch(TerminalAction::Log("Loading project thumbnails...".into(), "info".into()));
                ctx.dispatch(TerminalAction::Log("Render complete.".into(), "success".into()));
            }
        }
    });

    let projects = vec![
        ProjectItem {
            id: "through-their-eyes",
            title: "Through Their Eyes",
            description: "Local LLM-driven NPC simulation with 4 concurrent agents and complex memory hierarchy.",
            image_url: "https://placehold.co/600x400/252526/d4d4d4?text=Through+Their+Eyes",
        },
        ProjectItem {
            id: "cpu-ray-tracer",
            title: "CPU Ray Tracer",
            description: "A high-performance software ray tracer built from scratch.",
            image_url: "https://placehold.co/600x400/252526/d4d4d4?text=CPU+Ray+Tracer",
        },
    ];

    html! {
        <div class="portfolio-container">
            <h1 style="margin-bottom: 20px;">{ "Portfolio" }</h1>
            <div class="portfolio-grid" style="
                display: grid;
                grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
                gap: 20px;
            ">
                { for projects.iter().map(|project| {
                    html! {
                        <Link<Route> to={Route::Project { id: project.id.to_string() }} classes="portfolio-card-link">
                            <div class="portfolio-card" style="
                                background-color: #252526;
                                border: 1px solid #444;
                                border-radius: 5px;
                                overflow: hidden;
                                transition: transform 0.2s;
                                height: 100%;
                                display: flex;
                                flex-direction: column;
                            ">
                                <div class="card-image" style="height: 200px; overflow: hidden;">
                                    <img src={project.image_url} alt={project.title} style="width: 100%; height: 100%; object-fit: cover;" />
                                </div>
                                <div class="card-content" style="padding: 15px; flex: 1; display: flex; flex-direction: column;">
                                    <h3 style="margin: 0 0 10px 0; color: #569cd6;">{ project.title }</h3>
                                    <p style="margin: 0; font-size: 0.9em; color: #cccccc;">{ project.description }</p>
                                </div>
                            </div>
                        </Link<Route>>
                    }
                }) }
            </div>
            
            <style>
                { "
                    .portfolio-card:hover {
                        transform: translateY(-5px);
                        border-color: #569cd6 !important;
                    }
                    .portfolio-card-link {
                        text-decoration: none;
                        color: inherit;
                    }
                " }
            </style>
        </div>
    }
}
