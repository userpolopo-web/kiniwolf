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
- `Ctrl+T`: open a tab.
- `Ctrl+W`: close the current tab.

## Notes

The toolbar and each loaded tab use native WebViews. Use the gear button to change settings.

Memory saver is optional and off by default. When enabled, switching tabs destroys the inactive page's WebView. Only its title and URL remain in memory; selecting it recreates and reloads the page. This reduces memory, but does not promise zero RAM. Page history, scroll, unsaved forms, downloads, audio and ongoing page work may be lost or interrupted. With memory saver off, visited tabs stay loaded in the background. Disabling memory saver does not immediately reload already suspended tabs.

On Windows, password saving and account autofill use WebView2's native password manager and consent prompts. This setting also controls general form autofill. Saved passwords are managed by WebView2, never copied into application JSON or JavaScript. Turning it off does not delete existing credentials; changes take effect on subsequent navigation. Cookies and website sessions share a persistent local browser profile, so login persistence still depends on the website. This is local storage, not account synchronization or separate user profiles.

Settings are stored in `%LOCALAPPDATA%\Kiniwolf\settings.json`; cookies and credentials live in `%LOCALAPPDATA%\Kiniwolf\WebView2`. Sites may disable or be incompatible with native password detection.

## Sign-in popups

Popups use WebView2's native popup windows so `window.opener`, `postMessage`, and delayed navigation from `about:blank` remain intact. They are not converted into independent tabs. Tabs that open popups remain loaded until closed, even with memory saver enabled, to avoid interrupting authentication. This conservative exception also applies after closing the popup.

Google may reject OAuth authentication inside embedded browsers independently of popup handling. Such restrictions require signing in and using that site in a supported external browser; an external browser's session is not automatically transferred into Kiniwolf. See https://developers.google.com/identity/protocols/oauth2/policies.

Manual regression: serve `tests/fixtures` over HTTP (for example, `python -m http.server 18749 --directory tests/fixtures`) and open `/popup.html` in Kiniwolf. Click `Open test popup`. The parent must show `PASS: opener communication preserved`; the previous implementation showed `FAIL: popup was canceled`. Repeat with memory saver enabled, switch tabs while the popup is open, and verify the opener remains loaded.
