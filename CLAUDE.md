# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Calm is a privacy-focused, minimalist web browser built with Rust. It implements 18 layers of fingerprinting protection and blocks 30+ tracking domains by default. The browser emphasizes privacy-by-default with zero telemetry and zero data collection.

## Technology Stack

- **Language**: Rust (1.70+)
- **WebView**: WRY (local fork with Calm-specific modifications) - See `libs/wry/`
- **Window Management**: TAO 0.30
- **Configuration**: YAML (serde_yaml)
- **Platform Support**: macOS (primary), Linux, Windows

**Important**: This project uses a **local fork of WRY** located in `libs/wry/`. This fork contains Calm-specific modifications and should not be replaced with the upstream version without careful consideration.

## Build Commands

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run the browser
cargo run --release

# Run with a specific URL
cargo run --release -- https://example.com

# Format code
cargo fmt

# Lint code
cargo clippy

# Run tests
cargo test
```

## Installation

**IMPORTANT**: After making changes, always install both the desktop app AND the command-line binary:

```bash
# Step 1: Build release binary
cargo build --release

# Step 2: Install macOS application (creates Calm.app in /Applications)
./install.sh

# Step 3: Install command-line binary to ~/.local/bin
cp target/release/calm ~/.local/bin/
chmod +x ~/.local/bin/calm
```

These three steps should be run after every change to ensure both the desktop app and CLI are up to date.

## Architecture

### Core Modules

- **`src/main.rs`**: Application entry point and event loop
- **`src/config.rs`**: Configuration management (`~/.calm.yml`)
- **`src/tabs/`**: Tab management system
  - `manager.rs`: TabManager - central tab lifecycle management
  - `tab.rs`: Individual tab struct with cleanup logic
  - `split_view.rs`: Split view functionality
- **`src/privacy/`**: Privacy protection layer
  - `mod.rs`: Privacy script aggregation
  - `scripts/`: JavaScript injection for fingerprinting protection
    - `core.rs`: Core privacy script
    - `fingerprint.rs`: Canvas/WebGL/Audio protection
    - `tracking.rs`: Tracking domain blocker
    - `title_tracker.rs`: Page title and URL tracking
    - `context_menu.rs`: Custom context menu with privacy features
- **`src/ui/`**: User interface components
  - `template.rs`: Main HTML template
  - `tab_bar/`: Sidebar with tab management
  - `download_overlay/`: Download manager sidebar
  - `animations/`: Loading and transition animations
  - `settings.rs`: Settings page
- **`src/window/`**: Multi-window management
  - `browser_window.rs`: Browser window wrapper
  - `builder.rs`: Window creation with IPC handlers
  - `manager.rs`: Multi-window lifecycle management
- **`src/url_cleaner.rs`**: URL tracking parameter removal and YouTube redirects
- **`src/vimium_hints.rs`**: Vim-style keyboard navigation

### Key Patterns

1. **IPC Communication**: Tab bar and tabs communicate via JSON messages through WRY's IPC system
   - IPC handlers are defined in `window/builder.rs` for tab bar actions
   - Individual tab IPC handlers are in `tabs/manager.rs`
   - Actions: `new_tab`, `close_tab`, `switch_tab`, `navigate_back`, `navigate_forward`, `inspect_element`, `update_title`, `update_url`, etc.

2. **Privacy Scripts**: JavaScript code is injected before page load via `WebViewBuilder::with_initialization_script()`
   - Scripts are conditionally enabled based on `PrivacySettings` in `~/.calm.yml`
   - All scripts use IIFE pattern to avoid polluting global scope

3. **State Management**: Uses `Rc<RefCell<T>>` for shared mutable state
   - `TabManager`, `Config`, sidebar visibility, etc.
   - Avoids complex async/await patterns for simplicity

4. **Custom Protocol**: `calmfile://localhost` for local file access
   - Converts `file://` URLs internally
   - Proper MIME type detection using `infer` crate

## Configuration

User configuration is stored in `~/.calm.yml` with the following structure:

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

ui:
  vim_mode: true
  debug: false

redirects:
  youtube_to_invidious: true
  invidious_instance: https://yewtu.be
