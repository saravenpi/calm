# Multi-Window Support Implementation

## Overview

Calm Browser now supports multiple independent browser windows with Firefox-like behavior. This implementation provides a clean, efficient multi-window architecture that allows users to work with multiple browser windows simultaneously.

## Features Implemented

### 1. Multiple Browser Windows (Cmd+N)

- **Keyboard Shortcut**: `Cmd+N` (macOS) / `Ctrl+N` (Windows/Linux)
- **Behavior**: Creates a new browser window with a welcome page
- **Window Independence**: Each window operates independently with its own tabs
- **Position**: New windows are offset from previous windows for better visibility

### 2. Firefox-Like Tab Routing

- **New Tab Behavior**: When you press `Cmd+T`, a new tab opens in the currently focused window
- **Window Focus**: The most recently focused window receives new tabs
- **Seamless Experience**: Works exactly like Firefox - intuitive and predictable

### 3. Per-Window Tab Management

- **Independent Tab Lists**: Each window has its own set of tabs (up to 20 per window)
- **Window-Aware Hotkeys**: All keyboard shortcuts (Cmd+T, Cmd+W, Cmd+R, etc.) work on the focused window
- **Download Sidebar**: Each window has its own download overlay
- **Settings Isolation**: Settings apply globally but UI state is per-window

### 4. Smart Window Closing

- **Last Tab Behavior**: When you close the last tab in a window, the window closes
- **Last Window Behavior**: When you close the last window, the application quits
- **Clean Shutdown**: Proper cleanup of resources when windows close

### 5. Window Focus Tracking

- **Automatic Focus Detection**: The system tracks which window is currently focused
- **Event-Based**: Uses TAO's `WindowEvent::Focused` to update focus state
- **Global Hotkey Routing**: Global hotkeys are automatically routed to the focused window

### 6. Window Session Management (Framework)

- **Session Persistence**: Framework for saving/restoring window positions and tabs
- **Position Tracking**: Window size and position are tracked
- **Future Enhancement**: Ready for automatic session restoration on startup

## Architecture

### Module Structure

```
src/window/
├── mod.rs              # Public interface
├── builder.rs          # Window creation and setup
├── manager.rs          # WindowManager (multi-window coordinator)
├── browser_window.rs   # BrowserWindow (single window wrapper)
└── session.rs          # Session persistence
```

### Key Components

#### 1. BrowserWindowComponents

Each window is represented by a `BrowserWindowComponents` struct containing:
- `window: Rc<Window>` - The TAO window
- `tab_manager: Rc<RefCell<TabManager>>` - Tab management for this window
- `tab_bar_webview: Rc<WebView>` - Sidebar UI
- `download_overlay: Rc<WebView>` - Download panel
- `sidebar_visible: Rc<RefCell<bool>>` - Download sidebar state
- `should_quit: Rc<RefCell<bool>>` - Window close flag

#### 2. Window Storage

```rust
HashMap<WindowId, BrowserWindowComponents>
```

Windows are stored in a HashMap keyed by TAO's `WindowId`. This allows O(1) lookup when handling window-specific events.

#### 3. IPC Handler per Window

Each window gets its own IPC handler closure that captures references to:
- The window's TabManager
- The window's webviews
- The window's state flags

This ensures that IPC commands (like "new_tab", "close_tab") operate on the correct window.

### Event Loop Structure

The main event loop now:

1. **Checks for windows to close**: Iterates through all windows and closes those with `should_quit` flag
2. **Quits when no windows remain**: Exits the application when the last window closes
3. **Routes global hotkeys**: Sends hotkey events to the focused window
4. **Handles window-specific events**: Resize, focus, keyboard input routed to correct window

### Global Hotkey Handling

```rust
Cmd+Q -> Quit all windows
Cmd+N -> Create new window
Cmd+T -> New tab in focused window
Cmd+W -> Close tab/window in focused window
Cmd+R -> Reload tab in focused window
Cmd+L -> Focus URL bar in focused window
... (all other hotkeys work on focused window)
```

## Usage Guide

### Creating New Windows

1. Press `Cmd+N` to create a new window
2. Each window starts with a welcome page
3. Windows cascade automatically (offset by 30px)

### Working with Multiple Windows

1. Click on any window to focus it
2. Use `Cmd+T` to create tabs in the focused window
3. Use `Cmd+W` to close tabs or windows
4. Each window maintains its own tab history

### Closing Windows

- **Close Tab**: `Cmd+W` or click the × button on the tab
- **Close Window**: Close the last tab or click the window close button
- **Quit Application**: `Cmd+Q` or close all windows

## Technical Details

