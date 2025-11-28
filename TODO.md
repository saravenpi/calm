# Calm Browser - Feature Roadmap

This document tracks features and improvements needed to make Calm a fully usable day-to-day browser.

## Core Browser Features

### Tab Management
- [ ] Tab reordering (drag and drop)
- [ ] Tab pinning
- [ ] Tab groups/collections
- [ ] Recently closed tabs restoration
- [ ] Tab session saving/restoration
- [ ] Duplicate tab functionality
- [ ] Tab muting (audio control)
- [ ] Tab discarding for memory management

### Navigation & History
- [ ] Full browsing history with search
- [ ] History sidebar/panel
- [ ] Bookmarks system
  - [ ] Add/remove bookmarks
  - [ ] Bookmark folders/organization
  - [ ] Bookmarks bar
  - [ ] Import/export bookmarks
- [ ] Reading list
- [ ] URL bar autocomplete from history

### User Experience
- [ ] Keyboard shortcuts customization
- [ ] Context menus (right-click)
  - [ ] Page context menu
  - [ ] Link context menu
  - [ ] Image context menu
  - [ ] Selection context menu
- [ ] Find in page (Cmd+F)
- [ ] Zoom controls (Cmd+/-, Cmd+0)
- [ ] Full-screen mode
- [ ] Picture-in-Picture support
- [ ] Print functionality
- [ ] Page info/security indicators
- [ ] Developer tools integration

### Privacy & Security
- [ ] Cookie management
- [ ] Cache management
- [ ] Private/Incognito mode
- [ ] Site permissions (location, camera, microphone, etc.)
- [ ] HTTPS indicators
- [ ] Password manager integration
- [ ] Content blockers/ad blocking
- [ ] Tracking protection
- [ ] Clear browsing data

### Settings & Customization
- [ ] Settings/Preferences UI
- [ ] Default search engine selection
- [ ] Homepage customization
- [ ] New tab page customization
- [ ] Theme support (beyond current black/white)
- [ ] Font size/family customization
- [ ] Language preferences
- [ ] Default download location

### Downloads
- [x] Download tracking (basic implementation exists)
- [ ] Download pause/resume
- [ ] Download history
- [ ] Download location selection per file
- [ ] Open containing folder
- [ ] Scan with antivirus integration

### Multi-Window Support
- [x] Multiple browser windows (Cmd+N creates new window)
- [x] Firefox-like behavior (new tabs open in focused window)
- [x] Per-window tab management
- [x] Window focus tracking
- [x] Close window when last tab closes
- [ ] Move tabs between windows (drag-and-drop - UI implementation pending)
- [x] Window session management (framework implemented)
- [x] Split view/side-by-side tabs (Cmd+Shift+S)
  - [x] Vertical and horizontal split orientations
  - [x] Toggle split view on/off
  - [x] Swap panes functionality
  - [x] IPC commands for split view control
  - [x] UI controls and visual indicators (buttons in tab bar)
  - [ ] Drag to resize split ratio (requires custom draggable divider implementation)
  - [ ] Visual split divider line between panes

### Media & Content
- [ ] Media playback controls
- [ ] Fullscreen video support
- [ ] PDF viewer
- [ ] Image viewer
- [ ] View source functionality

## Performance & Stability

- [ ] Memory optimization
- [ ] Tab lazy loading
- [ ] Crash recovery
- [ ] Update mechanism
- [ ] Performance monitoring
- [ ] Reduce startup time
- [ ] Efficient tab suspension

## Sync & Cloud Features

- [ ] Cross-device sync
  - [ ] Sync bookmarks
  - [ ] Sync history
  - [ ] Sync open tabs
  - [ ] Sync settings
- [ ] Account system

## Developer Features

- [ ] Extensions/Add-ons support
- [ ] User scripts support
- [ ] Web Inspector/DevTools
- [ ] Responsive design mode
- [ ] Console access

## Accessibility

- [ ] Screen reader support
- [ ] High contrast mode
- [ ] Keyboard-only navigation improvements
- [ ] Text-to-speech
- [ ] Customizable UI scaling

## Platform-Specific

### macOS
- [x] Traffic light positioning (implemented)
- [ ] Touch Bar support
- [ ] Native notifications
- [ ] Handoff support
- [ ] Share menu integration

### Cross-Platform
- [ ] Windows support testing
- [ ] Linux support testing
- [ ] Platform-specific installers

## Current Limitations to Address

1. Maximum 20 tabs - consider increasing or making dynamic
2. Fixed download sidebar width - make resizable
3. No tab overflow handling for many tabs
4. No visual feedback for long-running page loads
5. Missing SSL/certificate information display

## Nice-to-Have Features

- [ ] Reader mode
- [ ] Screenshot tool
- [ ] QR code generator for current page
- [ ] Translate page
- [ ] Dark mode for web content
- [ ] Collections/workspaces
- [x] Vertical tabs (Zen Browser style) - COMPLETED
- [ ] Tab stacking
- [ ] Mouse gestures
- [ ] Voice commands

## Implementation Notes

### Drag-to-Resize Split View
To implement draggable split resizing:
1. Create a semi-transparent divider overlay WebView between panes
2. Add mouse event listeners for drag start/move/end
3. Calculate new split ratio based on mouse position
4. Send IPC message to adjust split ratio
5. Update webview bounds in real-time during drag

### Move Tabs Between Windows
To implement drag-and-drop between windows:
1. Add drag event listeners to tabs in tab bar
2. Implement drag preview/ghost element
3. Track mouse position across windows
4. Detect drop target window
5. Transfer tab state and webview to target window
6. Update both tab bars via IPC

### YouTube to Invidious Redirect
- [x] Backend URL redirect logic (url_cleaner.rs)
- [x] Client-side link interception (JavaScript injection)
- [x] Configurable instance and toggle (Config struct)
- [x] Handles all YouTube URL formats (youtu.be, shorts, embed, etc.)

## Known Issues

- Document any bugs or issues here as they're discovered

## Performance Targets

- Startup time: < 1s
- New tab creation: < 100ms
- Tab switch: < 50ms
- Memory per tab: < 100MB average

---

Last updated: 2025-11-27
