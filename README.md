# Calm Browser

<p align="center">
  <img src="calm_logo.png" alt="Calm Browser Logo" width="200">
</p>

<p align="center">
  <strong>A minimalist, privacy-focused web browser built with Rust</strong>
</p>

<p align="center">
  Implementing comprehensive fingerprinting protection inspired by Tor Browser and Brave
</p>

<p align="center">
  <img src="https://img.shields.io/badge/rust-1.70+-orange.svg" alt="Rust Version">
  <img src="https://img.shields.io/badge/platform-macOS%20%7C%20Linux%20%7C%20Windows-blue.svg" alt="Platform">
  <img src="https://img.shields.io/badge/license-MIT-green.svg" alt="License">
</p>

---

## Quick Start

```bash
# macOS Installation
./install.sh

# Or build manually
cargo build --release
./target/release/calm

# Open a URL
calm https://example.com

# Search the web
calm privacy focused browser
```

## Features

- 18 configurable privacy protection layers
- 30+ tracking domains blocked (Google Analytics, Facebook Pixel, Twitter/TikTok Analytics, session replay tools)
- Zero data collection or telemetry
- Native Rust performance with low resource usage
- Privacy protections enabled by default
- Multi-tab browsing with visual tab bar
- Download manager sidebar
- Fully configurable via ~/.calm.yml
- Custom protocol support for local files (calmfile://)

## Privacy Features

### Fingerprinting Defenses

1. Hardware spoofing - Normalizes CPU cores, RAM, platform, touch points
2. Screen normalization - Standardizes resolution to 1920x1080
3. Timezone normalization - Reports UTC
4. Battery API blocking - Prevents battery-based tracking
5. WebRTC blocking - Prevents IP leaks
6. Media device blocking - Prevents camera/microphone enumeration
7. Geolocation blocking - Disables location APIs
8. Network information spoofing - Standardizes connection type
9. Storage quota spoofing - Normalizes storage metrics
10. Permissions API hardening - Denies sensitive permissions
11. Credentials API blocking - Prevents credential enumeration
12. Privacy headers - Automatic DNT, Sec-GPC, Client Hints on all requests
13. Tracking domain blocking - 30+ domains blocked
14. User agent normalization - Common Chrome user agent
15. Canvas noise injection - Adds pixel noise to prevent fingerprinting
16. WebGL spoofing - GPU information normalized with noise
17. Audio randomization - Audio data noise injection
18. Font enumeration restriction - Limited to 9 standard fonts

### Tracking Protection

Blocks 30+ tracking domains including:
- Google (Analytics, Tag Manager, DoubleClick, Ads)
- Facebook (Pixel, Connect)
- Twitter/X (Analytics, Ads)
- TikTok (Pixel, Analytics)
- Session replay (Hotjar, Mouseflow, Crazy Egg, Lucky Orange, ClickTale, Inspectlet, FullStory)
- Analytics (Mixpanel, Segment, Heap, Amplitude, Chartbeat, New Relic, Quantserve, Optimizely)

See [PRIVACY.md](PRIVACY.md) for details.

### Multi-Tab Browsing

- Visual tab bar at the top with tab titles and close buttons
- Active tab highlighting
- Audio indicator for tabs playing media
- Smooth tab animations (opening/closing)
- Keyboard shortcuts for navigation
- URL bar with auto-focus on new tabs
- Back/forward navigation buttons
- No tab limit - browse freely

### Download Management

- Toggle sidebar with Cmd+J
- Shows all downloads with progress
- Download status tracking
- Clean, organized interface
- Smooth slide-in/slide-out animations

### Design

- Lightweight Rust implementation
- Clean, minimal interface
- Low resource usage
- Native performance
- Cross-platform support (macOS, Linux, Windows)

## Installation

### Prerequisites

- Rust 1.70+ ([rustup.rs](https://rustup.rs))
- macOS: Xcode Command Line Tools
- Linux: WebKitGTK development files

### macOS Application Install (Recommended)

```bash
./install.sh
```

This will:
- Build the release binary
- Create a proper macOS application bundle
- Install Calm.app to /Applications
- Register with Spotlight/Raycast

After installation, you can launch Calm by:
- Searching "Calm" in Spotlight (Cmd+Space)
- Using Raycast or Alfred
- Opening from /Applications folder
- Clicking on HTTP/HTTPS links (when set as default browser)

### Set as Default Browser

To set Calm as your default web browser:

1. Open **System Settings** > **Desktop & Dock**
2. Scroll down to **Default web browser**
3. Select **Calm** from the dropdown

Or run: `open -a 'System Settings' x-apple.systempreferences:com.apple.preference.general`

Calm will now open automatically when you click web links.

### Manual Build

```bash
cargo build --release
```

Binary location: `target/release/calm`

### Command-line Install (Alternative)

```bash
cargo build --release
cp target/release/calm ~/.local/bin/
```

Ensure `~/.local/bin` is in your PATH.

## Configuration

Calm can be configured via `~/.calm.yml`. The file is automatically created on first run with default values.

### Configuration Options

```yaml
search_engine: https://start.duckduckgo.com/?q={}
default_url: https://start.duckduckgo.com

privacy:
  hardware_spoofing: true
  screen_normalization: true
  timezone_normalization: true
  battery_blocking: true
  webrtc_blocking: true
  media_device_blocking: true
  geolocation_blocking: true
  network_info_spoofing: true
  storage_quota_spoofing: true
  permissions_hardening: true
  credentials_blocking: true
  privacy_headers: true
  tracking_domain_blocking: true
  canvas_fingerprint_protection: true
  webgl_fingerprint_protection: true
  audio_fingerprint_protection: true
  font_enumeration_restriction: true
```

**General Options:**
- `search_engine`: Search engine URL pattern (use `{}` as placeholder for search query)
- `default_url`: URL or file path to open when Calm starts without arguments

**Privacy Options (all enabled by default):**
- `hardware_spoofing`: Spoofs CPU cores, RAM, platform details
- `screen_normalization`: Standardizes screen resolution to 1920x1080
- `timezone_normalization`: Reports UTC timezone
- `battery_blocking`: Prevents battery-based fingerprinting
- `webrtc_blocking`: Blocks WebRTC to prevent IP leaks
- `media_device_blocking`: Blocks camera/microphone enumeration
- `geolocation_blocking`: Disables location APIs
- `network_info_spoofing`: Spoofs connection type
- `storage_quota_spoofing`: Normalizes storage metrics
- `permissions_hardening`: Denies sensitive permissions
- `credentials_blocking`: Blocks credential enumeration
- `privacy_headers`: Adds DNT and Sec-GPC headers
- `tracking_domain_blocking`: Blocks 30+ tracking domains
- `canvas_fingerprint_protection`: Canvas noise injection
- `webgl_fingerprint_protection`: WebGL information spoofing
- `audio_fingerprint_protection`: Audio data randomization
- `font_enumeration_restriction`: Limits to 9 standard fonts

**Examples:**

```yaml
# Open GitHub on startup
default_url: https://github.com

# Open a local HTML file
default_url: file:///Users/username/Documents/homepage.html

# Use Google as search engine
search_engine: https://www.google.com/search?q={}

# Disable specific privacy features (not recommended)
privacy:
  canvas_fingerprint_protection: false
  webgl_fingerprint_protection: false
```

See `.calm.yml.example` for a fully documented configuration file.

## Usage

```bash
calm                      # Opens default_url from config
calm https://example.com  # Opens specific URL
calm github.com          # Auto-adds https://
calm rust programming    # Searches using configured search engine
```

### Keyboard Shortcuts

All shortcuts work reliably with dual-layer event handling (native + JavaScript):

- **Cmd+T** (Ctrl+T): Open new tab
- **Cmd+W** (Ctrl+W): Close current tab (or quit if last tab)
- **Cmd+Q** (Ctrl+Q): Quit application immediately
- **Cmd+R** (Ctrl+R): Reload current tab
- **Cmd+L** (Ctrl+L): Focus and select URL bar
- **Cmd+J** (Ctrl+J): Toggle downloads sidebar
- **Enter**: Navigate to URL (when in URL bar)
- **Esc**: Blur URL bar (exit focus)

Shortcuts use physical key codes for consistent cross-platform behavior.

### Tab Management

- **Cmd+T** opens a new tab and focuses the URL bar
- Auto-detects URLs (adds https:// if needed)
- Searches using your configured search engine
- Press Enter to navigate, Esc to blur the URL bar

Tab bar shows all open tabs with close buttons and active tab highlighting.

## Technical Details

### JavaScript Injection

Privacy scripts are injected before page load using WRY's initialization scripts:

1. **WebRTC Blocking**: Prevents IP leaks by blocking RTCPeerConnection
2. **Tracking Domain Blocking**: Intercepts XMLHttpRequest and Fetch API calls
3. **Element Blocking**: Intercepts appendChild/insertBefore for scripts and iframes
4. **Fingerprinting API Spoofing**: Overrides navigator, screen, canvas, WebGL, audio, fonts
5. **Browser Normalization**: Standardizes all identifiable characteristics

### User Agent

Generic Chrome user agent for maximum compatibility:
```
Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36
```

### Custom Protocol Handler

Calm uses a custom `calmfile://` protocol for local file access:
- Converts `file://` URLs to `calmfile://localhost` internally
- Provides secure local file rendering
- Proper MIME type detection
- Cross-platform path resolution

### Keyboard Event Handling

Dual-layer event system for maximum reliability:
1. **Native Layer** (TAO/Rust): Captures events at window level using physical key codes
2. **JavaScript Layer** (WebView): Captures events in capture phase with preventDefault
3. **Fallback Safety**: If one layer fails, the other ensures shortcuts work

This approach solves the common WRY/WKWebView issue where focused webviews steal keyboard events.

## Technology Stack

- **Language**: Rust
- **WebView**: WRY (cross-platform WebView library)
- **Window Management**: TAO
- **Rendering Engine**:
  - macOS: WKWebView (Safari engine)
  - Linux: WebKitGTK
  - Windows: WebView2 (Edge engine)

## Limitations

- Some websites may have compatibility issues due to aggressive privacy protections
- WebRTC-dependent sites (video conferencing) will not work due to blocking
- Canvas/WebGL-heavy sites may experience reduced performance due to noise injection
- No browser extension support (by design)
- No bookmarks or history persistence (privacy-focused)
- Sites that detect fingerprinting protection may show warnings or block access

## Future Enhancements

- Private bookmarks (encrypted, local-only)
- Session management with privacy controls
- EasyList/EasyPrivacy filter list integration
- HTTPS-only mode with automatic upgrading
- Per-site privacy settings override
- Custom CSS injection for site theming
- Import/export settings
- Tab groups and organization
- Find in page functionality

## Contributing

Contributions are welcome! Please follow these guidelines:

1. Fork the repository
2. Create a feature branch
3. Make your changes with clear commit messages
4. Test thoroughly on your platform
5. Submit a pull request with detailed description

### Development Setup

```bash
# Clone the repository
git clone https://github.com/saravenpi/calm.git
cd calm

# Build and run
cargo build --release
./target/release/calm

# Run tests
cargo test

# Install locally (macOS)
./install.sh
```

### Code Style

- Follow Rust standard formatting (use `cargo fmt`)
- Run `cargo clippy` before submitting
- Add tests for new features
- Update documentation for changes

## Performance

Calm is designed for efficiency:

- **Memory**: ~50-100MB base + ~100-200MB per tab
- **CPU**: Minimal usage when idle, efficient rendering
- **Startup**: < 1 second cold start
- **Binary Size**: ~10MB (release build)

Performance may vary based on:
- Number of active tabs
- Complexity of loaded pages
- Privacy protection settings
- System resources

## Security

Calm prioritizes security and privacy:

- No analytics or telemetry
- No automatic updates (manual control)
- No cloud sync or account system
- All data stays local
- No third-party dependencies for tracking
- Source code available for auditing

## License

MIT License - See LICENSE file for details

## FAQ

### Why another browser?

Calm focuses on simplicity and privacy by default. Unlike major browsers, Calm has:
- No telemetry or data collection
- All privacy features enabled by default
- Minimal UI with keyboard-first navigation
- Lightweight Rust implementation
- Complete transparency (open source)

### How is this different from Brave or Firefox?

- **Simpler**: No built-in crypto, rewards, or complex features
- **Smaller**: ~10MB binary vs 200MB+ for major browsers
- **Faster startup**: < 1 second cold start
- **More private**: 18 layers of protection with no compromises
- **Keyboard-centric**: Designed for power users

### Does Calm block ads?

Calm blocks 30+ tracking domains (analytics, pixels, session replay). For comprehensive ad blocking, use a system-wide blocker or DNS filtering.

### Can I use Calm as my daily driver?

Yes, for privacy-focused browsing. However:
- Video conferencing won't work (WebRTC blocked)
- Some sites may detect fingerprint protection
- No extension support for site-specific fixes

### Will my keyboard shortcuts always work?

Yes! Calm uses a dual-layer event system (native + JavaScript) to ensure shortcuts work even when webviews steal focus - a common issue in webview-based browsers.

### How do I report issues?

Open an issue on GitHub with:
- OS version and platform
- Steps to reproduce
- Expected vs actual behavior
- Console output if available

### Can I contribute?

Absolutely! See the Contributing section above. All contributions are welcome, from bug reports to feature implementations.

## Disclaimer

Calm provides strong privacy protections but cannot guarantee complete anonymity. For maximum privacy:
- Use with VPN or Tor
- Use encrypted DNS (Cloudflare 1.1.1.1, Quad9)
- Avoid logging into accounts that identify you
- Clear browsing data regularly
- Consider using Tails OS for maximum anonymity

## Author

**saravenpi**

## Acknowledgments

- Built with [WRY](https://github.com/tauri-apps/wry) - Cross-platform WebView library
- Built with [TAO](https://github.com/tauri-apps/tao) - Cross-platform window creation
- Inspired by privacy-focused browsers like Brave, Tor Browser, and LibreWolf
- Privacy techniques adapted from Tor Browser and Brave Research

---

<p align="center">
  Made with privacy in mind by <strong>saravenpi</strong>
</p>
