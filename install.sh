#!/bin/bash

set -e

echo "Building Calm Browser..."
cargo build --release

echo ""
echo "Creating macOS application bundle..."

rm -rf macos-bundle/Calm.app/Contents/MacOS
mkdir -p macos-bundle/Calm.app/Contents/MacOS
cp target/release/calm macos-bundle/Calm.app/Contents/MacOS/calm
chmod +x macos-bundle/Calm.app/Contents/MacOS/calm

echo "Installing Calm.app to /Applications..."
rm -rf /Applications/Calm.app
cp -R macos-bundle/Calm.app /Applications/

echo "Registering app with Launch Services..."
/System/Library/Frameworks/CoreServices.framework/Frameworks/LaunchServices.framework/Support/lsregister -f /Applications/Calm.app

echo ""
echo "Installation complete!"
echo ""
echo "Calm Browser is now installed in /Applications/"
echo ""
echo "Launch Calm by:"
echo "  - Searching 'Calm' in Spotlight (Cmd+Space)"
echo "  - Using Raycast or Alfred"
echo "  - Opening from /Applications/Calm.app"
echo ""
echo "Set as Default Browser:"
echo "  1. Open System Settings > Desktop & Dock"
echo "  2. Scroll down to 'Default web browser'"
echo "  3. Select 'Calm' from the dropdown"
echo ""
echo "Or use this command:"
echo "  open -a 'System Settings' x-apple.systempreferences:com.apple.preference.general"
echo ""
echo "Calm Browser features:"
echo "  - 17 configurable privacy protections"
echo "  - 30+ tracking domains blocked"
echo "  - Canvas, WebGL, and Audio fingerprinting protection"
echo "  - WebRTC leak prevention"
echo "  - Zero data collection"
echo "  - Fully configurable via ~/.calm.yml"
echo ""
