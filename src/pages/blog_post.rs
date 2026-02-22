use yew::prelude::*;
use crate::context::{TerminalContext, TerminalAction};

#[derive(Properties, PartialEq)]
pub struct BlogPostProps {
    pub id: String,
}

#[function_component(BlogPost)]
pub fn blog_post(props: &BlogPostProps) -> Html {
    let ctx = use_context::<TerminalContext>();
    let id = props.id.clone();

    use_effect_with(id.clone(), {
        let ctx = ctx.clone();
        move |id| {
            if let Some(ctx) = ctx {
                ctx.dispatch(TerminalAction::Log(format!("Opening blog post: {}", id), "info".into()));
            }
        }
    });

    let content = match props.id.as_str() {
        "ray-tracer-transparency" => html! {
            <div class="blog-post">
                <h1>{ "Adding Transparency to My CPU Ray Tracer" }</h1>
                <div class="post-meta" style="color: #888; margin-bottom: 20px;">
                    <span>{ "2026-02-17" }</span>
                    <span style="margin: 0 10px;">{ "|" }</span>
                    <span>{ "Graphics Programming" }</span>
                </div>
                
                <div class="post-content">
                    <p>{ "Transparency is a crucial feature for realistic rendering, allowing light to pass through objects like glass, water, or thin fabrics. In this post, I'll explore how I implemented transparency in my CPU-based ray tracer." }</p>
                    
                    <h3>{ "The Concept" }</h3>
                    <p>{ "When a ray hits a transparent surface, it splits into two components: reflection and refraction. The balance between these two is determined by the Fresnel equations." }</p>
                    
                    // Placeholder for future content
                    <p><em>{ "More details coming soon..." }</em></p>
                </div>
            </div>
        },
        _ => html! {
            <div class="blog-post-not-found">
                <h1>{ "Post Not Found" }</h1>
                <p>{ "The blog post you are looking for does not exist." }</p>
            </div>
        }
    };

    html! {
        <div class="blog-post-container" style="max-width: 800px; margin: 0 auto;">
            { content }
        </div>
    }
}
