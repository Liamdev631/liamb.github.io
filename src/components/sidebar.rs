use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;
use crate::context::{TerminalContext, TerminalAction};
use crate::components::terminal::Terminal;
use crate::content::get_all_posts;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let is_portfolio_expanded = use_state(|| true);
    let is_blog_expanded = use_state(|| false);
    let ctx = use_context::<TerminalContext>().expect("No Terminal Context Found");

    let toggle_portfolio = {
        let is_portfolio_expanded = is_portfolio_expanded.clone();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            is_portfolio_expanded.set(!*is_portfolio_expanded);
        })
    };

    let toggle_blog = {
        let is_blog_expanded = is_blog_expanded.clone();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            is_blog_expanded.set(!*is_blog_expanded);
        })
    };

    let log_nav = {
        let ctx = ctx.clone();
        move |msg: &str| {
            ctx.dispatch(TerminalAction::Log(msg.to_string(), "info".to_string()));
        }
    };

    let all_posts = get_all_posts();
    let mut unique_tags: Vec<&str> = all_posts.iter().flat_map(|p| p.tags.clone()).collect();
    unique_tags.sort();
    unique_tags.dedup();

    html! {
        <div class="sidebar" style="display: flex; flex-direction: column;">
            <div class="sidebar-top" style="flex: 1; overflow-y: auto;">
                <div class="sidebar-header">
                    { "EXPLORER" }
                    <i class="codicon codicon-ellipsis"></i>
                </div>
                
                <div class="sidebar-section">
                    <div class="sidebar-section-header">
                        <i class="codicon codicon-chevron-down" style="margin-right: 2px;"></i>
                        { "PORTFOLIO-WORKSPACE" }
                    </div>
                    
                    <div class="sidebar-content">
                        // Home File
                        <div class="sidebar-item">
                            <span class="arrow-container"></span> // Spacer
                            <Link<Route> to={Route::Home} classes="sidebar-link">
                                <div style="display: flex; align-items: center; width: 100%;" onclick={let log = log_nav.clone(); move |_| log("Opening Home...")}>
                                    <img src="https://raw.githubusercontent.com/vscode-icons/vscode-icons/master/icons/file_type_html.svg" class="file-icon-img" alt="html" />
                                    { "Home" }
                                </div>
                            </Link<Route>>
                        </div>

                        // About File
                        <div class="sidebar-item">
                            <span class="arrow-container"></span> // Spacer
                            <Link<Route> to={Route::About} classes="sidebar-link">
                                <div style="display: flex; align-items: center; width: 100%;" onclick={let log = log_nav.clone(); move |_| log("Opening About...")}>
                                    <img src="https://raw.githubusercontent.com/vscode-icons/vscode-icons/master/icons/file_type_html.svg" class="file-icon-img" alt="html" />
                                    { "About" }
                                </div>
                            </Link<Route>>
                        </div>

                        // Blog Folder (Expandable)
                        <div class="sidebar-item-container">
                            <div class="sidebar-item">
                                <span class="arrow-container" onclick={toggle_blog}>
                                    <i class={classes!("codicon", if *is_blog_expanded { "codicon-chevron-down" } else { "codicon-chevron-right" })}></i>
                                </span>
                                <Link<Route> to={Route::Blog} classes="sidebar-link">
                                    <div style="display: flex; align-items: center; width: 100%;" onclick={let log = log_nav.clone(); move |_| log("cd Blog && ls -la")}>
                                        <img 
                                            src={if *is_blog_expanded { "https://raw.githubusercontent.com/vscode-icons/vscode-icons/master/icons/default_folder_opened.svg" } else { "https://raw.githubusercontent.com/vscode-icons/vscode-icons/master/icons/default_folder.svg" }} 
                                            class="file-icon-img" 
                                            alt="blog" 
                                        />
                                        { "Blog" }
                                    </div>
                                </Link<Route>>
                            </div>

                            { if *is_blog_expanded {
                                html! {
                                    <div class="nested-files">
                                        { for unique_tags.iter().map(|tag| {
                                            let tag_str = tag.to_string();
                                            html! {
                                                <div class="sidebar-item">
                                                    <span class="arrow-container"></span>
                                                    <Link<Route> to={Route::BlogTag { tag: tag_str.clone() }} classes="sidebar-link">
                                                        <div style="display: flex; align-items: center; width: 100%;" onclick={let log = log_nav.clone(); let t = tag_str.clone(); move |_| log(&format!("Filtering by tag: {}", t))}>
                                                            <img src="https://raw.githubusercontent.com/vscode-icons/vscode-icons/master/icons/default_file.svg" class="file-icon-img" alt="tag" />
                                                            { tag }
                                                        </div>
                                                    </Link<Route>>
                                                </div>
                                            }
                                        }) }
                                    </div>
                                }
                            } else {
                                html! {}
                            }}
                        </div>

                        // Portfolio Folder (Expandable)
                        <div class="sidebar-item-container">
                            <div class="sidebar-item">
                                <span class="arrow-container" onclick={toggle_portfolio}>
                                    <i class={classes!("codicon", if *is_portfolio_expanded { "codicon-chevron-down" } else { "codicon-chevron-right" })}></i>
                                </span>
                                <Link<Route> to={Route::Portfolio} classes="sidebar-link">
                                    <div style="display: flex; align-items: center; width: 100%;" onclick={let log = log_nav.clone(); move |_| log("cd Portfolio && ls -la")}>
                                        <img 
                                            src={if *is_portfolio_expanded { "https://raw.githubusercontent.com/vscode-icons/vscode-icons/master/icons/default_folder_opened.svg" } else { "https://raw.githubusercontent.com/vscode-icons/vscode-icons/master/icons/default_folder.svg" }} 
                                            class="file-icon-img" 
                                            alt="portfolio" 
                                        />
                                        { "Portfolio" }
                                    </div>
                                </Link<Route>>
                            </div>

                            { if *is_portfolio_expanded {
                                html! {
                                    <div class="nested-files">
                                        <div class="sidebar-item">
                                            <span class="arrow-container"></span>
                                            <Link<Route> to={Route::Project { id: "through-their-eyes".to_string() }} classes="sidebar-link">
                                                <div style="display: flex; align-items: center; width: 100%;" onclick={let log = log_nav.clone(); move |_| log("Opening Through Their Eyes...")}>
                                                    <img src="https://raw.githubusercontent.com/vscode-icons/vscode-icons/master/icons/file_type_vcxproj.svg" class="file-icon-img" alt="proj" />
                                                    { "Through Their Eyes" }
                                                </div>
                                            </Link<Route>>
                                        </div>
                                        <div class="sidebar-item">
                                            <span class="arrow-container"></span>
                                            <Link<Route> to={Route::Project { id: "cpu-ray-tracer".to_string() }} classes="sidebar-link">
                                                <div style="display: flex; align-items: center; width: 100%;" onclick={let log = log_nav.clone(); move |_| log("Opening CPU Ray Tracer...")}>
                                                    <img src="https://raw.githubusercontent.com/vscode-icons/vscode-icons/master/icons/file_type_vcxproj.svg" class="file-icon-img" alt="proj" />
                                                    { "CPU Ray Tracer" }
                                                </div>
                                            </Link<Route>>
                                        </div>
                                    </div>
                                }
                            } else {
                                html! {}
                            } }
                        </div>
                    </div>
                </div>
            </div>
            
            <div class="sidebar-bottom" style="height: 30%; min-height: 150px; border-top: 1px solid #3c3c3c;">
                 <Terminal />
            </div>
        </div>
    }
}
