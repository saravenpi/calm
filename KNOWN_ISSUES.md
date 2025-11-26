# Known Issues and Limitations

## iframe-Based Tabs (Current Implementation)

Calm Browser currently uses iframes for tab management. This has several important limitations:

### Websites That Don't Work

Many popular websites block being loaded in iframes using **X-Frame-Options** headers. These sites will appear blank:

**❌ Sites that DON'T work:**
- google.com (X-Frame-Options: SAMEORIGIN)
- facebook.com (X-Frame-Options: DENY)
- twitter.com / x.com (X-Frame-Options: DENY)
- instagram.com (X-Frame-Options: DENY)
- youtube.com (mixed - some videos work, main site doesn't)
- github.com (partially - some pages work)
- reddit.com (X-Frame-Options: SAMEORIGIN)
- amazon.com (X-Frame-Options: DENY)
- netflix.com (X-Frame-Options: DENY)

**✅ Sites that DO work:**
- start.duckduckgo.com (iframe-friendly)
- wikipedia.org (works)
- archive.org (works)
- stackoverflow.com (works sometimes)
- news.ycombinator.com (works)
- lobste.rs (works)
- many personal blogs and smaller sites

### How to Check If a Site Works

Open the browser console (right-click → Inspect → Console):
- If you see **"Cannot access iframe content (likely X-Frame-Options)"** - the site blocks iframes
- The tab will show the domain name but the content area will be blank

### Workarounds

1. **Use iframe-friendly sites**: DuckDuckGo, Wikipedia, HackerNews, etc.
2. **Check the console**: Look for X-Frame-Options errors
3. **Wait for native WebView implementation**: Future versions will use native WebView per tab

## Why This Limitation Exists

Websites use X-Frame-Options headers to prevent **clickjacking attacks**. This is a security feature that prevents malicious sites from embedding their content in invisible iframes.

Example headers:
```
X-Frame-Options: DENY            # Never allow framing
X-Frame-Options: SAMEORIGIN      # Only allow same domain
Content-Security-Policy: frame-ancestors 'none'  # Modern alternative
```

## Future Solution

**Planned for v0.3.0:**
- Replace iframe-based tabs with native WebView instances
- Each tab will have its own WebView (like a real browser)
- No X-Frame-Options limitations
- Full website compatibility
- Better isolation and security

## Testing Your Site

To test if a site will work in Calm:

```bash
# Check X-Frame-Options header
curl -I https://example.com | grep -i frame

# Try loading in Calm
calm https://example.com
```

## Recommended Sites to Test

Try these iframe-friendly sites:

```bash
calm https://start.duckduckgo.com
calm https://en.wikipedia.org
calm https://news.ycombinator.com
calm https://lobste.rs
calm https://archive.org
```

## Current Development Status

- ✅ Tab management UI
- ✅ Keyboard shortcuts
- ✅ Privacy protections (JS level)
- ✅ Address overlay
- ⚠️ iframe-based rendering (limited compatibility)
- 🚧 Native WebView per tab (planned)
- 🚧 Full website compatibility (planned)

## Reporting Issues

If a site that should work doesn't:
1. Open Developer Console (right-click → Inspect)
2. Check for errors
3. Note the X-Frame-Options header
4. Report with console output

## Temporary Recommendation

For now, Calm Browser works best for:
- Privacy-focused browsing of iframe-friendly sites
- Reading documentation
- Browsing technical content (HN, Stack Overflow)
- Wikipedia research
- Archive.org browsing

For sites like Google, Facebook, Twitter, use your regular browser until we implement native WebView tabs.
