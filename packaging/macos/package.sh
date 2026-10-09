#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
version="${1:?Usage: package.sh VERSION}"
arch="$(uname -m)"
stage="$(mktemp -d)"
trap 'rm -rf "$stage"' EXIT
app="$stage/Kiniwolf Browser.app"
mkdir -p "$app/Contents/MacOS" dist
install -m 755 target/release/kiniwolf-browser "$app/Contents/MacOS/kiniwolf-browser"
cat > "$app/Contents/Info.plist" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
<key>CFBundleName</key><string>Kiniwolf Browser</string>
<key>CFBundleDisplayName</key><string>Kiniwolf Browser</string>
<key>CFBundleIdentifier</key><string>io.github.userpolopo-web.kiniwolf</string>
<key>CFBundleExecutable</key><string>kiniwolf-browser</string>
<key>CFBundlePackageType</key><string>APPL</string>
<key>CFBundleShortVersionString</key><string>$version</string>
<key>CFBundleVersion</key><string>$version</string>
<key>LSMinimumSystemVersion</key><string>11.0</string>
<key>NSHighResolutionCapable</key><true/>
<key>NSAppTransportSecurity</key><dict><key>NSAllowsArbitraryLoadsInWebContent</key><true/></dict>
</dict></plist>
EOF
plutil -lint "$app/Contents/Info.plist"
codesign --force --deep --sign - "$app"
codesign --verify --deep --strict "$app"
ln -s /Applications "$stage/Applications"
hdiutil create -volname "Kiniwolf Browser" -srcfolder "$stage" -ov -format UDZO "dist/Kiniwolf-$version-macos-$arch.dmg"
