use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;
use crate::components::sidebar::Sidebar;
use crate::components::terminal::Terminal;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    pub children: Html,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    let route = use_route::<Route>().unwrap_or(Route::Home);
    
    let (active_file, active_icon_html) = match route {
        Route::Home => ("home.html", html! { <img src="https://raw.githubusercontent.com/vscode-icons/vscode-icons/master/icons/file_type_html.svg" class="file-icon-img" alt="html" /> }),
        Route::About => ("about.html", html! { <img src="https://raw.githubusercontent.com/vscode-icons/vscode-icons/master/icons/file_type_html.svg" class="file-icon-img" alt="html" /> }),
        Route::Blog => ("blog", html! { <img src="https://raw.githubusercontent.com/vscode-icons/vscode-icons/master/icons/default_folder.svg" class="file-icon-img" alt="blog" /> }),
        Route::Portfolio => ("portfolio", html! { <img src="https://raw.githubusercontent.com/vscode-icons/vscode-icons/master/icons/default_folder.svg" class="file-icon-img" alt="portfolio" /> }),
        Route::Project { id } => match id.as_str() {
            "through-their-eyes" => ("through_their_eyes.proj", html! { <img src="https://raw.githubusercontent.com/vscode-icons/vscode-icons/master/icons/file_type_vcxproj.svg" class="file-icon-img" alt="proj" /> }),
            _ => ("unknown.proj", html! { <img src="https://raw.githubusercontent.com/vscode-icons/vscode-icons/master/icons/default_file.svg" class="file-icon-img" alt="file" /> }),
        },
        Route::NotFound => ("404.rs", html! { <img src="https://raw.githubusercontent.com/vscode-icons/vscode-icons/master/icons/file_type_rust.svg" class="file-icon-img" alt="rs" /> }),
    };

    html! {
        <div class="vscode-layout">
            <div class="activity-bar">
                <div class="activity-icon active">
                    <i class="codicon codicon-files" style="font-size: 24px;"></i>
                </div>
                <div class="activity-icon">
                    <i class="codicon codicon-search" style="font-size: 24px;"></i>
                </div>
                <div class="activity-icon">
                    <i class="codicon codicon-source-control" style="font-size: 24px;"></i>
                </div>
                <div class="activity-icon">
                    <i class="codicon codicon-debug-alt" style="font-size: 24px;"></i>
                </div>
                <div class="activity-icon">
                    <i class="codicon codicon-extensions" style="font-size: 24px;"></i>
                </div>
            </div>
            
            <Sidebar />
            
            <div class="editor-area">
                <div class="tab-bar">
                    <div class="tab active">
                        { active_icon_html }
                        { active_file }
                        <span class="tab-close" style="margin-left: 10px; opacity: 0.5;">{ "×" }</span>
                    </div>
                </div>
                <div class="content-container">
                    { props.children.clone() }
                </div>
            </div>
            
            <Terminal />
            
            <div class="status-bar">
                <div class="status-left" style="display: flex;">
                    <div class="status-item">
                        <i class="codicon codicon-source-control" style="margin-right: 5px; font-size: 12px;"></i>
                        <span style="margin-right: 5px">{ "main*" }</span>
                    </div>
                    <div class="status-item">
                        <i class="codicon codicon-sync" style="margin-right: 5px; font-size: 12px;"></i>
                    </div>
                    <div class="status-item">
                        <span style="margin-right: 5px">{ "⊗ 0" }</span>
                        <span>{ "⚠ 0" }</span>
                    </div>
                </div>
                <div class="status-right" style="display: flex;">
                    <div class="status-item">{ "Ln 42, Col 1" }</div>
                    <div class="status-item">{ "UTF-8" }</div>
                    <div class="status-item">{ "Rust" }</div>
                    <div class="status-item">{ "Prettier" }</div>
                    <div class="status-item">
                        <i class="codicon codicon-bell" style="font-size: 12px;"></i>
                    </div>
                </div>
            </div>
        </div>
    }
}
