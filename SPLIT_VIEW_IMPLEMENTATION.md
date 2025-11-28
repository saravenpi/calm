# Split View Implementation

## Overview

Calm Browser now supports split-view functionality, allowing you to view two tabs side-by-side in the same window. This feature is perfect for comparing documents, referencing documentation while coding, or monitoring multiple pages simultaneously.

## Features

### 1. Split View Toggle (Cmd+Shift+S)

- **Keyboard Shortcut**: `Cmd+Shift+S` (macOS) / `Ctrl+Shift+S` (Windows/Linux)
- **Behavior**: Toggles between normal view and split view
- **Requirements**: At least 2 tabs must be open in the window
- **Initial Layout**: Opens in vertical split (side-by-side) with 50/50 ratio

### 2. Dual Pane Layout

- **Primary Pane**: Shows the currently active tab
- **Secondary Pane**: Shows the next available tab
- **Both panes**: Fully functional, independent webviews
- **Visibility**: Hidden tabs remain in memory but are not displayed

### 3. Split Orientations

- **Vertical Split**: Side-by-side layout (default)
- **Horizontal Split**: Top-bottom layout
- **Toggle**: Press `Cmd+Shift+S` while in split view to change orientation (planned)

### 4. Split View Controls

#### Keyboard Shortcuts
- `Cmd+Shift+S` - Toggle split view on/off
- IPC commands available for:
  - `toggle_split_orientation` - Switch between vertical/horizontal
  - `swap_split_panes` - Swap which tab is in which pane

## Architecture

### Module Structure

```
src/tabs/split_view.rs - New module (217 lines)
  ├─ SplitViewState     - State management
  ├─ SplitViewManager   - Business logic
  └─ SplitOrientation   - Vertical/Horizontal enum
```

### Integration Points

#### TabManager Integration

```rust
pub struct TabManager {
    // ... existing fields
    split_view: SplitViewManager,  // NEW
}
```

New methods added to TabManager:
- `toggle_split_view(&mut self, window: &Window) -> bool`
- `is_split_view_enabled(&self) -> bool`
- `toggle_split_orientation(&mut self, window: &Window)`
- `swap_split_panes(&mut self)`
- `get_split_view_state(&self) -> Option<(usize, usize, SplitOrientation)>`
- `update_split_view_layout(&mut self, window: &Window, download_sidebar: Option<u32>)`

#### Resize Behavior

Both `resize_all_tabs()` and `resize_all_tabs_with_sidebar()` now check if split view is enabled and call `update_split_view_layout()` instead of applying uniform bounds to all tabs.

### State Management

```rust
pub struct SplitViewState {
    enabled: bool,
    orientation: SplitOrientation,     // Vertical or Horizontal
    primary_tab_id: Option<usize>,     // Left/Top tab
    secondary_tab_id: Option<usize>,   // Right/Bottom tab
    split_ratio: f32,                  // 0.0 to 1.0 (default: 0.5)
    active_pane: SplitPane,            // Primary or Secondary
}
```

### Layout Calculation

The `calculate_bounds()` method computes webview rectangles based on:
- Window size
- Tab sidebar width (250px)
- Download sidebar width (if visible)
- Split orientation
- Split ratio

#### Vertical Split Example

```
┌──────┬─────────────────┬─────────────────┬──────┐
│ Tabs │  Primary Pane   │ Secondary Pane  │ Down │
│ 250px│   (Tab ID: 1)   │   (Tab ID: 2)   │ 360px│
│      │                 │                 │      │
│      │                 │                 │      │
└──────┴─────────────────┴─────────────────┴──────┘
```

#### Horizontal Split Example

```
┌──────┬─────────────────────────────────┬──────┐
│ Tabs │      Primary Pane (Tab 1)       │ Down │
│ 250px├─────────────────────────────────┤ 360px│
│      │     Secondary Pane (Tab 2)      │      │
│      │                                 │      │
└──────┴─────────────────────────────────┴──────┘
```

## Implementation Details

### Window Resize Handling

When split view is enabled:

1. Window resize event fires
2. `handle_window_resize()` is called
3. TabManager's `resize_all_tabs()` is invoked
4. Detects split view is enabled
5. Calls `update_split_view_layout()` instead of uniform resize
6. Calculates new bounds for primary and secondary panes
7. Hides all other tabs
8. Shows only the two split tabs with correct bounds

### Tab Visibility Management

```rust
fn update_split_view_layout(&mut self, window: &Window, download_sidebar: Option<u32>) {
    let (primary_bounds, secondary_bounds) =
        self.split_view.state().calculate_bounds(window, ...);

    for (tab_id, tab) in &self.tabs {
        if state.primary_tab_id == Some(*tab_id) {
            tab.webview.set_bounds(primary_bounds);
            tab.show();  // Make visible
        } else if state.secondary_tab_id == Some(*tab_id) {
            tab.webview.set_bounds(secondary_bounds);
            tab.show();  // Make visible
        } else {
            tab.hide();  // Hide all other tabs
        }
    }
}
```

### IPC Commands

Three new IPC commands added to `src/window/builder.rs`:

```rust
Some("toggle_split_view") => {
    let toggled = tab_manager.borrow_mut().toggle_split_view(&window);
}

Some("toggle_split_orientation") => {
    tab_manager.borrow_mut().toggle_split_orientation(&window);
}

Some("swap_split_panes") => {
    tab_manager.borrow_mut().swap_split_panes();
}
```

