---
alwaysApply: false
description: Guidelines for contributing blog posts to the portfolio site.
---
# Blog Post Contribution Guidelines

This document outlines the procedure for adding new blog posts to the portfolio site. Currently, the blog system is implemented as a static collection within the Rust codebase.

## 1. Locate the Blog Component
Navigate to the file:
`src/pages/blog.rs`

## 2. Add a New Post Entry
Find the `posts` vector inside the `blog` function component. It looks like this:

```rust
let posts = vec![
    ("Rust for Embedded Systems", "2025-01-15", "Exploring the benefits of Rust..."),
    // ... existing posts
];
```

To add a new post, insert a new tuple at the beginning of the vector (to keep it chronologically sorted, or wherever appropriate):

```rust
("Your Post Title", "YYYY-MM-DD", "A brief excerpt or summary of the post content."),
```

### Format Requirements
- **Title**: String literal. Keep it concise.
- **Date**: String literal in `YYYY-MM-DD` format.
- **Excerpt**: String literal. A short 1-2 sentence description displayed on the main blog list.

## 3. (Optional) Update Terminal Logs
If you want to reflect the new post count in the "dummy terminal" logs, update the log message in the `use_effect_with` block:

```rust
ctx.dispatch(TerminalAction::Log("Found X entries.".into(), "info".into())); // Update X
```

## 4. Verify Changes
Run the local development server to ensure the new post renders correctly:
```bash
trunk serve
```
Check the Blog page in your browser.

## 5. Commit and Deploy
Once verified, commit your changes to the `main` branch. GitHub Actions will automatically rebuild and deploy the site to GitHub Pages.

```bash
git add src/pages/blog.rs
git commit -m "Add blog post: [Title]"
git push origin main
```
