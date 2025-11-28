# Calm Browser - Window and Tab Management Architecture

## Executive Summary

Calm is a **single-window, multi-tab** Tauri-based browser built with:
- **Backend:** Rust using WRY (WebView) and TAO (windowing)
- **Frontend:** HTML/CSS/JavaScript for UI
- **IPC Communication:** JSON-based message passing between frontend and backend
- **State Management:** RefCell-based shared state in Rust with global hotkey management

## 1. WINDOW MANAGEMENT

### 1.1 Single Window Architecture

**Location:** `src/main.rs` (lines 42-103)

```rust
// macOS-specific configuration
#[cfg(target_os = "macos")]
let window = Rc::new(
    WindowBuilder::new()
        .with_title("Calm Browser - Privacy-Focused")
        .with_inner_size(tao::dpi::LogicalSize::new(1200, 800))
        .with_title_hidden(true)
        .with_titlebar_transparent(true)
        .with_fullsize_content_view(true)
        .build(&event_loop)
        .unwrap()
);
```

**Key Points:**
- Single window created at startup using TAO's `WindowBuilder`
- Platform-specific configuration (macOS transparent titlebar, fullsize content view)
- Default size: 1200x800 pixels
- All tabs are rendered as child webviews within this single window
- No multi-window support currently exists

### 1.2 Window State Management

**Location:** `src/main.rs` (lines 122-132)

Core shared state uses Rust's `Rc<RefCell<T>>` pattern:

```rust
let config = Rc::new(RefCell::new(config));
let tab_manager = Rc::new(RefCell::new(TabManager::new(TAB_SIDEBAR_WIDTH, config.borrow().clone())));
let tab_bar_webview_ref: Rc<RefCell<Option<Rc<wry::WebView>>>> = Rc::new(RefCell::new(None));
let download_overlay_ref: Rc<RefCell<Option<Rc<wry::WebView>>>> = Rc::new(RefCell::new(None));
let sidebar_visible = Rc::new(RefCell::new(false));
let should_quit = Rc::new(RefCell::new(false));
```

**State Variables:**
- `config` - Application configuration (URLs, search engine, privacy settings)
- `tab_manager` - Central tab management
- `tab_bar_webview_ref` - Reference to sidebar/tab bar webview
- `download_overlay_ref` - Reference to downloads panel
- `sidebar_visible` - Downloads panel visibility state
- `should_quit` - Application termination flag

### 1.3 Window Resizing and Layout

**Location:** `src/main.rs` (lines 610-646)

The window handles resize, scale factor, move, and theme change events:

```rust
Event::WindowEvent {
    event: WindowEvent::Resized(_),
    ..
} | Event::WindowEvent {
    event: WindowEvent::ScaleFactorChanged { .. },
    ..
}
```

Layout Constants:
- `TAB_SIDEBAR_WIDTH: u32 = 250` - Left sidebar for tab management
- `DOWNLOAD_SIDEBAR_WIDTH: i32 = 360` - Right sidebar for downloads

**Tab Content Area Calculation:**
```
content_width = window_width - TAB_SIDEBAR_WIDTH - (download_sidebar_width if visible)
```

## 2. TAB MANAGEMENT

### 2.1 Tab Structure

**Location:** `src/tabs/tab.rs`

```rust
pub struct Tab {
    pub webview: WebView,
    pub url: String,
}
```

**Methods:**
- `new(id, url, webview)` - Create new tab
- `set_url(url)` - Update URL
- `get_url()` - Get current URL
- `show()` - Make tab visible
- `hide()` - Hide tab
- `cleanup()` - Stop media, clear caches, navigate to about:blank

### 2.2 TabManager

**Location:** `src/tabs/manager.rs`

