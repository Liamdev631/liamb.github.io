# Liam's Rust Portfolio

This is a Rust-based portfolio built with [Yew](https://yew.rs/) and compiled to WebAssembly using [Trunk](https://trunkrs.dev/).

## Local Development

To run this project locally, you need to have Rust and `trunk` installed.

1.  **Install Rust**: https://rustup.rs/
2.  **Install Trunk**:
    ```bash
    cargo install trunk
    ```
3.  **Run the development server**:
    ```bash
    trunk serve
    ```
    Open your browser at `http://localhost:8080`.

## Deployment

This project is configured to deploy automatically to GitHub Pages using GitHub Actions.
Any push to the `main` branch will trigger a build and deployment to the `gh-pages` branch.

Ensure your repository settings for GitHub Pages are configured to serve from the `gh-pages` branch.
