use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;
use crate::context::{TerminalContext, TerminalAction};

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let is_portfolio_expanded = use_state(|| true);
    let ctx = use_context::<TerminalContext>().expect("No Terminal Context Found");

    let toggle_portfolio = {
        let is_portfolio_expanded = is_portfolio_expanded.clone();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            is_portfolio_expanded.set(!*is_portfolio_expanded);
        })
    };

    let log_nav = {
        let ctx = ctx.clone();
        move |msg: &str| {
            ctx.dispatch(TerminalAction::Log(msg.to_string(), "info".to_string()));
        }
    };

    html! {
        <div class="sidebar">
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
                            <div style="display: flex; align-items: center; width: 100%;" onclick={let log = log_nav.clone(); move |_| log("Opening home.html...")}>
                                <img src="https://raw.githubusercontent.com/vscode-icons/vscode-icons/master/icons/file_type_html.svg" class="file-icon-img" alt="html" />
                                { "home.html" }
                            </div>
                        </Link<Route>>
                    </div>

                    // About File
                    <div class="sidebar-item">
                        <span class="arrow-container"></span> // Spacer
                        <Link<Route> to={Route::About} classes="sidebar-link">
                            <div style="display: flex; align-items: center; width: 100%;" onclick={let log = log_nav.clone(); move |_| log("Opening about.html...")}>
                                <img src="https://raw.githubusercontent.com/vscode-icons/vscode-icons/master/icons/file_type_html.svg" class="file-icon-img" alt="html" />
                                { "about.html" }
                            </div>
                        </Link<Route>>
                    </div>

                    // Blog Folder
                    <div class="sidebar-item">
                         <span class="arrow-container">
                             <i class="codicon codicon-chevron-right"></i>
                         </span>
                        <Link<Route> to={Route::Blog} classes="sidebar-link">
                            <div style="display: flex; align-items: center; width: 100%;" onclick={let log = log_nav.clone(); move |_| log("cd blog && ls -la")}>
                                <img src="https://raw.githubusercontent.com/vscode-icons/vscode-icons/master/icons/default_folder.svg" class="file-icon-img" alt="folder" />
                                { "blog" }
                            </div>
                        </Link<Route>>
                    </div>

                    // Portfolio Folder (Expandable)
                    <div class="sidebar-item-container">
                        <div class="sidebar-item">
                            <span class="arrow-container" onclick={toggle_portfolio}>
                                <i class={classes!("codicon", if *is_portfolio_expanded { "codicon-chevron-down" } else { "codicon-chevron-right" })}></i>
                            </span>
                            <Link<Route> to={Route::Portfolio} classes="sidebar-link">
                                <div style="display: flex; align-items: center; width: 100%;" onclick={let log = log_nav.clone(); move |_| log("cd portfolio && ls -la")}>
                                    <img 
                                        src={if *is_portfolio_expanded { "https://raw.githubusercontent.com/vscode-icons/vscode-icons/master/icons/default_folder_opened.svg" } else { "https://raw.githubusercontent.com/vscode-icons/vscode-icons/master/icons/default_folder.svg" }} 
                                        class="file-icon-img" 
                                        alt="portfolio" 
                                    />
                                    { "portfolio" }
                                </div>
                            </Link<Route>>
                        </div>

                        { if *is_portfolio_expanded {
                            html! {
                                <div class="nested-files">
                                    <div class="sidebar-item">
                                        <span class="arrow-container"></span>
                                        <Link<Route> to={Route::Project { id: "through-their-eyes".to_string() }} classes="sidebar-link">
                                            <div style="display: flex; align-items: center; width: 100%;" onclick={let log = log_nav.clone(); move |_| log("Opening through_their_eyes.proj...")}>
                                                <img src="https://raw.githubusercontent.com/vscode-icons/vscode-icons/master/icons/file_type_vcxproj.svg" class="file-icon-img" alt="proj" />
                                                { "through_their_eyes.proj" }
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
    }
}
