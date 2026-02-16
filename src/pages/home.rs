use yew::prelude::*;
use crate::context::{TerminalContext, TerminalAction};

#[function_component(Home)]
pub fn home() -> Html {
    let ctx = use_context::<TerminalContext>();

    use_effect_with((), {
        let ctx = ctx.clone();
        move |_| {
            if let Some(ctx) = ctx {
                ctx.dispatch(TerminalAction::Log("Initialized Home Environment".into(), "success".into()));
                ctx.dispatch(TerminalAction::Log("Loading social modules...".into(), "info".into()));
                ctx.dispatch(TerminalAction::Log("Ready.".into(), "success".into()));
            }
        }
    });

    let icon_style = "width: 48px; height: 48px; object-fit: contain; transition: transform 0.2s;";

    html! {
        <div class="home-container" style="display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100%; text-align: center;">
            <header class="hero" style="margin-bottom: 40px;">
                <h1 style="font-size: 3em; margin-bottom: 10px;">{ "Liam Bury (利昂)" }</h1>
                <p class="subtitle" style="font-size: 1.2em; color: #cccccc;">{ "AI Engineer & Game Developer" }</p>
            </header>

            <section class="socials-section">
                <h2 style="margin-bottom: 20px;">{ "Connect with Me" }</h2>
                <div class="social-links" style="display: flex; gap: 20px; justify-content: center; align-items: center;">
                    <a href="https://github.com/Liamdev631" target="_blank" title="GitHub">
                        <img src="https://skillicons.dev/icons?i=github" alt="GitHub" style={icon_style} />
                    </a>
                    <a href="https://www.linkedin.com/in/liambury" target="_blank" title="LinkedIn">
                        <img src="https://skillicons.dev/icons?i=linkedin" alt="LinkedIn" style={icon_style} />
                    </a>
                    <a href="https://www.instagram.com/bury_liam/" target="_blank" title="Instagram">
                        <img src="https://skillicons.dev/icons?i=instagram" alt="Instagram" style={icon_style} />
                    </a>
                    <a href="https://discordapp.com/users/chengduum" target="_blank" title="Discord">
                        <img src="https://skillicons.dev/icons?i=discord" alt="Discord" style={icon_style} />
                    </a>
                </div>
            </section>
            
            <style>
                { "
                    .social-links img:hover {
                        transform: scale(1.1);
                    }
                " }
            </style>
        </div>
    }
}