```rust
pub struct TabManager {
    tabs: HashMap<usize, Tab>,           // Map of tab_id -> Tab
    active_tab_id: Option<usize>,        // Currently displayed tab
    next_tab_id: usize,                  // Auto-incrementing tab ID generator
    next_download_id: Arc<Mutex<usize>>, // Download counter
    tab_sidebar_width: u32,              // Sidebar width
    tab_bar_webview: Option<Rc<WebView>>, // Reference to tab bar UI
    download_overlay: Option<Rc<WebView>>, // Reference to downloads UI
    config: Config,                      // Configuration
}
```

**Max Tabs:** `const MAX_TABS: usize = 20`

### 2.3 Tab Lifecycle

#### Tab Creation

**Location:** `src/tabs/manager.rs` (lines 143-429)

```rust
pub fn create_tab(&mut self, window: &Window, url: &str) -> Result<usize, wry::Error>
pub fn create_tab_with_html(&mut self, window: &Window, html: &str) -> Result<usize, wry::Error>
```

**Process:**
1. Generate new tab ID (auto-incrementing)
2. Calculate bounds (accounting for sidebar)
3. Clean URL using `url_cleaner::clean_url()`
4. Create WebView with:
   - Position: X = TAB_SIDEBAR_WIDTH, Y = 0
   - Size: (window_width - sidebar, window_height)
   - Privacy user agent
   - DevTools enabled
   - Download handlers
   - IPC handler
   - Initialization scripts (privacy + vimium hints)
5. Store tab in HashMap
6. Return tab_id

**Initialization Scripts Injected:**
- Console override (redirects console output to backend via IPC)
- Privacy scripts (fingerprinting protection, tracking blocking)
- Vimium hints script (if vim_mode enabled)

#### Tab Switching

**Location:** `src/tabs/manager.rs` (lines 431-462)

```rust
pub fn switch_to_tab(&mut self, tab_id: usize) {
    // Hide current active tab
    if let Some(current_id) = self.active_tab_id {
        if let Some(current_tab) = self.tabs.get(&current_id) {
            current_tab.hide();  // Sets webview visibility to false
        }
    }
    
    // Show new active tab
    if let Some(new_tab) = self.tabs.get(&tab_id) {
        new_tab.show();  // Sets webview visibility to true
        self.active_tab_id = Some(tab_id);
        
        // Update tab bar UI
        let script = format!(
            "window.setActiveTab({}); window.updateUrlBar({});",
            tab_id, escaped_url
        );
        self.tab_bar_webview.evaluate_script(&script);
    }
}
```

**Mechanism:** Tabs are hidden/shown by toggling webview visibility, not destruction/creation.

#### Tab Closure

**Location:** `src/tabs/manager.rs` (lines 464-492)

```rust
pub fn close_tab(&mut self, tab_id: usize) {
    if let Some(tab) = self.tabs.remove(&tab_id) {
        drop(tab);  // Triggers Tab::Drop which calls cleanup()
        
        // Switch to adjacent tab if needed
        if self.active_tab_id == Some(tab_id) {
            // Find next tab intelligently
            let next_tab_id = /* logic to find adjacent or last tab */
            if let Some(next_id) = next_tab_id {
                self.switch_to_tab(next_id);
            }
        }
    }
}
```

**Cleanup:** When tab is dropped, it calls `Tab::cleanup()` which:
- Stops all audio/video
- Closes AudioContext
- Clears service worker caches
- Navigates to `about:blank`

### 2.4 Tab Navigation

**Location:** `src/tabs/manager.rs` (lines 503-538)

```rust
pub fn reload_active_tab(&self)
pub fn navigate_back(&self)
pub fn navigate_forward(&self)
pub fn navigate_to(&mut self, tab_id: usize, url: &str)
```

These methods execute JavaScript in the webview:
```rust
let _ = tab.webview.evaluate_script("window.location.reload();");
let _ = tab.webview.evaluate_script("window.history.back();");
let _ = tab.webview.evaluate_script("window.history.forward();");
```

### 2.5 Tab Resizing

When window is resized, all tabs must be resized:

**Location:** `src/tabs/manager.rs` (lines 555-583)

