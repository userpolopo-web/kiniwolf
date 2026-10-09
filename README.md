# Kiniwolf Browser

Kiniwolf Browser is a lightweight desktop browser built in Rust with Wry and WebView2. It keeps the interface minimal with a black, gray, and white theme while supporting address entry, search, back, forward, reload, and home navigation.

## Prerequisites

- Rust stable toolchain.
- Microsoft Edge WebView2 Runtime on Windows.
- Linux: GTK 3 and WebKitGTK 4.1 (Ubuntu 22.04+); building requires their development packages.
- macOS 11+: WKWebView is provided by macOS.

## Installers

Download installers from https://github.com/userpolopo-web/kiniwolf/releases:

- Windows x64: run `Kiniwolf-VERSION-windows-x64-setup.exe`. Installs per user, adds a Start menu entry, offers a desktop shortcut and includes an uninstaller. If WebView2 is missing, setup installs it using Microsoft's signed bootstrapper; this step needs internet access.
- macOS: open the `arm64` DMG for Apple Silicon or `x86_64` DMG for Intel, then drag Kiniwolf Browser to Applications. Bundles are ad-hoc signed, without an Apple Developer ID or notarization.
- Ubuntu/Debian amd64 with WebKitGTK 4.1: install the DEB using `sudo apt install ./Kiniwolf-VERSION-linux-amd64.deb`. This resolves runtime dependencies and adds a menu entry. Remove with `sudo apt remove kiniwolf-browser`.

Windows installers are currently unsigned by a publisher certificate. System trust prompts may appear. Uninstalling preserves local profiles and settings. Native password saving is available on Windows only; Linux and macOS use their platform web engines without Kiniwolf password-manager integration.

Profile locations: Windows `%LOCALAPPDATA%\Kiniwolf`; macOS `~/Library/Application Support/Kiniwolf`; Linux `$XDG_DATA_HOME/Kiniwolf` or `~/.local/share/Kiniwolf`. WKWebView manages website storage through macOS rather than the Windows-style profile directory.

`.github/workflows/installers.yml` builds and tests on all four runners when pushing to `master`. Artifacts are available under GitHub Actions. Pushing a version tag matching Cargo.toml (for example `v0.1.0`) publishes all installers and SHA-256 checksums to GitHub Releases after every platform succeeds.

Local packaging after `cargo build --release --locked`:

```sh
bash packaging/linux/package.sh 0.1.0
bash packaging/macos/package.sh 0.1.0
```

For Windows, install Inno Setup 6, place Microsoft's WebView2 bootstrapper at `dist/MicrosoftEdgeWebview2Setup.exe`, then compile `packaging/windows/setup.iss` with `ISCC /DAppVersion=0.1.0`. The GitHub workflow performs these steps automatically.

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
