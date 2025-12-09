pub fn get_adblock_script() -> &'static str {
    r#"
(function() {
    'use strict';

    const AD_DOMAINS = new Set([
        'doubleclick.net', 'googleadservices.com', 'googlesyndication.com',
        'google-analytics.com', 'googletagmanager.com', 'googletagservices.com',
        'adservice.google.com', 'pagead2.googlesyndication.com',
        'ads.youtube.com', 'ads-api.twitter.com', 'ads.facebook.com',
        'facebook.com/tr', 'connect.facebook.net', 'static.ads-twitter.com',
        'analytics.twitter.com', 'pixel.twitter.com',
        'scorecardresearch.com', 'sb.scorecardresearch.com',
        'outbrain.com', 'widgets.outbrain.com', 'taboola.com',
        'cdn.taboola.com', 'adnxs.com', 'ib.adnxs.com',
        'criteo.com', 'static.criteo.net', 'pubmatic.com',
        'ads.pubmatic.com', 'rubiconproject.com', 'fastclick.net',
        'casalemedia.com', 'openx.net', 'amazon-adsystem.com',
        'media.net', 'advertising.com', 'quantserve.com',
        'adroll.com', 'advertising.apple.com', 'iadsdk.apple.com',
        'metrics.apple.com', 'metrics.icloud.com',
        'pinterest.com/ct', 'log.pinterest.com', 'ads.pinterest.com',
        'ads.linkedin.com', 'snap.licdn.com', 'px.ads.linkedin.com',
        'bat.bing.com', 'ads.microsoft.com', 'adnxs.com',
        'pubads.g.doubleclick.net', 'tpc.googlesyndication.com',
        'pagead.l.google.com', 'partner.googleadservices.com',
        'adclick.g.doubleclick.net', 'stats.g.doubleclick.net',
        'ad.doubleclick.net', 'cm.g.doubleclick.net',
        'mediavisor.doubleclick.net', 'ade.googlesyndication.com',
        'www.googleadservices.com', 'www.google-analytics.com',
        'ssl.google-analytics.com', 'www.googletagmanager.com',
        'www.googletagservices.com', 'adservice.google.com',
        'ads.yahoo.com', 'analytics.yahoo.com', 'geo.yahoo.com',
        'udc.yahoo.com', 'udcm.yahoo.com', 'advertising.yahoo.com',
        'analytics.query.yahoo.com', 'partnerads.ysm.yahoo.com',
        'log.fc.yahoo.com', 's.yimg.com', 'analytics.cdn-apple.com',
        'xp.apple.com', 'supportmetrics.apple.com',
        'securemetrics.apple.com', 'metrics.mzstatic.com',
        'marketingtools.apple.com', 'metrics-dra.apple.com',
        'p-cdn.adobe.io', 'assets.adobedtm.com', 'dpm.demdex.net',
        'everesttech.net', 'omtrdc.net', 'adobedtm.com',
        'chartbeat.com', 'chartbeat.net', 'static.chartbeat.com',
        'hotjar.com', 'vars.hotjar.com', 'script.hotjar.com',
        'mouseflow.com', 'cdn-test.mouseflow.com',
        'o2.mouseflow.com', 'a.mouseflow.com',
        'crazyegg.com', 'script.crazyegg.com', 'dnn506yrbagrg.cloudfront.net',
        'heapanalytics.com', 'cdn.heapanalytics.com', 'heap-cdn.heapanalytics.com',
        'fullstory.com', 'rs.fullstory.com', 'edge.fullstory.com',
        'mixpanel.com', 'cdn.mxpnl.com', 'api-js.mixpanel.com',
        'segment.com', 'cdn.segment.com', 'api.segment.io',
        'amplitude.com', 'api.amplitude.com', 'cdn.amplitude.com',
        'zopim.com', 'v2.zopim.com', 'zendesk.com',
        'static.zdassets.com', 'assets.zendesk.com', 'ekr.zdassets.com',
        'intercom.io', 'widget.intercom.io', 'js.intercomcdn.com',
        'drift.com', 'js.driftt.com', 'customer.io',
        'track.customer.io', 'assets.customer.io',
        'clarity.ms', 'www.clarity.ms', 't.clarity.ms',
        'reddit.com/api/v1/ads', 'redditmedia.com/ads',
        'alb.reddit.com', 'events.redditmedia.com',
        'adcolony.com', 'ads.adcolony.com', 'events3alt.adcolony.com',
        'appnexus.com', 'secure.adnxs.com', 'ib.adnxs.com',
        'bidswitch.net', 'x.bidswitch.net', 'rtb.bidswitch.net',
        'smartadserver.com', 'ww251.smartadserver.com', 'diff.smartadserver.com',
        'yieldmo.com', 'static.yieldmo.com', 'ads.yieldmo.com',
        'sharethrough.com', 'btloader.com', 'native.sharethrough.com',
        'indexww.com', 'js-sec.indexww.com', 'as-sec.casalemedia.com',
        'sovrn.com', 'ap.lijit.com', 'cdn.lijit.com',
        'triplelift.com', 'tlx.3lift.com', 'eb2.3lift.com',
        'teads.tv', 'cdn.teads.tv', 's8t.teads.tv',
        'moatads.com', 'z.moatads.com', 'px.moatads.com',
        'adsrvr.org', 'insight.adsrvr.org', 'match.adsrvr.org',
        'adform.net', 'track.adform.net', 'cm.adform.net',
        'onetrust.com', 'cdn.cookielaw.org', 'geolocation.onetrust.com',
        'monetate.net', 'se.monetate.net', 'api.monetate.net',
        'bazaarvoice.com', 'display.ugc.bazaarvoice.com',
        'sentry.io', 'browser.sentry-cdn.com', 'sentry-cdn.com',
        'bugsnag.com', 'd2wy8f7a9ursnm.cloudfront.net',
        'newrelic.com', 'js-agent.newrelic.com', 'bam.nr-data.net',
        'tealiumiq.com', 'tags.tiqcdn.com', 'collect.tealiumiq.com',
        'krxd.net', 'cdn.krxd.net', 'beacon.krxd.net',
        'bluekai.com', 'tags.bluekai.com', 'stags.bluekai.com',
        'rlcdn.com', 'ib.rlcdn.com', 's.rlcdn.com',
        'serving-sys.com', 'bs.serving-sys.com', 'ds.serving-sys.com',
        'adtech.de', 'adserver.adtech.de', 'aka-cdn.adtech.de',
        'adobetag.com', 'nexus.ensighten.com', 'adobedtm.com',
        '2mdn.net', 'g.2mdn.net', 's0.2mdn.net',
        'mookie1.com', 'server.mookie1.com', 'sync.mookie1.com',
        'eyeota.net', 'ps.eyeota.net', 'match.eyeota.net',
        'skimresources.com', 's.skimresources.com', 'r.skimresources.com',
        'viglink.com', 'cdn.viglink.com', 'api.viglink.com',
        'impactradius-event.com', 'impactradius-go.com',
        'avantlink.com', 'www.avantlink.com', 'classic.avantlink.com',
        'linksynergy.com', 'click.linksynergy.com',
        'cj.com', 'www.emjcd.com', 'www.awin1.com',
        'awin1.com', 'www.dwin1.com', 'dwin1.com',
        'commission-junction.com', 'www.tkqlhce.com',
        'pepperjam.com', 'www.pepperjamnetwork.com',
        'adtraction.com', 'www.adtraction.com',
        'tradedoubler.com', 'clk.tradedoubler.com', 'imp.tradedoubler.com',
        'webgains.com', 'track.webgains.com',
        'optimizely.com', 'cdn.optimizely.com', 'logx.optimizely.com',
        'vwo.com', 'dev.visualwebsiteoptimizer.com',
        'ab-tasty.com', 'try.abtasty.com',
        'convertexperiments.com', 'cdn-3.convertexperiments.com',
        'launchdarkly.com', 'app.launchdarkly.com',
        'instana.com', 'eum.instana.io',
        'elastic.co', 'rum-http-intake.logs.datadoghq.com',
        'datadoghq.com', 'browser-intake-datadoghq.com',
        'loggly.com', 'logs-01.loggly.com',
        'splunk.com', 'http-inputs-splunk.com',
        'sumo.com', 'collectors.sumologic.com',
        'braze.com', 'sdk.iad-01.braze.com', 'sdk.iad-03.braze.com',
        'appsflyer.com', 't.appsflyer.com', 'launches.appsflyer.com',
        'adjust.com', 'app.adjust.com', 'view.adjust.com',
        'kochava.com', 'control.kochava.com',
        'branch.io', 'api.branch.io', 'cdn.branch.io',
        'singular.net', 's2s.singular.net',
        'tiktokcdn.com', 'analytics.tiktok.com', 'ads.tiktok.com',
        'tiktokv.com', 'analytics-sg.tiktok.com',
        'snapchat.com/px', 'tr.snapchat.com', 'sc-static.net',
        'pinterest.com/v3', 'ct.pinterest.com', 'widgets.pinterest.com',
        'instagram.com/logging', 'i.instagram.com/api/v1/ads',
        'linkedin.com/px', 'dc.ads.linkedin.com',
        'twitter.com/i/adsct', 'static.ads-twitter.com', 't.co/i/adsct',
        'youtube.com/api/stats/ads', 'youtube.com/ptracking', 'youtube.com/api/stats/atr',
        'youtube.com/pagead', 'youtube.com/get_midroll_info', 'youtube.com/ad_data_204'
    ]);

    const AD_PATTERNS = [
        /\/ads?\//i, /\/advert/i, /\/banner/i, /\/sponsor/i,
        /\/track/i, /\/beacon/i, /\/pixel/i, /analytics/i,
        /\/tag\.js/i, /\/gtag/i, /\/ga\.js/i, /\/fbevents/i,
        /doubleclick/i, /adsystem/i, /adserver/i, /adservice/i,
        /pagead/i, /sponsored/i, /promotion/i, /outbrain/i,
        /taboola/i, /criteo/i, /adnxs/i, /\.ads\./i,
        /metrics/i, /telemetry/i, /tracking/i, /collector/i,
        /\/affiliate/i, /\/conversion/i, /\/impressions?/i, /\/clicks?/i
    ];

    const YOUTUBE_AD_SELECTORS = [
        '.video-ads', '.ytp-ad-module', '.ytp-ad-overlay-container',
        '.ytp-ad-text-overlay', '.ytp-ad-player-overlay',
        '.ytp-ad-progress-list', '.ytp-ad-image-overlay',
        '.ytp-ad-skip-button-container', '.ad-container',
        '.ad-showing', '.ytp-ad-persistent-progress-bar-container',
        'ytd-promoted-sparkles-web-renderer', 'ytd-compact-promoted-video-renderer',
        'ytd-promoted-video-renderer', 'ytd-ad-slot-renderer',
        'ytd-banner-promo-renderer', 'ytd-display-ad-renderer',
        'ytd-statement-banner-renderer', 'ytd-in-feed-ad-layout-renderer',
        'ytd-player-legacy-desktop-watch-ads-renderer',
        '#masthead-ad', '.ytd-action-companion-ad-renderer',
        'ytd-merch-shelf-renderer', 'tp-yt-paper-dialog:has(#feedback)',
        '[id^="player-ads"]', '.ytd-promoted-sparkles-text-search-renderer',
        'ytd-ad-slot-renderer', 'yt-mealbar-promo-renderer',
        '#player-ads', 'ytd-engagement-panel-section-list-renderer[target-id="engagement-panel-ads"]',
        '.ytd-compact-promoted-item-renderer', '.ytd-promoted-video-inline-renderer',
        'ytd-video-masthead-ad-v3-renderer', 'ytd-primetime-promo-renderer',
        '#merch-shelf', 'ytd-companion-slot-renderer',
        'ytd-action-companion-ad-renderer', 'ytd-watch-next-secondary-results-renderer > #items > ytd-compact-promoted-video-renderer'
    ];

    const GENERIC_AD_SELECTORS = [
        '[id*="ad-"]', '[class*="ad-"]', '[id*="ads-"]', '[class*="ads-"]',
        '[id*="banner"]', '[class*="banner"]', '[id*="sponsor"]', '[class*="sponsor"]',
        '[id*="promo"]', '[class*="promo"]', '[aria-label*="Advertisement"]',
        '[data-ad]', '[data-ads]', 'iframe[src*="doubleclick"]',
        'iframe[src*="googlesyndication"]', 'iframe[src*="/ads/"]',
        'div[data-google-query-id]', 'ins.adsbygoogle',
        '.advertisement', '.advertising', '#ad', '#ads',
        '.ad-container', '.ads-container', '.ad-wrapper', '.ads-wrapper',
        '[id*="google_ads"]', '[class*="google_ads"]',
        '[id*="taboola"]', '[class*="taboola"]',
        '[id*="outbrain"]', '[class*="outbrain"]',
        '.native-ad', '.sponsored-content', '.promoted-content',
        '[data-ad-slot]', '[data-ad-unit]', '[data-dfp-id]'
    ];

    const ANTI_ADBLOCK_SELECTORS = [
        '[id*="adblock"]', '[class*="adblock"]',
        '[id*="ad-block"]', '[class*="ad-block"]',
        '[id*="adblocker"]', '[class*="adblocker"]',
        'div[style*="blur"]', '.adblock-modal', '.adblock-popup',
        '.adblock-overlay', '#adblock-notification',
        'tp-yt-paper-dialog', 'ytd-popup-container'
    ];

    function isAdUrl(url) {
        try {
            const urlObj = new URL(url, window.location.href);
            const hostname = urlObj.hostname.toLowerCase();
            const pathname = urlObj.pathname.toLowerCase();
            const fullUrl = urlObj.href.toLowerCase();

            if (AD_DOMAINS.has(hostname)) return true;

            for (const domain of AD_DOMAINS) {
                if (hostname.endsWith('.' + domain) || hostname === domain) {
                    return true;
                }
            }

            for (const pattern of AD_PATTERNS) {
                if (pattern.test(pathname) || pattern.test(fullUrl)) {
                    return true;
                }
            }

            return false;
        } catch (e) {
            return false;
        }
    }

    function pruneAdData(obj) {
        if (!obj || typeof obj !== 'object') return obj;

        if (Array.isArray(obj)) {
            return obj.map(pruneAdData);
        }

        const keys = ['adPlacements', 'adSlots', 'playerAds', 'adBreakParams',
                      'adPlacementRenderer', 'adSlotRenderer', 'displayAdRenderer',
                      'adInfo', 'adVideoId', 'companionAd'];

        for (const key of keys) {
            if (key in obj) {
                delete obj[key];
            }
        }

        for (const key in obj) {
            if (obj.hasOwnProperty(key)) {
                obj[key] = pruneAdData(obj[key]);
            }
        }

        return obj;
    }

    const originalFetch = window.fetch;
    window.fetch = function(...args) {
        const url = args[0];
        const urlString = typeof url === 'string' ? url : url?.url || '';

        if (isAdUrl(urlString)) {
            return Promise.reject(new Error('Blocked by Calm'));
        }

        return originalFetch.apply(this, args).then(response => {
            if (urlString.includes('youtube.com/youtubei/v1/') ||
                urlString.includes('/player') ||
                urlString.includes('/next') ||
                urlString.includes('/get_video_info')) {

                return response.clone().text().then(text => {
                    try {
                        let data = JSON.parse(text);
                        data = pruneAdData(data);

                        const modifiedText = JSON.stringify(data)
                            .replace(/"adPlacements":/g, '"no_ads":')
                            .replace(/"adSlots":/g, '"no_ads":')
                            .replace(/"playerAds":/g, '"no_ads":');

                        return new Response(modifiedText, {
                            status: response.status,
                            statusText: response.statusText,
                            headers: response.headers
                        });
                    } catch (e) {
                        return response;
                    }
                });
            }

            return response;
        });
    };

    const OriginalXHR = window.XMLHttpRequest;
    window.XMLHttpRequest = function() {
        const xhr = new OriginalXHR();
        const originalOpen = xhr.open;
        const originalSend = xhr.send;

        xhr.open = function(method, url, ...rest) {
            this._url = url;

            if (isAdUrl(url)) {
                return;
            }
            return originalOpen.call(this, method, url, ...rest);
        };

        xhr.send = function(...args) {
            if (this._url && (this._url.includes('youtube.com/youtubei/v1/') ||
                              this._url.includes('/player') ||
                              this._url.includes('/get_video_info'))) {

                const originalOnLoad = this.onload;
                const originalOnReadyStateChange = this.onreadystatechange;

                this.onreadystatechange = function() {
                    if (this.readyState === 4 && this.status === 200) {
                        try {
                            const responseText = this.responseText;
                            let data = JSON.parse(responseText);
                            data = pruneAdData(data);

                            Object.defineProperty(this, 'responseText', {
                                writable: true,
                                value: JSON.stringify(data)
                            });
                            Object.defineProperty(this, 'response', {
                                writable: true,
                                value: JSON.stringify(data)
                            });
                        } catch (e) {}
                    }

                    if (originalOnReadyStateChange) {
                        return originalOnReadyStateChange.apply(this, arguments);
                    }
                };

                if (originalOnLoad) {
                    this.onload = function() {
                        try {
                            const responseText = this.responseText;
                            let data = JSON.parse(responseText);
                            data = pruneAdData(data);

                            Object.defineProperty(this, 'responseText', {
                                writable: true,
                                value: JSON.stringify(data)
                            });
                        } catch (e) {}
                        return originalOnLoad.apply(this, arguments);
                    };
                }
            }

            return originalSend.apply(this, args);
        };

        return xhr;
    };
    window.XMLHttpRequest.prototype = OriginalXHR.prototype;

    const originalCreateElement = document.createElement;
    document.createElement = function(tagName, options) {
        const element = originalCreateElement.call(document, tagName, options);

        if (tagName.toLowerCase() === 'script' || tagName.toLowerCase() === 'iframe' ||
            tagName.toLowerCase() === 'img' || tagName.toLowerCase() === 'link') {

            const observeElement = (elem) => {
                const srcDescriptor = Object.getOwnPropertyDescriptor(elem.constructor.prototype, 'src');
                const hrefDescriptor = Object.getOwnPropertyDescriptor(elem.constructor.prototype, 'href');

                if (srcDescriptor) {
                    Object.defineProperty(elem, 'src', {
                        get: srcDescriptor.get,
                        set: function(value) {
                            if (isAdUrl(value)) {
                                return;
                            }
                            srcDescriptor.set.call(this, value);
                        },
                        configurable: true
                    });
                }

                if (hrefDescriptor && tagName.toLowerCase() === 'link') {
                    Object.defineProperty(elem, 'href', {
                        get: hrefDescriptor.get,
                        set: function(value) {
                            if (isAdUrl(value)) {
                                return;
                            }
                            hrefDescriptor.set.call(this, value);
                        },
                        configurable: true
                    });
                }
            };

            observeElement(element);
        }

        return element;
    };

    function hideYouTubeAds() {
        try {
            const player = document.querySelector('.html5-video-player');
            if (player && player.classList.contains('ad-showing')) {
                player.classList.remove('ad-showing');
                player.classList.add('ad-interrupting');

                const skipButton = document.querySelector('.ytp-ad-skip-button, .ytp-skip-ad-button, .ytp-ad-skip-button-modern');
                if (skipButton) {
                    skipButton.click();
                }

                const video = document.querySelector('video.html5-main-video');
                if (video) {
                    if (video.duration && video.currentTime < video.duration - 0.5) {
                        video.currentTime = video.duration - 0.1;
                    }
                    video.playbackRate = 16;
                    video.muted = true;
                }
            }

            if (window.ytInitialPlayerResponse) {
                pruneAdData(window.ytInitialPlayerResponse);
            }

            if (window.ytInitialData) {
                pruneAdData(window.ytInitialData);
            }

            const ytcfg = window.ytcfg;
            if (ytcfg && typeof ytcfg.set === 'function') {
                try {
                    const playerVars = ytcfg.get('PLAYER_VARS') || {};
                    playerVars.ad_slots = 0;
                    playerVars.ad3_module = 0;
                    ytcfg.set('PLAYER_VARS', playerVars);
                } catch (e) {}
            }

        } catch (e) {}
    }

    function processShadowRoots(root) {
        try {
            const walker = document.createTreeWalker(
                root,
                NodeFilter.SHOW_ELEMENT,
                null,
                false
            );

            let node;
            while (node = walker.nextNode()) {
                if (node.shadowRoot) {
                    hideAdsInTree(node.shadowRoot);
                    processShadowRoots(node.shadowRoot);
                }
            }
        } catch (e) {}
    }

    function hideAdsInTree(root) {
        const allSelectors = [...YOUTUBE_AD_SELECTORS, ...GENERIC_AD_SELECTORS];

        for (const selector of allSelectors) {
            try {
                const elements = root.querySelectorAll(selector);
                for (const el of elements) {
                    if (el && el.parentNode && !el.hasAttribute('data-calm-hidden')) {
                        el.style.cssText = 'display: none !important; visibility: hidden !important;';
                        el.setAttribute('data-calm-hidden', 'true');

                        requestAnimationFrame(() => {
                            if (el.parentNode) {
                                el.remove();
                            }
                        });
                    }
                }
            } catch (e) {}
        }
    }

    function hideAds() {
        const isYouTube = window.location.hostname.includes('youtube.com');

        if (isYouTube) {
            hideYouTubeAds();
            hideAdsInTree(document);
            processShadowRoots(document.body || document.documentElement);
        } else {
            hideAdsInTree(document);
        }

        for (const selector of ANTI_ADBLOCK_SELECTORS) {
            try {
                const elements = document.querySelectorAll(selector);
                for (const el of elements) {
                    if (el && el.parentNode) {
                        const text = (el.textContent || '').toLowerCase();
                        if (text.includes('ad') && (text.includes('block') || text.includes('detect'))) {
                            el.style.cssText = 'display: none !important;';
                            requestAnimationFrame(() => el.remove());
                        }
                    }
                }
            } catch (e) {}
        }
    }

    function injectCSS() {
        if (document.getElementById('calm-adblock-styles')) return;

        const style = document.createElement('style');
        style.id = 'calm-adblock-styles';
        style.textContent = `
            .video-ads, .ytp-ad-module, .ytp-ad-overlay-container,
            .ytp-ad-text-overlay, .ytp-ad-player-overlay,
            ytd-promoted-sparkles-web-renderer, ytd-compact-promoted-video-renderer,
            ytd-promoted-video-renderer, ytd-ad-slot-renderer,
            ytd-display-ad-renderer, ytd-player-legacy-desktop-watch-ads-renderer,
            ytd-engagement-panel-section-list-renderer[target-id="engagement-panel-ads"],
            ytd-companion-slot-renderer, ytd-action-companion-ad-renderer,
            [id*="google_ads"], [class*="google_ads"],
            ins.adsbygoogle, .adsbygoogle,
            iframe[src*="doubleclick"], iframe[src*="googlesyndication"],
            iframe[src*="/ads/"], iframe[src*="advertising"],
            [id*="taboola"], [class*="taboola"],
            [id*="outbrain"], [class*="outbrain"],
            .native-ad, .sponsored-content, .promoted-content,
            [data-ad-slot], [data-ad-unit], [data-dfp-id],
            tp-yt-paper-dialog:has(yt-mealbar-promo-renderer),
            ytd-popup-container:has([style*="adblock"]) {
                display: none !important;
                visibility: hidden !important;
                opacity: 0 !important;
                width: 0 !important;
                height: 0 !important;
                position: absolute !important;
                top: -9999px !important;
                left: -9999px !important;
                pointer-events: none !important;
                z-index: -1 !important;
            }

            .html5-video-player.ad-showing video,
            .html5-video-player.ad-interrupting video {
                display: block !important;
            }
        `;
        (document.head || document.documentElement).appendChild(style);
    }

    if (window.location.hostname.includes('youtube.com')) {
        const origStringify = JSON.stringify;
        JSON.stringify = function(obj, ...args) {
            if (obj && typeof obj === 'object') {
                obj = pruneAdData(obj);
            }
            return origStringify.call(this, obj, ...args);
        };

        const origParse = JSON.parse;
        JSON.parse = function(text, ...args) {
            const result = origParse.call(this, text, ...args);
            if (result && typeof result === 'object') {
                return pruneAdData(result);
            }
            return result;
        };

        const ytInitialPlayerResponseDesc = Object.getOwnPropertyDescriptor(window, 'ytInitialPlayerResponse');
        if (!ytInitialPlayerResponseDesc || ytInitialPlayerResponseDesc.configurable) {
            let _ytInitialPlayerResponse;
            Object.defineProperty(window, 'ytInitialPlayerResponse', {
                get: function() {
                    return _ytInitialPlayerResponse;
                },
                set: function(val) {
                    _ytInitialPlayerResponse = pruneAdData(val);
                },
                configurable: false
            });
        }

        const ytInitialDataDesc = Object.getOwnPropertyDescriptor(window, 'ytInitialData');
        if (!ytInitialDataDesc || ytInitialDataDesc.configurable) {
            let _ytInitialData;
            Object.defineProperty(window, 'ytInitialData', {
                get: function() {
                    return _ytInitialData;
                },
                set: function(val) {
                    _ytInitialData = pruneAdData(val);
                },
                configurable: false
            });
        }

        const originalDefineProperty = Object.defineProperty;
        Object.defineProperty = function(obj, prop, descriptor) {
            if ((prop === 'adPlacements' || prop === 'adSlots' || prop === 'playerAds') && descriptor) {
                return obj;
            }
            return originalDefineProperty.call(this, obj, prop, descriptor);
        };
    }

    Object.defineProperty(navigator, 'webdriver', {
        get: () => undefined,
        configurable: false
    });

    const getParameter = window.URLSearchParams.prototype.get;
    window.URLSearchParams.prototype.get = function(name) {
        if (name === 'ad' || name === 'ads' || name === 'ad_id' || name === 'adurl') {
            return null;
        }
        return getParameter.apply(this, arguments);
    };

    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', () => {
            injectCSS();
            hideAds();
        });
    } else {
        injectCSS();
        hideAds();
    }

    let rafScheduled = false;
    const observer = new MutationObserver((mutations) => {
        if (rafScheduled) return;

        let shouldProcess = false;
        for (const mutation of mutations) {
            if (mutation.addedNodes.length > 0) {
                shouldProcess = true;
                break;
            }
        }

        if (shouldProcess) {
            rafScheduled = true;
            requestAnimationFrame(() => {
                hideAds();
                rafScheduled = false;
            });
        }
    });

    if (document.body) {
        observer.observe(document.body, {
            childList: true,
            subtree: true
        });
    } else {
        document.addEventListener('DOMContentLoaded', () => {
            observer.observe(document.body, {
                childList: true,
                subtree: true
            });
        });
    }

    setInterval(hideAds, 1500);

    console.log('[Calm] Ultimate Adblock 2025 - uBlock Origin Enhanced');
})();
"#
}