```rust
pub fn resize_all_tabs(&mut self, window: &Window)
pub fn resize_all_tabs_with_sidebar(&mut self, window: &Window, download_sidebar_width: u32)
```

These update bounds on each tab's webview:
```rust
let bounds = Rect {
    position: LogicalPosition::new(TAB_SIDEBAR_WIDTH as i32, 0).into(),
    size: LogicalSize::new(content_width, window_size.height).into(),
};
for tab in self.tabs.values() {
    let _ = tab.webview.set_bounds(bounds);
}
```

## 3. IPC COMMUNICATION PATTERNS

### 3.1 IPC Handler Architecture

**Location:** `src/main.rs` (lines 164-386) - Tab Bar IPC
**Location:** `src/tabs/manager.rs` (lines 377-418) - Content Tab IPC

IPC uses JSON message format:
```json
{
    "action": "action_name",
    "param1": "value1"
}
```

### 3.2 Tab Bar IPC Commands (Frontend → Backend)

**From:** Tab bar UI (sidebar)
**Handler Location:** `src/main.rs` lines 164-386

| Action | Parameters | Effect |
|--------|-----------|--------|
| `switch_tab` | `tabId: u64` | Switch to specified tab |
| `close_tab` | `tabId: u64` | Close tab, or quit if last tab |
| `new_tab` | - | Create new welcome page tab |
| `open_url_new_tab` | `url: string` | Create tab with URL |
| `reload_tab` | - | Reload active tab |
| `navigate_back` | - | Go back in history |
| `navigate_forward` | - | Go forward in history |
| `toggle_downloads` | - | Show/hide downloads panel |
| `focus_url_bar` | - | Focus URL input field |
| `navigate_url` | `url: string` | Navigate active tab to URL |
| `open_settings` | - | Open settings page |
| `load_settings` | - | Load settings from config |
| `save_settings` | `settings: object` | Save settings and persist to disk |
| `quit_app` | - | Terminate application |

### 3.3 Content Tab IPC Commands (Content → Backend)

**Handler Location:** `src/tabs/manager.rs` lines 377-418

| Action | Parameters | Effect |
|--------|-----------|--------|
| `update_title` | `tabId: usize, title: string` | Update tab name in sidebar |
| `update_navigation_state` | `canGoBack: bool, canGoForward: bool` | Update navigation buttons |
| `console_log` | `level: string, message: string` | Log message from page to stderr |

### 3.4 Backend → Frontend Script Execution

Backend executes JavaScript in frontend via `webview.evaluate_script()`:

```rust
// Update tab UI
let script = format!(
    "window.addTab({}, {}); window.setActiveTab({}); window.updateUrlBar({});",
    tab_id, escaped_url, tab_id, escaped_url
);
let _ = webview.evaluate_script(&script);
```

**Frontend API Methods Called:**
- `window.addTab(tabId, url)` - Add tab to sidebar
- `window.removeTab(tabId)` - Remove tab from sidebar
- `window.setActiveTab(tabId)` - Highlight active tab
- `window.updateUrlBar(url)` - Update URL display
- `window.updateTabTitle(tabId, title)` - Update tab name
- `window.updateNavigationButtons(canGoBack, canGoForward)` - Update nav buttons
- `window.addDownload(id, filename, progress)` - Add download to list
- `window.completeDownload(id, filename)` - Mark download complete
- `window.failDownload(id)` - Mark download failed

## 4. FRONTEND STATE AND UI

### 4.1 Tab Bar UI

**Location:** `src/ui/tab_bar/`

HTML Structure:
- Left sidebar (250px wide)
- Navigation buttons: back, forward, reload, new tab
- Tab list area
- URL input bar
- Bottom controls: downloads, settings

