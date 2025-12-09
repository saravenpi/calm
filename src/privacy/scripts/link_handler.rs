pub fn get_link_handler_script() -> &'static str {
    r#"
        (function() {
            console.log('[LINK HANDLER] Initializing link handler script');

            function findLinkElement(target) {
                while (target && target !== document) {
                    if (target.tagName === 'A' && target.href) {
                        return target;
                    }
                    target = target.parentElement;
                }
                return null;
            }

            function shouldOpenInNewTab(e, link) {
                if (!link) return false;

                const targetAttr = link.getAttribute('target');
                if (targetAttr === '_blank' || targetAttr === '_new') {
                    return true;
                }

                if (e.button === 1) {
                    return true;
                }

                if ((e.ctrlKey || e.metaKey) && e.button === 0) {
                    return true;
                }

                return false;
            }

            function handleLinkClick(e) {
                const link = findLinkElement(e.target);

                if (!link || !link.href) return;

                if (shouldOpenInNewTab(e, link)) {
                    if (window.ipc) {
                        e.preventDefault();
                        e.stopPropagation();

                        try {
                            const absoluteUrl = new URL(link.href, window.location.href).href;
                            console.log('[LINK HANDLER] Opening link in new tab:', absoluteUrl);
                            window.ipc.postMessage(JSON.stringify({
                                action: 'open_url_new_tab',
                                url: absoluteUrl
                            }));
                        } catch (err) {
                            console.error('[LINK HANDLER] Failed to parse URL:', err);
                        }
                    }
                }
            }

            document.addEventListener('click', handleLinkClick, true);
            document.addEventListener('auxclick', handleLinkClick, true);

            const originalOpen = window.open;
            const originalAssign = window.location.assign;
            const originalReplace = window.location.replace;

            Object.defineProperty(window, 'open', {
                value: function(url, target, features) {
                    if (!url) return null;

                    try {
                        const absoluteUrl = new URL(url, window.location.href).href;

                        if (window.ipc && (target === '_blank' || target === '_new')) {
                            console.log('[LINK HANDLER] window.open intercepted:', absoluteUrl, 'target:', target);
                            window.ipc.postMessage(JSON.stringify({
                                action: 'open_url_new_tab',
                                url: absoluteUrl
                            }));
                            return null;
                        }
                    } catch (err) {
                        console.error('[LINK HANDLER] Failed to parse URL in window.open:', err);
                    }

                    return originalOpen.call(this, url, target, features);
                },
                writable: false,
                configurable: false
            });

            const formObserver = new MutationObserver(function(mutations) {
                mutations.forEach(function(mutation) {
                    mutation.addedNodes.forEach(function(node) {
                        if (node.nodeType === 1) {
                            if (node.tagName === 'FORM' && node.target === '_blank') {
                                node.addEventListener('submit', function(e) {
                                    if (window.ipc && node.action) {
                                        e.preventDefault();
                                        window.ipc.postMessage(JSON.stringify({
                                            action: 'open_url_new_tab',
                                            url: node.action
                                        }));
                                    }
                                });
                            }

                            const forms = node.querySelectorAll ? node.querySelectorAll('form[target="_blank"]') : [];
                            forms.forEach(function(form) {
                                form.addEventListener('submit', function(e) {
                                    if (window.ipc && form.action) {
                                        e.preventDefault();
                                        window.ipc.postMessage(JSON.stringify({
                                            action: 'open_url_new_tab',
                                            url: form.action
                                        }));
                                    }
                                });
                            });
                        }
                    });
                });
            });

            if (document.body) {
                formObserver.observe(document.body, { childList: true, subtree: true });
            } else {
                document.addEventListener('DOMContentLoaded', function() {
                    formObserver.observe(document.body, { childList: true, subtree: true });
                });
            }

            const existingForms = document.querySelectorAll('form[target="_blank"]');
            existingForms.forEach(function(form) {
                form.addEventListener('submit', function(e) {
                    if (window.ipc && form.action) {
                        e.preventDefault();
                        window.ipc.postMessage(JSON.stringify({
                            action: 'open_url_new_tab',
                            url: form.action
                        }));
                    }
                });
            });

            console.log('[LINK HANDLER] Link handler script initialized successfully');
        })();
    "#
}
