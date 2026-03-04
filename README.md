# Free Postman

A lightweight, fast API client for Windows. No accounts, no AI features, no upgrade nags, no telemetry. Just endpoints.

Built with **Tauri v2** (Rust backend) and **Svelte 5** (frontend). The final binary is ~10MB with native performance.

## Features

- **Unlimited workspaces** — organize projects independently
- **Nested folders** — group endpoints with infinite nesting
- **Request editor** — method, URL, params, headers, auth, body
- **Auth support** — None, Bearer Token, Basic Auth
- **Body types** — JSON, raw text, form URL-encoded
- **Response viewer** — status, time, size, headers, pretty-printed body
- **Dracula theme** — easy on the eyes
- **Persistent storage** — SQLite local database, your data stays on your machine
- **Keyboard shortcuts** — Ctrl+Enter to send

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Backend | Rust, Tauri v2, reqwest, rusqlite |
| Frontend | Svelte 5, TypeScript |
| Storage | SQLite (local) |
| HTTP engine | reqwest with rustls |

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (1.70+)
- [Node.js](https://nodejs.org/) (20+)
- [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) (pre-installed on Windows 10/11)

### Development

```bash
npm install
npx tauri dev
```

### Build for production

```bash
npx tauri build
```

The installer will be in `src-tauri/target/release/bundle/`.

## Project Structure

```
src/                    # Svelte frontend
  lib/
    api/                # Tauri invoke wrappers
    components/         # UI components (Sidebar, Request, shared)
    stores/             # Reactive state (workspaces, active request)
    theme/              # Dracula color tokens
src-tauri/              # Rust backend
  src/
    commands/           # Tauri commands (workspace, folder, request, http)
    models/             # Data structures
    db.rs               # SQLite init and migrations
    main.rs             # Entry point
```