**State Variables (JavaScript):**
```javascript
window.tabs = [];                    // Array of tab objects
window.currentUrl = '';              // Currently displayed URL
window.tabAudioState = {};           // Audio playing state per tab
window.focusedTabIndex = -1;         // Sidebar focus for vim navigation
window.lastGKeyTime = 0;             // For 'gg' detection
window.sidebarFocused = false;       // Is sidebar focused
window.vimMode = boolean;            // Vim keybindings enabled
```

### 4.2 Tab Object Structure (JavaScript)

```javascript
{
    id: number,          // Unique tab ID
    url: string,         // Current URL
    title: string        // Display title (hostname or 'New Tab')
}
```

### 4.3 Frontend Key Handlers

**URL Bar:**
- Enter → navigate
- Focus → select all

**Tab Bar (Vim Mode):**
- `j` → move focus down
- `k` → move focus up
- `Enter` → activate focused tab
- `d` → close focused tab
- `n` → new tab
- `/` → focus URL bar
- `g` → jump to first tab (when pressed twice within 500ms)
- `G` (Shift+g) → jump to last tab

**Mouse:**
- Button 3 (back) → navigate back
- Button 4 (forward) → navigate forward

## 5. KEYBOARD SHORTCUT HANDLING

### 5.1 Global Hotkeys (System-Level)

**Location:** `src/main.rs` (lines 133-153, 448-602)

Uses `global-hotkey` crate for system-level shortcuts that work even when app is unfocused:

| Hotkey | macOS | Other | Action |
|--------|-------|-------|--------|
| Cmd/Ctrl+R | ✓ | ✓ | Reload active tab |
| Cmd/Ctrl+L | ✓ | ✓ | Focus URL bar |
| Cmd/Ctrl+J | ✓ | ✓ | Toggle downloads |
| Cmd/Ctrl+E | ✓ | ✓ | Focus sidebar |
| Cmd/Ctrl+F | ✓ | ✓ | Open find dialog |
| Cmd/Ctrl+T | ✓ | ✓ | New tab (with debounce 250ms) |
| Cmd/Ctrl+W | ✓ | ✓ | Close tab (with debounce 250ms) |
| Cmd/Ctrl+Q | ✓ | ✓ | Quit app |

**Debouncing:** Cmd+T and Cmd+W have 250ms debounce to prevent duplicate events.

### 5.2 Page-Level Vim Shortcuts

**Location:** `src/main.rs` (lines 653-747)

Executed in content tabs when vim_mode is enabled:

| Key | Action | Condition |
|-----|--------|-----------|
| `j` | Scroll down 60px | Not in input/textarea/contenteditable |
| `k` | Scroll up 60px | Not in input/textarea/contenteditable |
| `h` | Scroll left 40px | Not in input/textarea/contenteditable |
| `l` | Scroll right 40px | Not in input/textarea/contenteditable |
| `gg` | Scroll to top | Double-press 'g' within 500ms |
| `G` | Scroll to bottom | Shift+G |
| `f` | Show link hints (short labels) | Window has calmShowHints |
| `F` | Show link hints (all links) | Window has calmShowHints |
| `/` | Open search | Window has calmStartSearch |

## 6. STATE MANAGEMENT APPROACH

### 6.1 Backend State Architecture

```
main() 
├─ window: Rc<Window>
├─ event_loop: EventLoop
├─ config: Rc<RefCell<Config>>
├─ tab_manager: Rc<RefCell<TabManager>>
│  ├─ tabs: HashMap<usize, Tab>
│  ├─ active_tab_id: Option<usize>
│  ├─ next_tab_id: usize
│  └─ ... (see TabManager struct)
├─ tab_bar_webview_ref: Rc<RefCell<Option<Rc<WebView>>>>
├─ download_overlay_ref: Rc<RefCell<Option<Rc<WebView>>>>
├─ sidebar_visible: Rc<RefCell<bool>>
├─ should_quit: Rc<RefCell<bool>>
└─ ... (other state refs)
```

**Key Pattern:** `Rc<RefCell<T>>` allows multiple ownership with interior mutability within single-threaded event loop.

