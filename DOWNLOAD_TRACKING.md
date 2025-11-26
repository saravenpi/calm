# Download Tracking System

## Overview

Calm Browser implements a download tracking system that monitors and displays
download progress through a dedicated download overlay UI.

## Architecture

### Components

1. **TabManager** (`src/tabs/manager.rs`)
   - Manages download event handlers
   - Assigns unique download IDs
   - Bridges WRY download events to the UI overlay

2. **Download Overlay** (`src/ui/download_overlay/`)
   - Displays active and completed downloads
   - Shows download progress, status, and errors
   - Auto-removes completed/failed downloads after timeout

3. **URL Cleaner** (`src/url_cleaner.rs`)
   - Removes tracking parameters from URLs
   - Cleans filenames for downloads
   - Ensures privacy during navigation

## How It Works

### Download Flow

```
1. User clicks download link
   ↓
2. WRY fires download_started event
   ↓
3. TabManager creates unique download ID
   ↓
4. Filename is sanitized (tracking params removed, URL decoded)
   ↓
5. Download path set to ~/Downloads/<cleaned_filename>
   ↓
6. UI overlay notified via window.addDownload()
   ↓
7. Download appears in overlay panel
   ↓
8. WRY fires download_completed event (success or failure)
   ↓
9. UI updated via window.completeDownload() or window.failDownload()
   ↓
10. Download auto-removed after 3s (success) or 5s (failure)
```

### Unique Download IDs

Each download receives a unique sequential ID, independent of tab IDs. This
allows multiple concurrent downloads per tab without collisions.

**Implementation:**
- Uses `Arc<Mutex<usize>>` for thread-safe ID generation
- ID is generated inside the download handler, ensuring each download gets a unique ID
- A HashMap maps download paths to IDs, allowing the completed handler to identify downloads
```rust
let download_id = {
    let mut id = download_id_counter.lock().unwrap();
    let current_id = *id;
    *id += 1;
    current_id
};
```

### Filename Sanitization

Downloaded files have their filenames cleaned to remove tracking parameters
and URL encoding:

**Before:** `file.pdf?utm_source=google&token=abc%20def`
**After:** `file.pdf`

**For URLs without filenames:** Uses timestamp-based naming (e.g., `download_1732630000`)
to avoid conflicts and prevent incorrect file extensions.

**Implementation:** `src/tabs/manager.rs:29-58`

### Download States

Downloads can be in three states:

- **In Progress**: Download ongoing (shows 0% due to WRY limitation)
- **Completed**: Download successful (green indicator, 3s timeout)
- **Failed**: Download failed (red indicator, 5s timeout)

## Known Limitations

### 1. No Real-Time Progress Tracking

**Issue:** Download progress is not updated in real-time
**Cause:** WRY library does not expose download progress events
**Impact:** Downloads show 0% → instantly complete/fail
**Workaround:** None available without WRY library changes

The JavaScript function `window.updateDownloadProgress()` exists but is never
called from Rust because WRY doesn't provide progress callbacks.

### 2. No Download Persistence

**Issue:** Download history is lost on browser restart
**Cause:** State only maintained in JavaScript memory
**Impact:** Cannot view past downloads or resume failed downloads
**Future:** Could implement SQLite/JSON storage

### 3. Limited WRY Download API

**Issue:** Cannot cancel in-progress downloads
**Cause:** WRY only exposes `started` and `completed` handlers
**Impact:** Users must wait for downloads to finish or fail

## URL Tracking Parameter Removal

Calm Browser automatically removes 60+ tracking parameters from URLs,
including:

### Marketing & Analytics
- `utm_*` (Google Analytics UTM parameters)
- `ga_*` (Google Analytics)
- `mtm_*` (Matomo tracking)
- `mkt_tok` (Marketo)
- `mc_cid`, `mc_eid` (Mailchimp)

### Click IDs
- `fbclid` (Facebook)
- `gclid` (Google)
- `msclkid` (Microsoft/Bing)
- `twclid` (Twitter)
- `yclid` (Yandex)
- `dclid` (DoubleClick)

### Other Trackers
- `igshid` (Instagram)
- `wickedid` (Wicked Reports)
- `vero_id`, `vero_conv` (Vero email tracking)
- `_hsenc`, `_hsmi`, `__hsfp`, `__hssc`, `__hstc` (HubSpot)
- `spm` (Chinese e-commerce tracking)
- Plus prefix patterns: `stm_*`, `pk_*`

**Full list:** `src/url_cleaner.rs:7-70`

### When URLs Are Cleaned

1. **Tab Creation:** URLs cleaned when opening new tabs
2. **Navigation:** URLs cleaned when navigating to new pages
3. **Downloads:** Filenames cleaned before saving to disk

## Testing

Run tests with:
```bash
cargo test
```

### Test Coverage

- URL parameter removal (UTM, fbclid, etc.)
- Prefix pattern matching (stm_*, pk_*)
- Legitimate parameter preservation
- Edge cases (no params, only tracking params)

## API Reference

### TabManager Methods

```rust
pub fn create_tab(&mut self, window: &Window, url: &str) -> Result<usize>
pub fn navigate_to(&mut self, tab_id: usize, url: &str)
pub fn set_download_overlay(&mut self, webview: Rc<WebView>)
```

### JavaScript API (Download Overlay)

```javascript
window.addDownload(id, filename, totalBytes)
window.updateDownloadProgress(id, downloadedBytes, totalBytes)  // Not used
window.completeDownload(id)
window.failDownload(id)
window.toggleVisibility(visible)
```

### URL Cleaner

```rust
pub fn clean_url(url_str: &str) -> Result<String, url::ParseError>
```

## Future Improvements

1. **Progress Tracking**: Implement polling or system APIs to track progress
2. **Download History**: Add SQLite database for persistent storage
3. **Download Management**: Pause, resume, cancel functionality
4. **Notifications**: System notifications for completed downloads
5. **Download Queue**: Concurrent download limit and queuing
6. **Statistics**: Track total downloads, bandwidth usage
7. **Custom Save Location**: Per-download save path selection

## Recent Changes

### 2025-11-26 (Latest Fix)
- **Fixed critical download ID reuse bug**: Each download now gets a unique ID instead of reusing the same ID per tab
- **Fixed incorrect file extensions**: Removed automatic .html extension addition for files without extensions
- **Improved filename handling**: Files without clear names now use timestamp-based naming (e.g., `download_1732630000`)
- **Thread-safe ID generation**: Implemented Arc<Mutex<usize>> for safe concurrent downloads
- **Path-to-ID mapping**: Added HashMap to correlate download start and completion events

### 2025-11-26 (Previous)
- Fixed download ID collision bug (was using tab_id)
- Added proper filename sanitization (URL decoding + query param removal)
- Implemented download failure notifications
- Added comprehensive URL tracking parameter removal (60+ params)
- Failed downloads display with red indicator for 5 seconds
- URLs cleaned on tab creation and navigation
- All tests passing (6/6)
