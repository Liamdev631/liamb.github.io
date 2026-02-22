use yew::prelude::*;
use crate::context::{TerminalContext, TerminalAction};

#[derive(Properties, PartialEq)]
pub struct ProjectProps {
    pub id: String,
}

#[function_component(Project)]
pub fn project(props: &ProjectProps) -> Html {
    let ctx = use_context::<TerminalContext>().expect("No Terminal Context");
    let id = props.id.clone();

    use_effect_with(id.clone(), {
        let ctx = ctx.clone();
        move |id| {
            ctx.dispatch(TerminalAction::Log(format!("Opening project: {}", id), "cmd".into()));
            ctx.dispatch(TerminalAction::Log("Loading project details...".into(), "info".into()));
            ctx.dispatch(TerminalAction::Log("Done.".into(), "success".into()));
        }
    });

    let (title, description, content) = match id.as_str() {
        "through-their-eyes" => (
            "Through Their Eyes",
            "Local LLM-driven game. Players converse with NPCs in real-time to solve complex social issues. Empathy, sympathy, negotiation, manipulation, backmail, and extortion are all key skills in this game where players must navigate the complexities of human relationships to achieve their goals.",
            html! {
                <div class="blog-post-content" style="max-width: 800px;">
                    <p style="font-style: italic; color: #888;">{ "Coming soon..." }</p>
                    // Blank blog post structure ready for content
                </div>
            }
        ),
        "cpu-ray-tracer" => (
            "CPU Ray Tracer",
            "A high-performance software ray tracer built from scratch in C++20.",
            html! {
                <div class="project-content-wrapper" style="max-width: 800px;">
                    <div class="video-container" style="position: relative; padding-bottom: 56.25%; height: 0; overflow: hidden; max-width: 100%; background: #000; margin-bottom: 20px;">
                        <iframe 
                            src="https://www.youtube.com/embed/o9TMVNlaug4" 
                            title="CPU Ray Tracer Demo"
                            frameborder="0" 
                            allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" 
                            allowfullscreen=true
                            style="position: absolute; top: 0; left: 0; width: 100%; height: 100%;"
                        ></iframe>
                    </div>
                    
                    <h3>{ "Project Summary" }</h3>
                    <p>{ "An extremely-poor-performance, multithreaded CPU-based Ray Tracer built from scratch in C++20. I created this project back in 2018 to play around with ray tracing and its techniques. Each new feature I added quartered the rendering time, but I have better hardware now so I've wanted to try a few new things and recently rebooted the project." }</p>
                    <p>
                        { "Check out the source code on GitHub: " }
                        <a href="https://github.com/Liamdev631/RayTracingDemo.git" target="_blank" style="color: #569cd6; text-decoration: none;">{ "https://github.com/Liamdev631/RayTracingDemo.git" }</a>
                    </p>

                    <h3>{ "Major Features" }</h3>
                    <ul style="list-style-type: disc; margin-left: 20px;">
                        <li><strong>{ "Multithreaded Rendering:" }</strong>{ " Parallelizes rendering tasks across all available CPU cores using std::thread for optimal performance." }</li>
                        <li><strong>{ "Physically Based Rendering (PBR):" }</strong>{ " Supports material properties including Albedo, Normal, Roughness, and Metallic maps for realistic surface interactions." }</li>
                        <li><strong>{ "Global Illumination:" }</strong>{ " Implements recursive ray tracing with configurable bounce depth and ambient light simulation." }</li>
                        <li><strong>{ "Advanced Sampling:" }</strong>{ " Features Stratified Sampling (jittered) for high-quality anti-aliasing and noise reduction." }</li>
                        <li><strong>{ "Flexible Scene System:" }</strong>{ " JSON-based scene description format allows for easy definition of geometry, lights, and camera settings." }</li>
                    </ul>

                    <h3>{ "Tech Stack" }</h3>
                    <ul style="list-style-type: disc; margin-left: 20px;">
                        <li><strong>{ "Language:" }</strong>{ " C++20" }</li>
                        <li><strong>{ "Build System:" }</strong>{ " CMake" }</li>
                        <li><strong>{ "Graphics:" }</strong>{ " SFML (Simple and Fast Multimedia Library)" }</li>
                        <li><strong>{ "Math:" }</strong>{ " GLM (OpenGL Mathematics)" }</li>
                        <li><strong>{ "Utilities:" }</strong>{ " nlohmann/json, CLI11, stb_image" }</li>
                    </ul>
                </div>
            }
        ),
        _ => (
            "Project Not Found",
            "The requested project file could not be read.",
            html! {}
        ),
    };

    html! {
        <div class="project-detail">
            <h1>{ title }</h1>
            <p class="subtitle">{ description }</p>
            <div class="project-content">
                { content }
            </div>
        </div>
    }
}
