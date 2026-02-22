use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;
use crate::context::{TerminalContext, TerminalAction};
use crate::content::get_all_posts;

#[derive(Properties, PartialEq)]
pub struct BlogProps {
    #[prop_or_default]
    pub tag: Option<String>,
}

#[function_component(Blog)]
pub fn blog(props: &BlogProps) -> Html {
    let ctx = use_context::<TerminalContext>();
    let tag = props.tag.clone();

    use_effect_with(tag.clone(), {
        let ctx = ctx.clone();
        move |tag| {
            if let Some(ctx) = ctx {
                if let Some(t) = tag {
                    ctx.dispatch(TerminalAction::Log(format!("Filtering blog posts by tag: {}", t), "cmd".into()));
                } else {
                    ctx.dispatch(TerminalAction::Log("Accessing /blog directory...".into(), "cmd".into()));
                }
            }
        }
    });

    let all_posts = get_all_posts();
    let posts = if let Some(t) = &props.tag {
        all_posts.into_iter().filter(|p| p.tags.contains(&t.as_str())).collect::<Vec<_>>()
    } else {
        all_posts
    };

    html! {
        <div class="blog-container">
            <h1>{ if let Some(t) = &props.tag { format!("Blog - Tag: {}", t) } else { "Blog".to_string() } }</h1>
            <p class="subtitle">{ "Thoughts on AI, Systems, and Rust." }</p>
            
            <div class="post-list">
                {
                    if posts.is_empty() {
                        html! { <p>{ "No posts found with this tag." }</p> }
                    } else {
                        posts.into_iter().map(|post| {
                            html! {
                                <div class="blog-post-preview" style="margin-bottom: 20px; padding-bottom: 20px; border-bottom: 1px solid #333;">
                                    <h2 style="margin-bottom: 5px;">{ post.title }</h2>
                                    <div style="font-size: 0.9em; color: #888; margin-bottom: 10px;">
                                        { post.date }
                                        { " | " }
                                        { for post.tags.iter().map(|tag| html! { <span style="margin-right: 5px; color: #4ec9b0;">{ format!("#{}", tag) }</span> }) }
                                    </div>
                                    <p>{ post.excerpt }</p>
                                    <Link<Route> to={Route::BlogPost { id: post.id.to_string() }} classes="read-more" >
                                        { "Read more..." }
                                    </Link<Route>>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                }
            </div>
        </div>
    }
}
