use yew::prelude::*;
use crate::context::{TerminalContext, TerminalAction};

#[derive(Clone, PartialEq)]
struct SkillCategory {
    title: &'static str,
    icons: &'static str, // Comma separated list for skill-icons.dev
}

#[function_component(About)]
pub fn about() -> Html {
    let ctx = use_context::<TerminalContext>();

    use_effect_with((), {
        let ctx = ctx.clone();
        move |_| {
            if let Some(ctx) = ctx {
                ctx.dispatch(TerminalAction::Log("Reading about.md...".into(), "cmd".into()));
                ctx.dispatch(TerminalAction::Log("Parsing personal data...".into(), "info".into()));
            }
        }
    });

    let categories = vec![
        SkillCategory {
            title: "AI Research & Development",
            icons: "pytorch,python,opencv,tensorflow,matlab,latex",
        },
        SkillCategory {
            title: "Systems & Embedded Engineering",
            icons: "cpp,c,linux,ros,arduino,raspberrypi",
        },
        SkillCategory {
            title: "DevOps & Tools",
            icons: "docker,kubernetes,githubactions,cmake,git,vim,vscode",
        },
        SkillCategory {
            title: "Full Stack & Simulation",
            icons: "rust,yew,wasm,unreal,unity,react",
        },
    ];

    html! {
        <div class="container about-container" style="display: flex; gap: 40px; flex-wrap: wrap; font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;">
            // LEFT PANE
            <div class="left-pane" style="flex: 1; min-width: 300px;">
                <header>
                    <h1>{ "About Me" }</h1>
                </header>

                <section class="summary">
                    <p style="font-size: 1.1em; line-height: 1.6; color: #cccccc;">
                        { "I'm Liam Bury. I've been writing code since I was 12, tinkering with everything from microcontrollers and robotics to video games and now AI. Engineering in all its forms is where my spirit and passion lie — I just can't stop designing and building." }
                    </p>
                    <p style="font-size: 1.1em; line-height: 1.6; color: #cccccc;">
                        { "I love using science and technology to expand the world (and I love sci-fi — especially The Expanse). My current passion is exploring every realm of possibility in AI, building things that make people rethink the reality they live in." }
                    </p>
                </section>

                <section class="education" style="margin-top: 40px;">
                    <h2>{ "Education" }</h2>
                    <div class="education-item" style="margin-bottom: 20px;">
                        <h3 style="margin-bottom: 5px;">{ "MS Computer Science" }</h3>
                        <div class="education-meta" style="color: #9cdcfe; margin-bottom: 5px;">{ "University of Electronic Science and Technology of China (2024–2026)" }</div>
                        <p style="margin: 0; opacity: 0.8;">{ "Thesis: Digital–Analog Hybrid Neuromorphic Hardware via Stochastic Computing" }</p>
                    </div>
                    <div class="education-item">
                        <h3 style="margin-bottom: 5px;">{ "BS Mathematics" }</h3>
                        <div class="education-meta" style="color: #9cdcfe; margin-bottom: 5px;">{ "Vermont State College (2015–2019)" }</div>
                        <p style="margin: 0; opacity: 0.8;">{ "3.7 GPA, Citizen Scholarship, President Chinese Club & ESports Club" }</p>
                    </div>
                </section>
            </div>

            // RIGHT PANE
            <div class="right-pane" style="flex: 1; min-width: 300px;">
                <section class="tech-stack">
                    <h2 style="border-bottom: 1px solid #3e3e42; padding-bottom: 10px; margin-bottom: 20px;">{ "Tech Stack" }</h2>
                    <div class="skills-grid">
                        { for categories.iter().map(|category| {
                            html! {
                                <div class="skill-category" style="margin-bottom: 30px;">
                                    <h3 style="margin-bottom: 15px; color: #dcdcaa;">{ category.title }</h3>
                                    <div class="skill-icons">
                                        <img src={format!("https://skillicons.dev/icons?i={}", category.icons)} alt={category.title} />
                                    </div>
                                </div>
                            }
                        }) }
                    </div>
                </section>
            </div>
        </div>
    }
}
