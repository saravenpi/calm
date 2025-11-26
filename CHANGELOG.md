# Changelog

All notable changes to Calm Browser will be documented in this file.

## [Unreleased] - 2025-11-26

### Added
- **Dynamic tab titles** that automatically update from page `<title>` tags
- **Real-time title tracking** using MutationObserver for live updates
- **Navigation state tracking** for back/forward button states
- **Comprehensive URL tracking parameter removal** (60+ tracking params)
- **Download failure notifications** with visual red indicators
- **Unique download IDs** preventing collisions on multiple downloads per tab
- **URL cleaner module** (`src/url_cleaner.rs`) with full test coverage
- **Title tracker script** (`src/privacy/scripts/title_tracker.rs`)

### Changed
- Tab titles now show actual page titles instead of just hostnames
- Download filenames are sanitized (query params removed, URL decoded)
- Navigation buttons (back/forward) now correctly enable/disable based on history
- URLs automatically cleaned on tab creation and navigation
- Downloads now get unique sequential IDs independent of tab IDs

### Fixed
- **Download ID collision bug** - multiple downloads in same tab now tracked correctly
- **Navigation history** - back/forward buttons now work properly with link clicks
- **Filename sanitization** - removes tracking params and URL-encodes filenames
- **Failed downloads** - now visible to users with error indication

## [0.2.0] - 2025-11-25

### Added
- **Multi-tab support** with visual tab bar at the top
- **Keyboard shortcuts** for tab management:
  - `Cmd+T` / `Ctrl+T`: Open new tab
  - `Cmd+W` / `Ctrl+W`: Close current tab
  - `Cmd+1-9`: Switch to specific tab
- **Semi-transparent address input overlay** with:
  - Rounded corners and modern design
  - Backdrop blur effect
  - Dark theme matching browser aesthetic
  - Smart URL detection (auto-adds https://)
  - DuckDuckGo search integration for non-URL queries
- **Tab bar UI** showing:
  - All open tabs with titles
  - Close button for each tab
  - Plus button to create new tabs
  - Active tab highlighting
- New UI module with HTML template system

### Changed
- Browser now loads with custom HTML UI instead of direct URL
- Updated README with tab features and keyboard shortcuts
- Updated architecture documentation

### Known Issues
- Tabs use iframes, which may be blocked by some sites with X-Frame-Options headers
- Privacy scripts cannot be injected into cross-origin iframe content
- Future versions will use native WebView instances per tab

## [0.1.0] - 2025-11-25

### Added
- Initial release of Calm Browser
- Privacy-focused Rust-based web browser
- Tracking protection (20+ domains blocked)
- Canvas fingerprinting protection
- WebRTC leak prevention
- DNT (Do Not Track) headers on all requests
- Generic user agent to reduce fingerprinting
- JavaScript injection for privacy protections
- Built with WRY (cross-platform WebView library)
- Built with TAO (window management)
- Comprehensive privacy documentation
- Installation script

### Privacy Features
- Blocks Google Analytics, Facebook Pixel, Twitter Analytics
- Blocks 20+ major tracking domains
- Adds noise to canvas operations
- Disables WebRTC connections
- Spoofs navigator properties
- Blocks dynamic tracking script injection
