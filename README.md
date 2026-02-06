# Tauri Tray App Boilerplate

Minimal, opinionated Tauri + React (Vite) boilerplate demonstrating a tray‑first
application template. Use this repo as the starting point for building native
tray apps with a web UI.

Features
- Start hidden and run in the system tray.
- Tray icon with left-click show/hide behavior.
- Tray menu with a checkable "Autostart" item and "Quit".
- Autostart integration via `tauri-plugin-autostart` and frontend commands.

Quick Start
1) Install Rust and set the toolchain if needed (project pins rust in
   `rust-toolchain.toml`).
2) Install frontend deps: `bun install` (or `npm install` / `pnpm install`).
3) Run dev server: `bun run tauri dev` (runs Vite then the native binary).

Project layout
- `src/` — React frontend (Vite). Example UI includes autostart toggle.
- `src-tauri/` — Rust backend. Key files: `src/lib.rs`, `src/tray.rs`,
  `src/autostart.rs`, `tauri.conf.json`.

Extend the template
- Add more tray menu items in `src-tauri/src/tray.rs`.
- Expose new backend commands with `#[tauri::command]` and register them in
  `src-tauri/src/lib.rs` (see `generate_handler!`).
- Synchronize frontend UI with backend by emitting events with
  `AppHandle.emit("event-name", payload)` and listening with
  `@tauri-apps/api/event` in the UI.

If you want, I can also:
1) Remove remaining unused artifacts and produce a freshly trimmed
   `Cargo.lock`/lockfile-free template.
2) Create a release-ready example (packaged icons and CI workflow).