### 6.2 State Persistence

**Location:** `src/config.rs`

Configuration stored in `~/.calm.yml`:
- `default_url` - Homepage
- `search_engine` - Search provider
- `ui.vim_mode` - Vim keybindings enabled
- Privacy settings (tracking blocking, fingerprinting protection, etc.)

Settings saved via IPC `save_settings` command.

### 6.3 Event Loop Architecture

**Location:** `src/main.rs` (lines 440-751)

```rust
event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;
    
    // Check if should quit
    if *should_quit.borrow() {
        *control_flow = ControlFlow::Exit;
        return;
    }
    
    // Handle global hotkeys
    if let Ok(global_hotkey_event) = GlobalHotKeyEvent::receiver().try_recv() {
        // ... process hotkey
    }
    
    // Handle window/keyboard events
    match event {
        Event::WindowEvent { ... } => { /* ... */ }
        _ => {}
    }
});
```

## 7. EXISTING MULTI-WINDOW/TAB INFRASTRUCTURE

### 7.1 Current Capabilities

**Tab Management:**
- ✓ Multiple tabs within single window
- ✓ Tab switching (hide/show via visibility toggle)
- ✓ Tab creation with URL or HTML content
- ✓ Tab closure with cleanup
- ✓ Tab history navigation
- ✓ Tab resizing on window resize

**Sidebar Infrastructure:**
- ✓ Tab bar webview as child of main window
- ✓ Download overlay as child of main window
- ✓ Position-relative layout system
- ✓ Dynamic bounds setting

### 7.2 What's Missing for Multi-Window

**Not Implemented:**
- ✗ Multiple windows per application instance
- ✗ Cross-window tab communication
- ✗ Window state persistence (position, size)
- ✗ Window-level IPC routing
- ✗ Moving tabs between windows
- ✗ Window grouping/organization
- ✗ Per-window tab managers

### 7.3 Potential Integration Points for Multi-Window

1. **Window Manager Layer:**
   - Create `WindowManager` struct to manage multiple `Window` instances
   - Each window gets its own `TabManager`
   - Share `Config` across windows via `Arc<Mutex<Config>>`

2. **IPC Routing:**
   - Extend IPC to include window ID
   - Route IPC messages to correct window's handler
   - Allow cross-window communication

3. **State Sharing:**
   - Keep `Config` global/shared
   - Keep `GlobalHotKeyManager` global
   - Per-window: `TabManager`, `tab_bar_webview`, window state

4. **Window Creation:**
   - Extend `--version` command to support multi-window URLs
   - Implement `Cmd+N` hotkey to open new window

## 8. PRIVACY AND SECURITY ARCHITECTURE

### 8.1 Privacy Scripts Injection

**Location:** `src/privacy/` module

Every new tab receives initialization scripts:
```rust
let privacy_script = privacy::get_combined_privacy_script_with_config(&self.config.privacy);
let vimium_script = if self.config.ui.vim_mode { 
    vimium_hints::get_vimium_hints_script() 
} else { 
    "" 
};
```

Scripts wrapped in try-catch:
```rust
format!("try {{\n{}\n}} catch(e) {{ console.error('[PRIVACY] Error:', e); }}", privacy_script)
```

### 8.2 Custom Protocols

- `calmfile://localhost` - Local file access with MIME type detection
- `calm://welcome` - Welcome page
- `calm://settings` - Settings page

### 8.3 User Agent Spoofing

```rust
.with_user_agent(privacy::get_privacy_user_agent())
```

## 9. DEBUGGING AND LOGGING

### 9.1 Debug Mode

**Location:** `src/debug.rs`

Controlled by config setting `ui.debug`:
- Enabled via `~/.calm.yml`
- Global atomic flag: `DEBUG_ENABLED`
- Macro: `debug_log!()` - outputs to stderr with [CALM DEBUG] prefix

### 9.2 DevTools

Each tab has DevTools enabled:
```rust
.with_devtools(true)
```

