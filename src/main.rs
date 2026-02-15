use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <header>
                <h1>{ "Liam's Portfolio" }</h1>
                <p>{ "Rust Developer | Open Source Enthusiast" }</p>
                <div class="social-links">
                    <a href="https://github.com/liamb" target="_blank">{ "GitHub" }</a>
                    <a href="mailto:liam@example.com">{ "Email" }</a>
                </div>
            </header>

            <main>
                <section>
                    <h2>{ "About Me" }</h2>
                    <p>
                        { "Hi! I'm Liam. This portfolio is built with Rust and Yew, compiled to WebAssembly. 
                           I love building efficient and reliable software." }
                    </p>
                </section>

                <section>
                    <h2>{ "Projects" }</h2>
                    <div class="project-card">
                        <h3>{ "Project One" }</h3>
                        <p>{ "A cool project written in Rust." }</p>
                    </div>
                    <div class="project-card">
                        <h3>{ "Project Two" }</h3>
                        <p>{ "Another awesome application." }</p>
                    </div>
                </section>
            </main>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
