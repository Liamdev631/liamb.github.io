
#[derive(Clone, Debug, PartialEq)]
pub struct BlogPost {
    pub id: &'static str,
    pub title: &'static str,
    pub date: &'static str,
    pub excerpt: &'static str,
    pub tags: Vec<&'static str>,
}

pub fn get_all_posts() -> Vec<BlogPost> {
    vec![
        BlogPost {
            id: "ray-tracer-transparency",
            title: "Adding Transparency to My CPU Ray Tracer",
            date: "2026-02-17",
            excerpt: "Transparency is a crucial feature for realistic rendering, allowing light to pass through objects like glass, water, or thin fabrics.",
            tags: vec!["Graphics", "Rust", "C++"],
        },
        BlogPost {
            id: "yew-journey",
            title: "My Journey with Yew",
            date: "2025-02-14",
            excerpt: "Building a portfolio site with Rust and WebAssembly.",
            tags: vec!["Rust", "WebAssembly", "Yew"],
        },
        BlogPost {
            id: "offline-ai",
            title: "Building Offline AI Agents",
            date: "2025-02-01",
            excerpt: "How to run local LLMs privacy-first with NVIDIA NeMo.",
            tags: vec!["AI", "Local LLM"],
        },
        BlogPost {
            id: "rust-embedded",
            title: "Rust for Embedded Systems",
            date: "2025-01-15",
            excerpt: "Exploring the benefits of Rust in resource-constrained environments.",
            tags: vec!["Rust", "Embedded"],
        },
    ]
}
