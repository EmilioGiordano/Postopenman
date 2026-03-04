# Free Postman - Project Guidelines

## Rust Conventions
- Rust 2021 edition, ownership-first design
- Never use unwrap() in production — use expect() with descriptive messages or proper error handling with Result/Option
- Minimize unsafe code; document any unsafe block
- Use thiserror for library errors, anyhow for application errors
- Trait-based architecture with generics where appropriate
- Async with tokio runtime
- All code must pass clippy and cargo fmt
- No comments unless logic is non-obvious

## Frontend Conventions
- Svelte 5 with TypeScript
- No comments in code
- Modular components, one responsibility per file

## Project Structure
- Tauri v2 + Svelte 5 + SQLite
- Backend: src-tauri/src/
- Frontend: src/
