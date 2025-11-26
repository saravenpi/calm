# Privacy Features Documentation

## Overview

Calm Browser implements comprehensive privacy protections inspired by Tor Browser and Brave. All protections are enabled by default with zero configuration required and zero data collection.

Calm implements 18 layers of fingerprinting protection plus tracking domain blocking.

## Core Philosophy

- Privacy by default - All protections active from first launch
- Zero telemetry - No data collection
- Zero trust - Every website treated as potentially hostile
- Blend in - Normalize browser characteristics across all users

## JavaScript-Based Protections

Calm injects privacy JavaScript into every webpage before it loads.

### 1. Hardware Spoofing

Websites can fingerprint based on CPU cores, RAM, and platform.

Protection:
- `hardwareConcurrency`: 4 cores
- `deviceMemory`: 8GB
- `platform`: Win32
- `maxTouchPoints`: 0
- `vendor`: Google Inc.
- Impact: Normalized hardware characteristics

### 2. Screen Normalization

Screen resolution can be used for fingerprinting.

Protection:
- Resolution: 1920x1080
- Color depth: 24-bit
- Available height: 1040px

### 3. Timezone Normalization

Timezone reveals geographic location.

Protection: All users report UTC (offset 0)

### 4. Battery API Blocking

Battery level creates unique fingerprints.

Protection: `navigator.getBattery()` returns rejected promise

### 5. WebRTC Blocking

WebRTC can leak real IP address even with VPN.

Protection: Blocks all WebRTC APIs:
- `RTCPeerConnection`
- `RTCDataChannel`
- `RTCSessionDescription`
- `RTCIceCandidate`

### 6. Media Device Blocking

Device enumeration creates fingerprints.

Protection:
- `enumerateDevices()` returns empty array
- `getUserMedia()` rejected
- `getDisplayMedia()` rejected

### 7. Geolocation Blocking

Protection: All geolocation APIs throw errors

### 8. Network Information Spoofing

Network connection info can fingerprint.

Protection: `navigator.connection` reports standardized values:
- effectiveType: 4g
- downlink: 10
- rtt: 50
- type: wifi

### 9. Storage Quota Spoofing

Protection: `navigator.storage.estimate()` returns normalized values (8GB quota, 1GB used)

### 10. Permissions API Hardening

Protection: Sensitive permissions (geolocation, notifications, midi, camera, microphone) always return 'denied'

### 11. Credentials API Blocking

Protection: All credential APIs return null/empty results

### 12. Privacy Headers

Automatic headers on all requests:
- `DNT: 1` (Do Not Track)
- `Sec-GPC: 1` (Global Privacy Control)
- `Sec-CH-UA`, `Sec-CH-UA-Mobile`, `Sec-CH-UA-Platform` (Client Hints)
- Referrer stripping

### 13. Tracking Domain Blocking

Blocks 30+ tracking domains at multiple injection points:

Blocked domains:
- Google: google-analytics.com, googletagmanager.com, doubleclick.net, googlesyndication.com, adservice.google.com, pagead2.googlesyndication.com
- Facebook: facebook.com/tr, facebook.net, connect.facebook.net, pixel.facebook.com
- Twitter/X: analytics.twitter.com, ads-twitter.com, static.ads-twitter.com
- TikTok: tiktok.com/i18n/pixel, ads.tiktok.com, analytics.tiktok.com
- Session replay: hotjar.com, mouseflow.com, crazyegg.com, luckyorange.com, clicktale.com, inspectlet.com, fullstory.com
- Analytics: mixpanel.com, segment.com, heap.io, amplitude.com, chartbeat.com, newrelic.com
- Other: amazon-adsystem.com, scorecardresearch.com, quantserve.com, optimizely.com

Blocking methods:
1. XMLHttpRequest interception
2. Fetch API interception
3. appendChild() interception
4. insertBefore() interception

### 14. User Agent Normalization

Common Chrome user agent:
```
Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36
```

### 15. Canvas Noise Injection

Canvas rendering produces unique fingerprints.

Protection:
- Adds ±1-2 pixel noise to RGB channels
- Invisible to humans
- Randomized on every read operation
- Methods: `toDataURL()`, `getImageData()`

### 16. WebGL Spoofing

WebGL reveals GPU details.

Protection:
- Vendor: "Intel Inc."
- Renderer: "Intel Iris OpenGL Engine"
- WebGL version: "WebKit WebGL"
- Numeric parameters: +0.0001 random noise

### 17. Audio Randomization

AudioContext creates fingerprints from audio processing.

Protection:
- Channel data: ±0.00005 noise
- Frequency data: ±0.05 noise
- Imperceptible to humans

### 18. Font Enumeration Restriction

Installed fonts create unique fingerprints.

Protection: Only reports 9 standard fonts (Arial, Courier New, Georgia, Times New Roman, Trebuchet MS, Verdana, serif, sans-serif, monospace)

## Summary

