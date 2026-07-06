# Liam's Portfolio

A plain HTML/CSS/JS GitHub Pages portfolio for Liam Bury. No build step, no dependencies.

## Stack

- HTML
- CSS (one stylesheet in `assets/style.css`)
- JavaScript (none required at the moment)
- GitHub Pages for hosting

## Local Development

Just open `index.html` in a browser, or serve the folder with any static server, for example:

```bash
python -m http.server 8000
```

Then visit `http://localhost:8000`.

## Layout

- `index.html` — Home
- `about.html` — About
- `portfolio.html` — Portfolio index
- `portfolio/*.html` — Individual project pages
- `assets/style.css` — Shared stylesheet

## Deployment

Pushing to `main` publishes the site via GitHub Pages. In the repository settings, GitHub Pages should be configured with:

- **Source:** Deploy from a branch
- **Branch:** `main` / root

No workflow or build step is required.