### Global Hotkey Registration

In `src/main.rs`:

```rust
let hotkey_split_view = HotKey::new(
    Some(cmd_or_ctrl | Modifiers::SHIFT),
    Code::KeyS
);

hotkey_manager.register(hotkey_split_view)
    .expect("Failed to register Cmd+Shift+S");
```

### Hotkey Handler

```rust
} else if hotkey_id == hotkey_split_view.id() {
    debug_log!("=== Cmd+Shift+S GlobalHotKey FIRED - toggling split view ===");
    let toggled = components.tab_manager.borrow_mut()
        .toggle_split_view(&components.window);
    if toggled {
        debug_log!("Split view enabled");
    } else {
        debug_log!("Split view disabled");
    }
}
```

## Usage Examples

### Basic Usage

1. Open at least 2 tabs
2. Press `Cmd+Shift+S`
3. The current tab and next tab appear side-by-side
4. Navigate in either pane independently
5. Press `Cmd+Shift+S` again to return to normal view

### With Download Sidebar

The split view automatically adjusts when the download sidebar is opened:

```
┌──────┬──────────┬──────────┬─────────────┐
│ Tabs │ Primary  │Secondary │  Downloads  │
│      │          │          │             │
└──────┴──────────┴──────────┴─────────────┘
```

### Tab Count Requirements

- **Minimum**: 2 tabs needed to enable split view
- **Behavior**: If you have only 1 tab, `Cmd+Shift+S` does nothing
- **Auto-disable**: Closing one of the split tabs disables split view

## Technical Specifications

### Performance

- **Layout calculation**: O(1) - simple arithmetic
- **Tab iteration**: O(n) where n = total tabs (typically ≤ 20)
- **Bounds update**: Instant - no animations
- **Memory overhead**: ~200 bytes for SplitViewState per window

### Bounds Calculation Formula

#### Vertical Split

```rust
let available_width = window_width - sidebar_width - download_width;
let split_pos = (available_width as f32 * split_ratio) as u32;

primary_bounds = Rect {
    x: sidebar_width,
    y: 0,
    width: split_pos,
    height: window_height,
};

secondary_bounds = Rect {
    x: sidebar_width + split_pos,
    y: 0,
    width: available_width - split_pos,
    height: window_height,
};
```

#### Horizontal Split

```rust
let available_height = window_height;
let split_pos = (available_height as f32 * split_ratio) as u32;

primary_bounds = Rect {
    x: sidebar_width,
    y: 0,
    width: available_width,
    height: split_pos,
};

secondary_bounds = Rect {
    x: sidebar_width,
    y: split_pos,
    width: available_width,
    height: available_height - split_pos,
};
```

### Split Ratio Constraints

- **Default**: 0.5 (50/50 split)
- **Range**: 0.2 to 0.8 (20% to 80%)
- **Prevents**: Panes from becoming too small to be useful
- **Future**: Drag-to-resize will use `adjust_split_ratio(delta: f32)`

## Future Enhancements

### Planned Features

1. **Drag-to-Resize Split**: Click and drag the divider to adjust split ratio
2. **UI Indicators**: Visual divider line between panes
3. **Pane Focus**: Visual indication of which pane is active
4. **Keyboard Navigation**: Switch active pane with keyboard
5. **Triple/Quad Split**: Support for more than 2 panes
6. **Saved Layouts**: Remember split configurations per workspace

### UI Controls (Pending)

- Visual split divider with drag handle
- Buttons in tab bar for split controls
- Active pane highlighting
- Minimap showing split layout

## Code Files

### New Files

- `src/tabs/split_view.rs` - Complete split view implementation (217 lines)

### Modified Files

- `src/tabs/mod.rs` - Export split view types
- `src/tabs/manager.rs` - Integration with TabManager (+100 lines)
- `src/main.rs` - Hotkey registration and handling (+25 lines)
- `src/window/builder.rs` - IPC commands (+18 lines)

### Total Changes

- **Files added**: 1
- **Files modified**: 4
- **Lines added**: ~360
- **Lines modified**: ~40

## Testing

The implementation has been tested for:

- ✅ Toggle split view with 2+ tabs
- ✅ Proper bounds calculation (vertical split)
- ✅ Horizontal split orientation (code ready, UI toggle pending)
- ✅ Window resize handling in split mode
- ✅ Download sidebar interaction with split view
- ✅ Tab visibility management (show 2, hide others)
- ✅ Return to normal view (all tabs resized correctly)
- ✅ Global hotkey (Cmd+Shift+S) works
- ⏳ IPC commands (implemented but not UI-triggered yet)
- ⏳ Swap panes (implemented but not UI-accessible yet)

## Known Limitations

1. **No visual divider**: Can't see where split occurs (tabs just appear side-by-side)
2. **No resize drag**: Split ratio is fixed at 50/50
3. **No pane selection**: Can't choose which tabs to split
4. **No orientation UI**: Must use IPC to change from vertical to horizontal
5. **Binary split only**: No 3-way or 4-way splits yet

These will be addressed in future updates with proper UI controls.

## Version

- **Feature Added**: v0.3.0
- **Date**: November 27, 2025
- **Author**: saravenpi
- **Status**: Core functionality complete, UI enhancements pending

## Related Documentation

- `MULTI_WINDOW_IMPLEMENTATION.md` - Multi-window support details
- `ARCHITECTURE.md` - Overall browser architecture
- `TODO.md` - Feature roadmap and pending items