| Layer | Protection | Status |
|-------|-----------|--------|
| 1 | Hardware Spoofing | Active |
| 2 | Screen Normalization | Active |
| 3 | Timezone Normalization | Active |
| 4 | Battery API Blocking | Blocked |
| 5 | WebRTC Blocking | Blocked |
| 6 | Media Devices Blocking | Blocked |
| 7 | Geolocation Blocking | Blocked |
| 8 | Network Info Spoofing | Active |
| 9 | Storage Quota Spoofing | Active |
| 10 | Permissions Hardening | Active |
| 11 | Credentials Blocking | Blocked |
| 12 | Privacy Headers | Injected |
| 13 | Tracking Domains (30+) | Blocked |
| 14 | User Agent Normalization | Active |
| 15 | Canvas Noise | Active |
| 16 | WebGL Spoofing | Active |
| 17 | Audio Randomization | Active |
| 18 | Font Restriction | Active |

## Data Collection

Calm collects zero data:
- No browsing history
- No search queries
- No cookies stored
- No form data
- No passwords
- No telemetry
- No crash reports
- No usage statistics
- No analytics
- No network requests to external servers

## Limitations

Browser-level privacy has inherent limits:

1. Network-level tracking
   - ISPs can see domains visited - Use VPN/Tor
   - DNS queries leak browsing - Use encrypted DNS (DoH/DoT)
   - IP address reveals location - Use VPN/proxy

2. First-party tracking
   - Websites track within their domain
   - Login-based tracking (logging in breaks anonymity)
   - Behavioral patterns within single site

3. Advanced fingerprinting
   - Combination attacks using unblocked APIs
   - Server-side fingerprinting (TLS, TCP/IP stack)
   - Timing attacks

4. Social engineering
   - Phishing and fake sites

## Known Compatibility Issues

Some features will not work:

1. WebRTC-dependent sites (Zoom, Google Meet, Discord video)
2. Geolocation-dependent apps
3. Canvas/WebGL-intensive apps may show noise artifacts
4. Font-dependent design tools see limited fonts

Trade-off: Privacy involves convenience sacrifices.

## Best Practices

Combine Calm with defense-in-depth:

### Network Layer
- Use VPN (Mullvad, ProtonVPN, IVPN)
- Use Tor for high anonymity needs
- Encrypted DNS (Cloudflare 1.1.1.1, Quad9)

### Behavior
- Don't login to accounts during anonymous browsing
- Use Calm for anonymous browsing, other browsers for logged-in sessions
- Assume any login ends anonymity
- Avoid downloads that link to your identity

### Operational Security
- Different browser = different identity
- Keep Calm updated
- Test protections periodically
- Maximum 20 tabs enforced (prevents memory exhaustion)

## Testing

Test your privacy:

1. Fingerprinting tests
   - browserleaks.com
   - amiunique.org
   - coveryourtracks.eff.org
   - Fingerprints should differ between sessions

2. WebRTC leak test
   - browserleaks.com/webrtc
   - Should show no WebRTC detected

3. Canvas fingerprinting
   - browserleaks.com/canvas
   - Hash should change on each page load

4. Privacy headers
   - DevTools → Network
   - Should see DNT: 1, Sec-GPC: 1

5. Hardware fingerprinting
   - browserleaks.com/javascript
   - Should show: 4 cores, 8GB RAM, Win32, 1920x1080

Expected results:
- Low uniqueness score
- Different fingerprint each session
- Tracking blocked
- No IP leaks (with VPN)

## Technical Implementation

Source code:
- `src/privacy/scripts.rs` - JavaScript protection layers
- `src/privacy/mod.rs` - Privacy coordination
- `src/tabs/manager.rs` - WebView configuration (max 20 tabs)
- `src/tabs/tab.rs` - Media and cache cleanup on close

Injection: Scripts run via `with_initialization_script()` before page JavaScript executes.

## Threat Model

Protects against:
- Third-party advertisers and trackers
- Fingerprinting scripts
- Cross-site tracking
- IP leaks via WebRTC
- Browser fingerprinting services

Does NOT protect against:
- Nation-state adversaries
- Network-level surveillance (use Tor)
- Endpoint compromise (malware)
- Social engineering
- Physical access

Use cases: Privacy-conscious browsing, avoiding corporate surveillance, researching sensitive topics.

## Comparison

| Feature | Calm | Tor Browser | Brave | Firefox |
|---------|------|-------------|-------|---------|
| Fingerprint Protection | Strong | Strongest | Strong | Moderate |
| Tracker Blocking | Strong | Strong | Strong | Moderate |
| WebRTC Blocking | Yes | Yes | Optional | Optional |
| Canvas Noise | Yes | Yes | Yes | No |
| WebGL Noise | Yes | Yes | Partial | No |
| Audio Noise | Yes | Yes | No | No |
| Zero Telemetry | Yes | Yes | Partial | No |

Position: Privacy-focused browser with comprehensive protections and native performance.

## Updates

Tracking domains based on:
- EasyPrivacy filter lists
- uBlock Origin filters
- Manual research

Fingerprinting protections based on:
- Academic research
- Creep.js detections
- FingerprintJS techniques

## Contributing

Contributions welcome:
1. Report unblocked trackers
2. Suggest new defenses
3. Test and report bypasses
4. Submit tracking domains

## Disclaimer

Calm provides strong privacy protections but is not a complete anonymity solution.

- Combine with VPN/Tor for network anonymity
- No browser provides perfect protection
- Your behavior matters as much as technology
- For maximum anonymity, use Tor Browser

Calm is for privacy-conscious users who want strong protection with normal browsing performance. Not designed for protection against nation-state adversaries.
