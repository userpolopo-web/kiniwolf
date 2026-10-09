# Rust WebView Browser Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a lightweight Rust desktop browser for Windows using WebView2, with a modern black, gray, and white interface and basic real navigation.

**Architecture:** Create a Cargo binary that uses `wry` and `tao` to open a native window with a WebView. Keep pure navigation normalization logic in a small testable module, and keep the browser shell HTML/CSS/JS in static assets embedded into the Rust binary.

**Tech Stack:** Rust 2021, Cargo, `wry`, `tao`, `url`, WebView2 on Windows, HTML/CSS/JavaScript assets.

**Spec:** `docs/superpowers/specs/2026-10-08-rust-webview-browser-design.md`

## Global Constraints

- The app is a native Rust desktop application.
- The renderer uses WebView2 through the `wry` ecosystem.
- The interface uses black, gray, and white only.
- Typed domains open as HTTPS URLs.
- Free text searches use DuckDuckGo.
- The first version excludes tabs, bookmarks, downloads UI, extensions, password management, custom engine work, and privacy sandboxing.
- Automated tests cover URL normalization.
- Manual verification includes `cargo check` and, when possible, launching the app on Windows.

## Review Focus

- Empty address input keeps the current page unchanged; covered in Task 2 URL normalization tests.
- Bare domains with paths such as `example.com/docs` become `https://example.com/docs`; covered in Task 2 URL normalization tests.
- Full HTTP URLs such as `http://localhost:3000` are preserved; covered in Task 2 URL normalization tests.
- Search text with spaces is encoded into a DuckDuckGo query; covered in Task 2 URL normalization tests.
- UI commands from JavaScript reject empty navigation messages; covered in Task 4 command parsing tests.

---

## File Structure

- `Cargo.toml`: package metadata and dependencies.
- `src/main.rs`: application startup, event loop, window, WebView construction, and IPC command handling.
- `src/browser.rs`: pure browser helpers, especially URL normalization.
- `src/assets.rs`: embedded HTML/CSS/JS asset strings and shell construction.
- `assets/shell.html`: browser shell UI markup.
- `assets/styles.css`: monochrome browser shell styling.
- `assets/shell.js`: toolbar behavior and IPC message sending.

### Task 1: Cargo Project Scaffold

**Files:**
- Create: `Cargo.toml`
- Create: `src/main.rs`
- Create: `.gitignore`

**Interfaces:**
- Consumes: none.
- Produces: Cargo binary package named `kiniwolf-browser`.

- [ ] **Step 1: Create the Cargo manifest**

Create `Cargo.toml` with package name `kiniwolf-browser`, Rust edition `2021`, and dependencies `wry`, `tao`, and `url`.

- [ ] **Step 2: Create a minimal entry point**

Create `src/main.rs` with `fn main() { println!("Kiniwolf Browser"); }`.

- [ ] **Step 3: Add Rust build ignores**

Create `.gitignore` with `target/` and common local editor/temp ignores.

- [ ] **Step 4: Verify the scaffold**

Run: `cargo check`

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add Cargo.toml src/main.rs .gitignore
git commit -m "chore: scaffold Rust browser app"
```

### Task 2: URL Normalization

**Files:**
- Create: `src/browser.rs`
- Modify: `src/main.rs`

**Interfaces:**
- Consumes: Cargo package from Task 1.
- Produces: `pub fn normalize_navigation_input(input: &str) -> Option<String>`.

- [ ] **Step 1: Write failing unit tests**

In `src/browser.rs`, add tests for:

```rust
assert_eq!(normalize_navigation_input(""), None);
assert_eq!(normalize_navigation_input(" example.com "), Some("https://example.com".to_string()));
assert_eq!(normalize_navigation_input("example.com/docs"), Some("https://example.com/docs".to_string()));
assert_eq!(normalize_navigation_input("https://example.com"), Some("https://example.com/".to_string()));
assert_eq!(normalize_navigation_input("http://localhost:3000"), Some("http://localhost:3000/".to_string()));
assert_eq!(
    normalize_navigation_input("rust webview browser"),
    Some("https://duckduckgo.com/?q=rust%20webview%20browser".to_string())
);
```

- [ ] **Step 2: Run tests to verify failure**

Run: `cargo test browser::tests -- --nocapture`

Expected: FAIL because `normalize_navigation_input` is not implemented.

- [ ] **Step 3: Implement `normalize_navigation_input(input: &str) -> Option<String>`**

Use `url::Url` for final URL parsing and serialization. Treat empty trimmed input as `None`, preserve explicit `http://` and `https://`, add `https://` for inputs that look like domains or localhost addresses, and encode all other text as a DuckDuckGo search query.

- [ ] **Step 4: Wire the module**

Add `mod browser;` in `src/main.rs`.

- [ ] **Step 5: Verify tests**

Run: `cargo test browser::tests -- --nocapture`

Expected: PASS.

- [ ] **Step 6: Commit**

```bash
git add src/browser.rs src/main.rs Cargo.toml
git commit -m "feat: add navigation input normalization"
```

### Task 3: Embedded Browser Shell Assets

