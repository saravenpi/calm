use crate::config::PrivacySettings;

pub fn get_privacy_script(settings: &PrivacySettings) -> String {
    let mut script = String::from("(function() { 'use strict';\n");
    script.push_str("const config = { configurable: false, enumerable: true };\n");

    if settings.hardware_spoofing {
        script.push_str(r#"
        Object.defineProperty(navigator, 'webdriver', { ...config, get: () => false });
        Object.defineProperty(navigator, 'plugins', { ...config, get: () => [] });
        Object.defineProperty(navigator, 'languages', { ...config, get: () => ['en-US', 'en'] });
        Object.defineProperty(navigator, 'hardwareConcurrency', { ...config, get: () => 4 });
        Object.defineProperty(navigator, 'deviceMemory', { ...config, get: () => 8 });
        Object.defineProperty(navigator, 'platform', { ...config, get: () => 'Win32' });
        Object.defineProperty(navigator, 'maxTouchPoints', { ...config, get: () => 0 });
        Object.defineProperty(navigator, 'vendor', { ...config, get: () => 'Google Inc.' });
        "#);
    }

    if settings.network_info_spoofing {
        script.push_str(r#"
        if (navigator.connection) {
            Object.defineProperty(navigator, 'connection', {
                ...config,
                get: () => ({
                    effectiveType: '4g',
                    downlink: 10,
                    rtt: 50,
                    saveData: false,
                    type: 'wifi'
                })
            });
        }
        "#);
    }

    if settings.battery_blocking {
        script.push_str(r#"
        if (navigator.getBattery) {
            navigator.getBattery = () => Promise.reject(new Error('Battery API disabled'));
        }
        "#);
    }

    if settings.webrtc_blocking {
        script.push_str(r#"
        if (window.RTCPeerConnection) {
            const BlockedRTC = function() { throw new Error('WebRTC disabled'); };
            window.RTCPeerConnection = BlockedRTC;
            window.webkitRTCPeerConnection = BlockedRTC;
            window.mozRTCPeerConnection = BlockedRTC;
            window.RTCDataChannel = BlockedRTC;
            window.RTCSessionDescription = BlockedRTC;
            window.RTCIceCandidate = BlockedRTC;
        }
        "#);
    }

    if settings.media_device_blocking {
        script.push_str(r#"
        if (navigator.mediaDevices) {
            navigator.mediaDevices.enumerateDevices = () => Promise.resolve([]);
            navigator.mediaDevices.getUserMedia = () => Promise.reject(new Error('Media access disabled'));
            navigator.mediaDevices.getDisplayMedia = () => Promise.reject(new Error('Screen capture disabled'));
        }
        "#);
    }

    if settings.geolocation_blocking {
        script.push_str(r#"
        if (navigator.geolocation) {
            const blockGeo = () => { throw new Error('Geolocation disabled'); };
            navigator.geolocation.getCurrentPosition = blockGeo;
            navigator.geolocation.watchPosition = blockGeo;
            navigator.geolocation.clearWatch = () => {};
        }
        "#);
    }

    if settings.credentials_blocking {
        script.push_str(r#"
        if (navigator.credentials) {
            navigator.credentials.get = () => Promise.resolve(null);
            navigator.credentials.store = () => Promise.resolve();
            navigator.credentials.create = () => Promise.resolve(null);
        }
        "#);
    }

    if settings.storage_quota_spoofing {
        script.push_str(r#"
        if (navigator.storage?.estimate) {
            navigator.storage.estimate = () => Promise.resolve({ quota: 8589934592, usage: 1073741824 });
        }
        "#);
    }

    if settings.permissions_hardening {
        script.push_str(r#"
        if (navigator.permissions) {
            const originalQuery = navigator.permissions.query;
            navigator.permissions.query = function(descriptor) {
                const blocked = ['geolocation', 'notifications', 'midi', 'camera', 'microphone'];
                if (blocked.includes(descriptor.name)) {
                    return Promise.resolve({ state: 'denied', onchange: null });
                }
                return originalQuery.apply(this, arguments);
            };
        }
        "#);
    }

    if settings.screen_normalization {
        script.push_str(r#"
        Object.defineProperty(screen, 'width', { ...config, get: () => 1920 });
        Object.defineProperty(screen, 'height', { ...config, get: () => 1080 });
        Object.defineProperty(screen, 'availWidth', { ...config, get: () => 1920 });
        Object.defineProperty(screen, 'availHeight', { ...config, get: () => 1040 });
        Object.defineProperty(screen, 'colorDepth', { ...config, get: () => 24 });
        Object.defineProperty(screen, 'pixelDepth', { ...config, get: () => 24 });
        "#);
    }

    if settings.timezone_normalization {
        script.push_str(r#"
        Date.prototype.getTimezoneOffset = function() { return 0; };
        "#);
    }

    script.push_str("\nwindow.__calm_privacy_enabled = true;\n})();\n");
    script
}


pub fn get_dark_mode_preference() -> &'static str {
    r#"
    (function() {
        'use strict';

        const style = document.createElement('style');
        style.textContent = `
            :root {
                color-scheme: dark;
            }
        `;

        if (document.head) {
            document.head.appendChild(style);
        } else {
            document.addEventListener('DOMContentLoaded', function() {
                document.head.appendChild(style);
            });
        }
    })();
    "#
}
