use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;
use crate::components::sidebar::Sidebar;

#[derive(Clone, PartialEq)]
struct TabEntry {
    route: Route,
    label: String,
    icon: Html,
}

fn route_to_tab(route: &Route) -> TabEntry {
    match route {
        Route::Home => TabEntry {
            route: Route::Home,
            label: "Home".to_string(),
            icon: html! { <img src="https://raw.githubusercontent.com/vscode-icons/vscode-icons/master/icons/file_type_html.svg" class="file-icon-img" alt="html" /> },
        },
        Route::About => TabEntry {
            route: Route::About,
            label: "About".to_string(),
            icon: html! { <img src="https://raw.githubusercontent.com/vscode-icons/vscode-icons/master/icons/file_type_html.svg" class="file-icon-img" alt="html" /> },
        },
        Route::Blog => TabEntry {
            route: Route::Blog,
            label: "Blog".to_string(),
            icon: html! { <img src="https://raw.githubusercontent.com/vscode-icons/vscode-icons/master/icons/default_folder.svg" class="file-icon-img" alt="blog" /> },
        },
        Route::BlogTag { tag } => TabEntry {
            route: Route::BlogTag { tag: tag.clone() },
            label: format!("Tag: {}", tag),
            icon: html! { <img src="https://raw.githubusercontent.com/vscode-icons/vscode-icons/master/icons/default_folder.svg" class="file-icon-img" alt="tag" /> },
        },
        Route::Portfolio => TabEntry {
            route: Route::Portfolio,
            label: "Portfolio".to_string(),
            icon: html! { <img src="https://raw.githubusercontent.com/vscode-icons/vscode-icons/master/icons/default_folder.svg" class="file-icon-img" alt="portfolio" /> },
        },
        Route::Project { id } => match id.as_str() {
            "through-their-eyes" => TabEntry {
                route: Route::Project { id: id.clone() },
                label: "Through Their Eyes".to_string(),
                icon: html! { <img src="https://raw.githubusercontent.com/vscode-icons/vscode-icons/master/icons/file_type_vcxproj.svg" class="file-icon-img" alt="proj" /> },
            },
            "cpu-ray-tracer" => TabEntry {
                route: Route::Project { id: id.clone() },
                label: "CPU Ray Tracer".to_string(),
                icon: html! { <img src="https://raw.githubusercontent.com/vscode-icons/vscode-icons/master/icons/file_type_vcxproj.svg" class="file-icon-img" alt="proj" /> },
            },
            _ => TabEntry {
                route: Route::Project { id: id.clone() },
                label: "Unknown Project".to_string(),
                icon: html! { <img src="https://raw.githubusercontent.com/vscode-icons/vscode-icons/master/icons/default_file.svg" class="file-icon-img" alt="file" /> },
            },
        },
        Route::BlogPost { id } => TabEntry {
            route: Route::BlogPost { id: id.clone() },
            label: id.clone(),
            icon: html! { <img src="https://raw.githubusercontent.com/vscode-icons/vscode-icons/master/icons/file_type_markdown.svg" class="file-icon-img" alt="md" /> },
        },
        Route::NotFound => TabEntry {
            route: Route::NotFound,
            label: "404 Not Found".to_string(),
            icon: html! { <img src="https://raw.githubusercontent.com/vscode-icons/vscode-icons/master/icons/file_type_rust.svg" class="file-icon-img" alt="rs" /> },
        },
    }
}

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    pub children: Html,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    let navigator = use_navigator().unwrap();
    let route = use_route::<Route>().unwrap_or(Route::Home);
    let tabs = use_state(|| vec![route_to_tab(&route)]);
    let drag_index = use_state(|| None::<usize>);

    {
        let tabs = tabs.clone();
        let route = route.clone();
        use_effect_with(route.clone(), move |_| {
            let mut next = (*tabs).clone();
            if !next.iter().any(|tab| tab.route == route) {
                next.push(route_to_tab(&route));
                tabs.set(next);
            }
            || ()
        });
    }

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
                    { for (*tabs).iter().cloned().enumerate().map(|(index, tab)| {
                        let is_active = tab.route == route;
                        let navigator = navigator.clone();
                        let tabs = tabs.clone();
                        let route_current = route.clone();
                        let drag_index = drag_index.clone();
                        let tab_route = tab.route.clone();
                        let tab_route_close = tab.route.clone();

                        let navigator_for_click = navigator.clone();
                        let on_click = Callback::from(move |_| {
                            navigator_for_click.push(&tab_route);
                        });

                        let drag_index_start = drag_index.clone();
                        let on_drag_start = Callback::from(move |event: DragEvent| {
                            event.prevent_default();
                            drag_index_start.set(Some(index));
                        });

                        let on_drag_over = Callback::from(|event: DragEvent| {
                            event.prevent_default();
                        });

                        let tabs_for_drop = tabs.clone();
                        let drag_index_for_drop = drag_index.clone();
                        let on_drop = Callback::from(move |event: DragEvent| {
                            event.prevent_default();
                            if let Some(from) = *drag_index_for_drop {
                                let to = index;
                                if from != to {
                                    let mut next = (*tabs_for_drop).clone();
                                    let item = next.remove(from);
                                    next.insert(to, item);
                                    tabs_for_drop.set(next);
                                }
                            }
                            drag_index_for_drop.set(None);
                        });

                        let tabs_for_close = tabs.clone();
                        let navigator_for_close = navigator.clone();
                        let on_close = Callback::from(move |event: MouseEvent| {
                            event.stop_propagation();
                            let mut next = (*tabs_for_close).clone();
                            if let Some(pos) = next.iter().position(|t| t.route == tab_route_close) {
                                next.remove(pos);
                                tabs_for_close.set(next.clone());
                                if route_current == tab_route_close {
                                    let new_route = if pos > 0 {
                                        next.get(pos - 1)
                                    } else {
                                        next.get(0)
                                    }
                                    .map(|t| t.route.clone())
                                    .unwrap_or(Route::Home);
                                    navigator_for_close.push(&new_route);
                                }
                            }
                        });

                        html! {
                            <div
                                class={classes!("tab", if is_active { "active" } else { "" })}
                                draggable="true"
                                onclick={on_click}
                                ondragstart={on_drag_start}
                                ondragover={on_drag_over}
                                ondrop={on_drop}
                            >
                                { tab.icon.clone() }
                                { tab.label.clone() }
                                <span class="tab-close" style="margin-left: auto; opacity: 0.5;" onclick={on_close}>{ "×" }</span>
                            </div>
                        }
                    }) }
                </div>
                <div class="content-container">
                    { props.children.clone() }
                </div>
            </div>
            
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
