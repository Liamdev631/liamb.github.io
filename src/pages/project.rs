use yew::prelude::*;
use crate::context::{TerminalContext, TerminalAction};

#[derive(Properties, PartialEq)]
pub struct ProjectProps {
    pub id: String,
}

#[function_component(Project)]
pub fn project(props: &ProjectProps) -> Html {
    let ctx = use_context::<TerminalContext>().expect("No Terminal Context");
    let id = props.id.clone();

    use_effect_with(id.clone(), {
        let ctx = ctx.clone();
        move |id| {
            ctx.dispatch(TerminalAction::Log(format!("Opening project: {}", id), "cmd".into()));
            ctx.dispatch(TerminalAction::Log("Loading project details...".into(), "info".into()));
            ctx.dispatch(TerminalAction::Log("Done.".into(), "success".into()));
        }
    });

    let (title, description, content) = match id.as_str() {
        "through-their-eyes" => (
            "Through Their Eyes",
            "Local LLM-driven NPC simulation with 4 concurrent agents.",
            html! {
                <div class="blog-post-content" style="max-width: 800px;">
                    <p style="font-style: italic; color: #888;">{ "Coming soon..." }</p>
                    // Blank blog post structure ready for content
                </div>
            }
        ),
        _ => (
            "Project Not Found",
            "The requested project file could not be read.",
            html! {}
        ),
    };

    html! {
        <div class="project-detail">
            <h1>{ title }</h1>
            <p class="subtitle">{ description }</p>
            <div class="project-content">
                { content }
            </div>
        </div>
    }
}