### 9.3 Console Override

JavaScript console output redirected to backend via IPC:
```javascript
window.ipc.postMessage(JSON.stringify({
    action: 'console_log',
    level: 'log|error|warn',
    message: '...'
}));
```

Prints to stderr with `[BROWSER ERROR]`, `[BROWSER WARN]`, or `[BROWSER]` prefix.

## 10. DOWNLOAD MANAGEMENT

### 10.1 Download Handler

**Location:** `src/tabs/manager.rs` (lines 311-376)

Two handlers per tab:
- `with_download_started_handler` - Called when download begins
  - Extracts filename from headers or URL
  - Updates UI via `window.addDownload()`
  - Returns true to accept download
  
- `with_download_completed_handler` - Called when download finishes
  - Detects file type and adds extension if needed
  - Calls `window.completeDownload()` or `window.failDownload()`

### 10.2 Download Directory

Default: `~/Downloads`

Downloads UI managed by `download_overlay` webview on right side of window.

## 11. ARCHITECTURAL PATTERNS & DESIGN DECISIONS

### 11.1 Tab Visibility vs Destruction

**Pattern:** Tabs are hidden/shown, not created/destroyed on switch
- Pro: Faster switching, preserves page state
- Con: All tabs consuming memory simultaneously
- Max tabs: 20 (safety limit)

### 11.2 Single RefCell<T> for Mutable State

**Pattern:** Interior mutability with `Rc<RefCell<T>>`
- Single-threaded event loop
- No Arc needed (not thread-safe)
- Borrowing rules enforced at runtime, not compile-time
- Panic if two mutable borrows active

### 11.3 Webview as Child of Window

**Pattern:** Tab webviews are children of main window
- Pro: All in one window, easier resizing
- Con: No true multi-window support
- Works with TAO's platform-specific APIs

### 11.4 JSON-Based IPC

**Pattern:** JSON strings passed via `window.ipc.postMessage()`
- Pro: Simple, human-readable
- Con: No type safety, parsing overhead
- No schema validation

### 11.5 JavaScript Execution for UI Updates

**Pattern:** Backend calls `webview.evaluate_script()` to update frontend
- Pro: Direct DOM manipulation, no callback complexity
- Con: Slow for frequent updates, no error handling
- Used for all tab bar updates

## 12. SUMMARY: KEY FILES AND THEIR PURPOSES

| File | Purpose |
|------|---------|
| `src/main.rs` | Window creation, event loop, IPC handler, global hotkeys |
| `src/tabs/manager.rs` | Tab CRUD operations, tab switching, tab communication |
| `src/tabs/tab.rs` | Tab data structure, cleanup on drop |
| `src/tabs/mod.rs` | Module exports |
| `src/ui/mod.rs` | UI module exports |
| `src/ui/tab_bar/mod.rs` | Tab bar HTML/CSS/JS assembly |
| `src/ui/tab_bar/html.rs` | Tab bar HTML structure |
| `src/ui/tab_bar/script.rs` | Tab bar JavaScript (state, event handlers) |
| `src/ui/tab_bar/styles.rs` | Tab bar CSS |
| `src/config.rs` | Configuration structures and persistence |
| `src/privacy/` | Privacy scripts injected into every page |
| `src/vimium_hints.rs` | Link hint system for vim navigation |
| `src/debug.rs` | Debug logging macro and control |

## 13. IMPORTANT CONSTRAINTS FOR MULTI-WINDOW IMPLEMENTATION

1. **Global Hotkeys:** Currently registered once at app start, would need per-window handling
2. **Config Sharing:** Must remain global to maintain consistency
3. **Tab Limits:** 20 tab max - would apply per-window or globally?
4. **Event Loop:** Single event loop handles all windows
5. **Download Manager:** Currently assumes single window, would need refactoring
6. **URL Bar Focus:** Cmd+L behavior assumes active window
7. **Settings:** Window position/size not currently persisted