**Files:**
- Create: `assets/shell.html`
- Create: `assets/styles.css`
- Create: `assets/shell.js`
- Create: `src/assets.rs`
- Modify: `src/main.rs`

**Interfaces:**
- Consumes: `normalize_navigation_input(input: &str) -> Option<String>` from Task 2.
- Produces: `pub fn browser_shell_html(home_url: &str) -> String`.

- [ ] **Step 1: Write asset smoke tests**

In `src/assets.rs`, add tests that assert `browser_shell_html("https://duckduckgo.com")` contains:

```rust
"id=\"address\""
"data-home=\"https://duckduckgo.com\""
"shell.js"
"styles.css"
```

- [ ] **Step 2: Run tests to verify failure**

Run: `cargo test assets::tests -- --nocapture`

Expected: FAIL because `browser_shell_html` is not implemented.

- [ ] **Step 3: Implement shell assets**

Create a top toolbar with buttons `back`, `forward`, `reload`, `home`, `go`, and an input with `id="address"`. Style it in black, gray, and white. In `shell.js`, send IPC messages shaped as JSON: `{ "type": "navigate", "value": "<input>" }`, `{ "type": "back" }`, `{ "type": "forward" }`, `{ "type": "reload" }`, and `{ "type": "home" }`.

- [ ] **Step 4: Implement `browser_shell_html(home_url: &str) -> String`**

Embed the three asset files with `include_str!`, inject `home_url` into a `data-home` attribute, and include the CSS and JS inline or with local asset references that Wry can load reliably.

- [ ] **Step 5: Wire the module**

Add `mod assets;` in `src/main.rs`.

- [ ] **Step 6: Verify tests**

Run: `cargo test assets::tests -- --nocapture`

Expected: PASS.

- [ ] **Step 7: Commit**

```bash
git add assets src/assets.rs src/main.rs
git commit -m "feat: add minimal browser shell"
```

### Task 4: Native Window, WebView, And IPC Commands

**Files:**
- Modify: `src/main.rs`
- Modify: `src/browser.rs`

**Interfaces:**
- Consumes: `browser_shell_html(home_url: &str) -> String` from Task 3 and `normalize_navigation_input(input: &str) -> Option<String>` from Task 2.
- Produces: a runnable native browser window.

- [ ] **Step 1: Add command parsing tests**

In `src/browser.rs`, add `pub enum BrowserCommand { Navigate(String), Back, Forward, Reload, Home }` and `pub fn parse_browser_command(message: &str) -> Option<BrowserCommand>`. Tests must verify valid command JSON parses and empty navigate values return `None`.

- [ ] **Step 2: Run tests to verify failure**

Run: `cargo test browser::tests -- --nocapture`

Expected: FAIL because command parsing is not implemented.

- [ ] **Step 3: Add dependencies for command parsing**

Add `serde` with `derive` and `serde_json` to `Cargo.toml`.

- [ ] **Step 4: Implement command parsing**

Parse the IPC JSON into an internal deserializable struct. Convert navigation values through `normalize_navigation_input`; return `None` when normalization returns `None`.

- [ ] **Step 5: Implement the Wry/Tao app**

Replace the initial `main` with an event loop, a window titled `Kiniwolf Browser`, and a WebView initialized with the shell HTML. Handle IPC commands by calling WebView navigation methods for navigate/back/forward/reload/home.

- [ ] **Step 6: Verify tests**

Run: `cargo test`

Expected: PASS.

- [ ] **Step 7: Verify build**

Run: `cargo check`

Expected: PASS.

- [ ] **Step 8: Commit**

```bash
git add Cargo.toml src/main.rs src/browser.rs
git commit -m "feat: launch WebView browser window"
```

### Task 5: Polish, Shortcuts, And Manual Run Notes

**Files:**
- Modify: `src/main.rs`
- Modify: `assets/styles.css`
- Modify: `assets/shell.js`
- Create: `README.md`

**Interfaces:**
- Consumes: runnable browser from Task 4.
- Produces: polished first version and usage notes.

- [ ] **Step 1: Add keyboard shortcut behavior**

Implement Enter-to-navigate in the address field, `Alt+Left` for back, `Alt+Right` for forward, `Ctrl+R` for reload, and `Ctrl+L` to focus/select the address field.

- [ ] **Step 2: Polish responsive styling**

Keep toolbar controls stable at small widths, ensure text stays readable, and keep the palette limited to black, gray, and white.

- [ ] **Step 3: Document the app**

Create `README.md` with project purpose, prerequisites including WebView2 runtime, and commands `cargo test`, `cargo check`, and `cargo run`.

- [ ] **Step 4: Verify all tests**

Run: `cargo test`

Expected: PASS.

- [ ] **Step 5: Verify build**

Run: `cargo check`

Expected: PASS.

- [ ] **Step 6: Try to launch**

Run: `cargo run`

Expected: The `Kiniwolf Browser` window opens. If the app cannot launch in the sandbox or WebView2 is unavailable, record the exact limitation in the final handoff.

- [ ] **Step 7: Commit**

```bash
git add README.md assets src
git commit -m "polish: finish first browser experience"
```