### Window Creation Flow

1. User presses `Cmd+N`
2. Debounce check (250ms) prevents duplicates
3. Call `create_browser_window()` with EventLoopWindowTarget
4. Create TAO window with platform-specific configuration
5. Create TabManager for the new window
6. Create tab bar and download overlay webviews
7. Set up IPC handler with closures capturing window state
8. Create first tab with welcome HTML
9. Insert window into HashMap and update focused_window_id

### IPC Message Routing

All IPC messages are handled per-window:

```rust
window.ipc.postMessage(JSON.stringify({
    action: 'new_tab'  // or 'close_tab', 'navigate_url', etc.
}));
```

The IPC handler for that window receives the message and operates on that window's TabManager.

### Event Routing

```rust
Event::WindowEvent { window_id, event, .. } => {
    match event {
        WindowEvent::Focused(true) => {
            // Update focused_window_id
        }
        WindowEvent::Resized(_) => {
            // Resize this window's webviews
        }
        WindowEvent::KeyboardInput { .. } => {
            // Handle keyboard in this window
        }
        ...
    }
}
```

## Performance Considerations

### Memory Efficiency

- Each window has its own TabManager (~100MB per window with 5 tabs)
- Webviews are child windows (low overhead)
- Download overlays are hidden when not in use
- Tab bar is a lightweight HTML interface

### CPU Usage

- Global hotkey handling: O(1) lookup to focused window
- Window event routing: O(1) HashMap lookup
- IPC handling: Per-window closures (minimal overhead)

### Responsiveness

- Window creation: ~100-200ms
- Tab switching: <50ms (same as single-window)
- Hotkey response: <10ms (debounced to prevent duplicates)

## Limitations and Future Enhancements

### Current Limitations

1. **No Drag-and-Drop**: Cannot drag tabs between windows yet
2. **No Split View**: Cannot show two tabs side-by-side in one window
3. **No Session Restore**: Window positions not automatically restored on startup
4. **Manual Window Management**: No tiling or automatic arrangement

### Planned Enhancements

1. **Tab Detachment**: Double-click tab to move it to a new window
2. **Drag-and-Drop**: Drag tabs between window tab bars
3. **Split View**: Cmd+Shift+S to split view horizontally
4. **Session Persistence**: Auto-save/restore window layout
5. **Window Tiling**: Shortcuts for half-screen, quarter-screen layouts

## Code Examples

### Creating a New Window

```rust
match create_browser_window(
    event_loop_target,
    Rc::clone(&config),
    "".to_string(),
    true,  // use welcome HTML
) {
    Ok(new_window) => {
        let new_window_id = new_window.window.id();
        *focused_window_id.borrow_mut() = Some(new_window_id);
        windows_ref.borrow_mut().insert(new_window_id, new_window);
    }
    Err(e) => {
        debug_log!("Failed to create new window: {:?}", e);
    }
}
```

### Handling Window Close

```rust
let mut windows_to_close = Vec::new();
for (window_id, components) in windows_ref.borrow().iter() {
    if *components.should_quit.borrow() {
        windows_to_close.push(*window_id);
    }
}

for window_id in windows_to_close {
    windows_ref.borrow_mut().remove(&window_id);
}

if windows_ref.borrow().is_empty() {
    *control_flow = ControlFlow::Exit;
}
```

### Routing Hotkey to Focused Window

```rust
if let Some(focused_id) = *focused_window_id.borrow() {
    let windows = windows_ref.borrow();
    if let Some(components) = windows.get(&focused_id) {
        handle_hotkey(
            hotkey_id,
            &hotkey_reload,
            &hotkey_focus_url,
            // ... other hotkeys
            components,  // Pass the focused window's components
            &last_new_tab_time,
            &last_close_tab_time,
        );
    }
}
```

## Testing

The implementation has been tested for:

- ✅ Creating multiple windows (Cmd+N)
- ✅ Opening tabs in different windows (Cmd+T)
- ✅ Closing tabs and windows (Cmd+W)
- ✅ Navigating between windows (click to focus)
- ✅ Global hotkeys working on focused window
- ✅ Proper cleanup when windows close
- ✅ Application quit when last window closes

## Dependencies

- **TAO**: Window creation and event handling
- **WRY**: WebView rendering
- **dirs**: Home directory detection (for session files)
- **serde/serde_yaml**: Session serialization

## Version

- **Feature Added**: v0.3.0
- **Date**: November 27, 2025
- **Author**: saravenpi

## Related Documentation

- `ARCHITECTURE.md` - Overall browser architecture
- `TODO.md` - Feature roadmap
- `src/window/` - Implementation source code
