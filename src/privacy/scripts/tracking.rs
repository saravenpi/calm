pub fn get_tracking_blocker() -> &'static str {
    r#"
    (function() {
        'use strict';

        const trackingDomains = [
            'google-analytics.com', 'googletagmanager.com', 'doubleclick.net',
            'facebook.com/tr', 'facebook.net', 'connect.facebook.net', 'pixel.facebook.com',
            'analytics.twitter.com', 'ads-twitter.com', 'static.ads-twitter.com',
            'analytics.google.com', 'stats.g.doubleclick.net', 'googlesyndication.com',
            'adservice.google.com', 'pagead2.googlesyndication.com', 'amazon-adsystem.com',
            'scorecardresearch.com', 'newrelic.com', 'hotjar.com', 'mouseflow.com',
            'crazyegg.com', 'luckyorange.com', 'clicktale.com', 'inspectlet.com',
            'quantserve.com', 'mixpanel.com', 'segment.com', 'fullstory.com',
            'heap.io', 'amplitude.com', 'chartbeat.com', 'optimizely.com',
            'tiktok.com/i18n/pixel', 'ads.tiktok.com', 'analytics.tiktok.com',
            'sentry.io', 'bugsnag.com', 'loggly.com', 'splunk.com',
            'clarity.ms', 't.clarity.ms', 'datadoghq.com', 'browser-intake-datadoghq.com',
            'tealiumiq.com', 'tags.tiqcdn.com', 'krxd.net', 'bluekai.com',
            'moatads.com', 'z.moatads.com', 'px.moatads.com',
            'reddit.com/api/v1/ads', 'events.redditmedia.com', 'alb.reddit.com',
            'snapchat.com/px', 'tr.snapchat.com', 'ct.pinterest.com',
            'instagram.com/logging', 'i.instagram.com/api/v1/ads',
            'linkedin.com/px', 'dc.ads.linkedin.com', 'snap.licdn.com',
            'twitter.com/i/adsct', 't.co/i/adsct',
            'youtube.com/ptracking', 'youtube.com/api/stats/atr', 'youtube.com/api/stats/ads',
            'appsflyer.com', 'adjust.com', 'branch.io', 'singular.net', 'kochava.com',
            'braze.com', 'customer.io', 'intercom.io', 'drift.com',
            'zendesk.com', 'zopim.com', 'assets.zendesk.com',
            'sumo.com', 'collectors.sumologic.com', 'instana.io',
            'launchdarkly.com', 'vwo.com', 'ab-tasty.com'
        ];

        const isTracking = (url) => trackingDomains.some(domain => url.includes(domain));

        const originalFetch = window.fetch;
        window.fetch = function(url, options = {}) {
            const urlStr = typeof url === 'string' ? url : url.url;
            if (isTracking(urlStr)) return Promise.reject(new Error('Blocked'));

            const headers = new Headers(options.headers || {});
            headers.set('DNT', '1');
            headers.set('Sec-GPC', '1');
            headers.set('Sec-CH-UA', '"Not_A Brand";v="8", "Chromium";v="131"');
            headers.set('Sec-CH-UA-Mobile', '?0');
            headers.set('Sec-CH-UA-Platform', '"Windows"');
            if (!headers.has('Referer')) headers.delete('Referer');
            options.headers = headers;
            return originalFetch(url, options);
        };

        const originalOpen = XMLHttpRequest.prototype.open;
        XMLHttpRequest.prototype.open = function(method, url) {
            if (isTracking(url)) return;
            return originalOpen.apply(this, arguments);
        };

        const originalSend = XMLHttpRequest.prototype.send;
        XMLHttpRequest.prototype.send = function() {
            try {
                this.setRequestHeader('DNT', '1');
                this.setRequestHeader('Sec-GPC', '1');
            } catch(e) {}
            return originalSend.apply(this, arguments);
        };

        const blockTrackerElement = (child) => {
            if (child.tagName === 'SCRIPT' || child.tagName === 'IFRAME') {
                const src = child.src || '';
                if (isTracking(src)) return true;
            }
            return false;
        };

        const originalAppendChild = Element.prototype.appendChild;
        Element.prototype.appendChild = function(child) {
            if (blockTrackerElement(child)) return child;
            return originalAppendChild.call(this, child);
        };

        const originalInsertBefore = Element.prototype.insertBefore;
        Element.prototype.insertBefore = function(child, reference) {
            if (blockTrackerElement(child)) return child;
            return originalInsertBefore.call(this, child, reference);
        };
    })();
    "#
}
