pub fn get_script() -> &'static str {
    r#"
        (function() {
            window.__calmDownloadFilenames = new Map();

            const originalCreateElement = document.createElement.bind(document);
            document.createElement = function(tagName) {
                const element = originalCreateElement(tagName);

                if (tagName.toLowerCase() === 'a') {
                    const originalSetAttribute = element.setAttribute.bind(element);
                    element.setAttribute = function(name, value) {
                        if (name.toLowerCase() === 'download' && value) {
                            const href = element.getAttribute('href');
                            if (href && href.startsWith('blob:')) {
                                console.log('[Download Intercept] Captured filename:', value, 'for blob:', href);
                                window.__calmDownloadFilenames.set(href, value);
                            }
                        }
                        return originalSetAttribute(name, value);
                    };

                    Object.defineProperty(element, 'download', {
                        get: function() {
                            return element.getAttribute('download');
                        },
                        set: function(value) {
                            if (value) {
                                const href = element.getAttribute('href') || element.href;
                                if (href && href.startsWith('blob:')) {
                                    console.log('[Download Intercept] Captured filename via property:', value, 'for blob:', href);
                                    window.__calmDownloadFilenames.set(href, value);
                                }
                            }
                            element.setAttribute('download', value);
                        }
                    });
                }

                return element;
            };

            window.getCalmFilenameForBlob = function(blobUrl) {
                const filename = window.__calmDownloadFilenames.get(blobUrl);
                console.log('[Download Intercept] Looking up filename for:', blobUrl, '=> ', filename);
                return filename || null;
            };

            console.log('[Download Intercept] Filename capture initialized');
        })();
    "#
}
