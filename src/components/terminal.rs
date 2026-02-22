use yew::prelude::*;
use crate::context::TerminalContext;
use web_sys::Element;

#[function_component(Terminal)]
pub fn terminal() -> Html {
    let ctx = use_context::<TerminalContext>().expect("No Terminal Context Found");
    let terminal_ref = use_node_ref();

    {
        let terminal_ref = terminal_ref.clone();
        let log_count = ctx.logs.len();
        use_effect_with(log_count, move |_| {
            if let Some(element) = terminal_ref.cast::<Element>() {
                element.set_scroll_top(element.scroll_height());
            }
            || ()
        });
    }
    
    html! {
        <div class="terminal">
            <div class="terminal-header">
                <div class="terminal-tab active">{ "TERMINAL" }</div>
                <div class="terminal-tab">{ "OUTPUT" }</div>
                <div class="terminal-tab">{ "DEBUG CONSOLE" }</div>
                <div class="terminal-tab">{ "PROBLEMS" }</div>
            </div>
            <div class="terminal-content" ref={terminal_ref}>
                { for ctx.logs.iter().map(|log| {
                    let class = match log.level.as_str() {
                        "success" => "log-success",
                        "warn" => "log-warn",
                        "cmd" => "log-cmd",
                        _ => "log-info",
                    };
                    html! {
                        <div class={classes!("log-line", class)}>
                            <span class="log-timestamp">{ "[INFO] " }</span>
                            { &log.message }
                        </div>
                    }
                }) }
                <div class="log-line log-cmd">
                    { "root@portfolio:~/projects# " }<span class="cursor-blink">{ "_" }</span>
                </div>
            </div>
        </div>
    }
}
