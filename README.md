# timeman — Tauri Tray App Boilerplate

timeman is a minimal, opinionated tray-first desktop app built with Tauri (Rust)
and a React (Vite) frontend. It demonstrates a compact, production-minded
architecture for tray utilities: the app lives in the system tray by default,
supports toggling the main window with a left-click, exposes an action menu on
right-click, and includes an autostart integration.

This repository is a solid starting point for building utilities like clocks,
timers, background workers, or any small tool where the tray is the primary
interface.

Highlights
- Tray-first UX: app starts hidden with a tray icon and a small, focused UI.
- Left-click toggles show/hide (hides to tray using `window.hide()`).
- Right-click opens a native context menu with actions (autostart, Show/Hide,
  Quit).
- Autostart integration using `tauri-plugin-autostart` with frontend sync via
  events.
- Small, easy-to-extend Rust backend (`src-tauri/`) and React frontend (`src/`).

Quick start (development)
1. Install Rust (recommended toolchain: 1.88.0) and ensure MSVC build tools are
   available on Windows.
   - Optionally pin the toolchain locally with: `rustup override set 1.88.0`.
2. Install frontend dependencies:
   - `bun install` (recommended) or `npm install` / `pnpm install`.
3. Run the dev environment (starts Vite and the native Tauri binary):
   - `bun run tauri dev`

Build and package
- Build the native binary: `cd src-tauri && cargo +1.88.0 build --release`
- Create OS installers using `tauri build` (follow Tauri docs for platform
  specific signing and packaging requirements).

Repository layout
- `src/` — React (Vite) frontend. Contains example UI (autostart toggle) and
  listens for backend events like `autostart-changed`.
- `src-tauri/` — Rust backend. Key files:
  - `src/` — Rust app code (entry points, tray builder, autostart glue).
  - `src/tray.rs` — tray menu and click handling (left-click toggles, right
    click opens menu). This file contains the UX logic you’ll likely extend.
  - `src/autostart.rs` — wrapper around `tauri-plugin-autostart`.
  - `tauri.conf.json` — Tauri config; app starts hidden by default.
- `src-tauri/icons/` — packaged icons generated with
  `npx @tauri-apps/cli icon src-tauri/icons/clock.svg`.

UX & behavior notes
- Left-click is intentionally implemented to strictly toggle show/hide. The
  window is hidden using `window.hide()` so it is removed from the taskbar.
- Right-click shows the native context menu. On some Linux desktops and macOS
  the tray behavior differs slightly — the implementation includes small
  fallbacks where necessary.
- Clicks are debounced (200ms) to avoid accidental double toggles.

Extending the project
- Add menu items in `src-tauri/src/tray.rs` and handle them with
  `.on_menu_event(...)`.
- Expose new backend functions with `#[tauri::command]` and register them in
  `src-tauri/src/lib.rs` using `generate_handler!`.
- Use `AppHandle.emit("event-name", payload)` to notify the frontend of
  state changes and listen in the UI using `@tauri-apps/api/event`.

Contributing
- File a PR for features or bug fixes. The `feat/tray-work` branch contains
  a recent tray UX improvement: left-click toggle, right-click menu, debounce,
  and tooltip updates.

Troubleshooting
- If the Rust build fails on Windows, ensure Visual Studio Build Tools and the
  Windows SDK are installed (select "Desktop development with C++").
- If tray icons or menus behave differently on your platform, test directly on
  that OS — system tray behavior varies across environments.

License
- This project is open-source. Add your preferred license file (`LICENSE`) to
  the repo.

Questions or help
- If you'd like, I can:
  1) Polish the README further with screenshots and code snippets.
  2) Add a CI workflow to build on Windows/Linux/macOS.
  3) Create release artifacts and a packaged installer example.

Enjoy building with timeman!
