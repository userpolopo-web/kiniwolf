# Kiniwolf Browser

Kiniwolf Browser is a lightweight desktop browser built in Rust with Wry and WebView2. It keeps the interface minimal with a black, gray, and white theme while supporting address entry, search, back, forward, reload, and home navigation.

## Prerequisites

- Rust stable toolchain.
- Microsoft Edge WebView2 Runtime on Windows.

## Commands

```powershell
cargo test
cargo check
cargo run
```

## Shortcuts

- `Enter` in the address field: navigate.
- `Alt+Left`: back.
- `Alt+Right`: forward.
- `Ctrl+R`: reload.
- `Ctrl+L`: focus and select the address field.

## Notes

This first version uses one small WebView for the toolbar and a second WebView for page content, so pages load in the browser engine itself while the controls remain visible.