```

## Adding New Features

### Adding a New Privacy Feature

1. Add configuration field to `PrivacySettings` in `src/config.rs`
2. Create JavaScript protection script in `src/privacy/scripts/`
3. Add conditional inclusion in `get_all_privacy_scripts_with_config()` in `src/privacy/scripts/mod.rs`
4. Update default configuration in `src/config.rs`
5. Document in `PRIVACY.md`

### Adding a New IPC Action

1. Add handler in `src/window/builder.rs` (for tab bar messages) or `src/tabs/manager.rs` (for individual tab messages)
2. Add JavaScript sender in appropriate UI component (e.g., `src/ui/tab_bar/script.rs`)
3. Test with debug mode enabled (`ui.debug: true` in `~/.calm.yml`)

### Adding UI Components

1. UI components are HTML/CSS/JavaScript strings in Rust
2. Use the Gohu font for consistency: `font-family: 'gohu', monospace;`
3. Font face is embedded via `fonts::get_gohu_font_face()`
4. Follow minimalist black/white design aesthetic
5. All UI uses `-webkit-app-region: drag` for macOS window dragging

## Debug Mode

Enable debug logging by setting `ui.debug: true` in `~/.calm.yml`. Debug messages use the `debug_log!` macro and appear in stderr.

## Common Patterns

### Creating a New Tab

```rust
let tab_id = tab_manager.borrow_mut().create_tab(
    &window,
    "https://example.com",
    None, // Optional HTML content
)?;
```

### Evaluating JavaScript in Active Tab

```rust
if let Some(webview) = tab_manager.borrow().get_active_tab_webview() {
    let _ = webview.evaluate_script("console.log('Hello');");
}
```

### Adding Context Menu Item

Modify `src/privacy/scripts/context_menu.rs`:
```javascript
items.push(createMenuItem('My Action', () => {
    // Action code
    hideContextMenu();
}));
```

## Testing

- Manual testing is primary approach
- Test privacy features at: https://coveryourtracks.eff.org/
- Test download functionality with various file types
- Test multi-tab behavior with media-playing sites
- Test vim navigation (j/k/gg/G/d/n//)

## Known Limitations

- WebRTC-dependent sites won't work (intentional - prevents IP leaks)
- Some canvas/WebGL-heavy sites may have reduced performance due to noise injection
- No browser extension support (by design)
- Developer tools require `devtools` feature flag (enabled in `Cargo.toml`)

## Keyboard Shortcuts

- **Cmd+T**: New tab
- **Cmd+W**: Close tab/window
- **Cmd+Q**: Quit
- **Cmd+R**: Reload
- **Cmd+L**: Focus URL bar
- **Cmd+J**: Toggle downloads
- **Cmd+N**: New window
- **Cmd+E**: Focus sidebar
- **Cmd+Shift+S**: Toggle split view

Vim mode (enabled by default):
- **j/k**: Navigate tabs
- **gg**: Jump to first tab
- **G**: Jump to last tab
- **d**: Close focused tab
- **n**: New tab
- **/**: Focus URL bar
- **Enter**: Open focused tab

## File Structure Notes

- JavaScript files are embedded as Rust string constants (not separate .js files)
- CSS is similarly embedded in Rust
- This allows single-binary distribution
- HTML templates use format strings for dynamic content

## Platform-Specific Code

```rust
#[cfg(target_os = "macos")]
{
    // macOS-specific code (transparent titlebar, traffic lights, etc.)
}
```

- macOS: WKWebView engine, native menu bar, traffic light positioning
- Linux: WebKitGTK engine
- Windows: WebView2 (Edge) engine

## Dependencies

Key dependencies and their purposes:
- `wry`: WebView library (local fork)
- `tao`: Window management
- `muda`: Native menu integration
- `serde`/`serde_json`/`serde_yaml`: Configuration and IPC
- `url`: URL parsing and cleaning
- `urlencoding`: URL encoding/decoding
- `global-hotkey`: System-wide keyboard shortcuts
- `infer`: MIME type detection for downloads
- `base64`: Font embedding
- `reqwest`: HTTP requests (for downloads)
- `content_disposition`: HTTP header parsing
- `dirs`: Cross-platform directory paths

## Documentation Files

- `README.md`: User-facing documentation
- `PRIVACY.md`: Detailed privacy feature documentation
- `ARCHITECTURE.md`: Window and tab management architecture (may be outdated)
- `KNOWN_ISSUES.md`: Known limitations and issues
- `DOWNLOAD_TRACKING.md`: Download feature documentation
- `CHANGELOG.md`: Version history
- `CLAUDE.md`: This file

## Development Workflow

1. Make changes to source code
2. Run `cargo fmt` to format
3. Run `cargo clippy` to check for issues
4. Build with `cargo build --release`
5. Install both versions:
   - Run `./install.sh` to install macOS app to /Applications
   - Run `cp target/release/calm ~/.local/bin/ && chmod +x ~/.local/bin/calm` for CLI
6. Test manually in the browser

## Important Notes

- Always use `debug_log!` macro for debug output, not `println!` or `eprintln!`
- Preserve the minimalist aesthetic - black/white color scheme, pixel-perfect borders
- All privacy features should be opt-out, not opt-in
- URL bar should always display cleaned URLs (tracking parameters removed)
- Downloads should auto-detect file extensions using `infer` crate
- Tab cleanup is critical - always stop audio/video before closing tabs
- IPC messages must be valid JSON strings
- Settings changes require browser restart to take effect
