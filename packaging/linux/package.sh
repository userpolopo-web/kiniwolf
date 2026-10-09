#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
version="${1:?Usage: package.sh VERSION}"
arch="$(dpkg --print-architecture)"
stage="$(mktemp -d)"
trap 'rm -rf "$stage"' EXIT
mkdir -p "$stage/DEBIAN" "$stage/usr/bin" "$stage/usr/share/applications" dist
install -m 755 target/release/kiniwolf-browser "$stage/usr/bin/kiniwolf-browser"
cat > "$stage/DEBIAN/control" <<EOF
Package: kiniwolf-browser
Version: $version
Section: web
Priority: optional
Architecture: $arch
Maintainer: Kiniwolf <noreply@github.com>
Depends: libgtk-3-0, libwebkit2gtk-4.1-0 (>= 2.38), libsoup-3.0-0, libc6 (>= 2.35), libgcc-s1
Homepage: https://github.com/userpolopo-web/kiniwolf
Description: Minimal desktop web browser
 Native web pages, tabs and optional memory saving.
EOF
cat > "$stage/usr/share/applications/io.github.userpolopo-web.kiniwolf.desktop" <<'EOF'
[Desktop Entry]
Type=Application
Name=Kiniwolf Browser
Comment=Browse the web
Exec=kiniwolf-browser
Terminal=false
Categories=Network;WebBrowser;
StartupWMClass=kiniwolf-browser
EOF
desktop-file-validate "$stage/usr/share/applications/io.github.userpolopo-web.kiniwolf.desktop"
dpkg-deb --root-owner-group --build "$stage" "dist/Kiniwolf-$version-linux-$arch.deb"
dpkg-deb --info "dist/Kiniwolf-$version-linux-$arch.deb"
