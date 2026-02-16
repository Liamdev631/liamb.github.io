use yew::prelude::*;
use crate::context::{TerminalContext, TerminalAction};

#[function_component(Blog)]
pub fn blog() -> Html {
    let ctx = use_context::<TerminalContext>();

    use_effect_with((), {
        let ctx = ctx.clone();
        move |_| {
            if let Some(ctx) = ctx {
                ctx.dispatch(TerminalAction::Log("Accessing /blog directory...".into(), "cmd".into()));
                ctx.dispatch(TerminalAction::Log("Found 3 entries.".into(), "info".into()));
            }
        }
    });

    let posts = vec![
        ("Rust for Embedded Systems", "2025-01-15", "Exploring the benefits of Rust in resource-constrained environments."),
        ("Building Offline AI Agents", "2025-02-01", "How to run local LLMs privacy-first with NVIDIA NeMo."),
        ("My Journey with Yew", "2025-02-14", "Building a portfolio site with Rust and WebAssembly."),
    ];

    html! {
        <div class="blog-container">
            <h1>{ "Blog" }</h1>
            <p class="subtitle">{ "Thoughts on AI, Systems, and Rust." }</p>
            
            <div class="post-list">
                {
                    posts.into_iter().map(|(title, date, excerpt)| {
                        html! {
                            <div class="blog-post-preview" style="margin-bottom: 20px; padding-bottom: 20px; border-bottom: 1px solid #333;">
                                <h2 style="margin-bottom: 5px;">{ title }</h2>
                                <div style="font-size: 0.9em; color: #888; margin-bottom: 10px;">{ date }</div>
                                <p>{ excerpt }</p>
                                <a href="#" style="color: #569cd6;">{ "Read more..." }</a>
                            </div>
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}
