# Rust WebView Browser Design

## Goal

Build a lightweight desktop browser for Windows in Rust, using the system WebView engine for real page rendering. The first version should feel modern and minimal, with a black, gray, and white interface, and should be usable as a simple everyday browser for opening pages, searching, and moving through navigation history.

## Scope

The browser will be a native Rust desktop application. It will use WebView2 through the `wry` ecosystem instead of implementing a browser engine. This keeps the project small, functional, and realistic while still rendering modern websites.

The first version includes:

- A native desktop window.
- A top toolbar with back, forward, reload, home, and go controls.
- A URL/search input.
- Real web navigation in a WebView.
- URL normalization for typed addresses.
- Search fallback for free text using DuckDuckGo.
- A minimal black, gray, and white visual style.
- Basic keyboard shortcuts for common navigation actions.

The first version does not include tabs, bookmarks, downloads UI, extensions, password management, custom engine work, or privacy sandboxing. Those can be added after the core browser is working.

## User Experience

The app opens to a clean start page with a centered address/search field or a default home page. The primary UI is the toolbar at the top and the web content below it. Controls should stay compact, recognizable, and stable across window sizes.

The visual language is monochrome:

- Near-black window background.
- Dark gray toolbar and input surfaces.
- White or light gray text.
- Subtle borders and hover states.
- No decorative gradients or heavy panels.

The user can type a full URL such as `https://example.com`, a domain such as `example.com`, or a search query such as `rust webview browser`. Domains open as HTTPS URLs. Search text opens a DuckDuckGo query.

## Architecture

The project will be a Cargo binary application.

Core parts:

- `main.rs`: application entry point, window setup, event loop, WebView construction, and command routing.
- `browser.rs`: browser state and navigation helpers, including URL normalization.
- `assets/`: local HTML, CSS, and JavaScript for the toolbar/start UI if the chosen implementation embeds a control shell.

The recommended implementation uses `wry` with `tao` for the window and event loop. If the platform integration works better with Wry's current APIs, the project may use Wry's re-exported application types where appropriate.

## Navigation Model

The WebView is the source of truth for page loading. Toolbar actions call WebView navigation APIs where available:

- Back calls the WebView back action.
- Forward calls the WebView forward action.
- Reload calls the WebView reload action.
- Home navigates to the configured home page.
- Go navigates to the normalized input value.

The input should update when navigation changes if Wry exposes the needed navigation event cleanly on Windows. If not, the first version may keep the typed URL visible after user-initiated navigation and leave automatic omnibox synchronization for a follow-up.

## Error Handling

Invalid or empty input should not crash the app. Empty input keeps focus in the address field. Text that cannot be parsed as a URL becomes a search query.

Startup failures should produce a clear error in the terminal during development. Common causes include missing WebView2 runtime or unsupported platform APIs.

## Testing And Verification

Automated tests will cover URL normalization because it is pure logic and easy to regress.

Manual verification will cover:

- `cargo check` succeeds.
- The app launches on Windows when WebView2 is available.
- A URL loads.
- A search query loads.
- Back, forward, reload, and home controls work.
- The interface stays readable in the black, gray, and white theme.

## Future Enhancements

Possible follow-up work:

- Tabs.
- Bookmarks.
- Download handling.
- History page.
- Find in page.
- Per-site zoom.
- Configurable home/search engine.
- Better navigation state synchronization.
