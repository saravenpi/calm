pub fn get_title_tracker_script() -> &'static str {
    r#"
        (function() {
            let lastTitle = '';
            let lastUrl = '';
            let historyLength = window.history.length;
            let historyPosition = window.history.length - 1;

            function updateTitle() {
                const currentTitle = document.title || 'New Tab';
                if (currentTitle !== lastTitle) {
                    lastTitle = currentTitle;
                    if (window.ipc) {
                        window.ipc.postMessage(JSON.stringify({
                            action: 'update_title',
                            title: currentTitle
                        }));
                    }
                }
            }

            function updateUrl() {
                const currentUrl = window.location.href;
                if (currentUrl !== lastUrl) {
                    lastUrl = currentUrl;
                    if (window.ipc) {
                        window.ipc.postMessage(JSON.stringify({
                            action: 'update_url',
                            url: currentUrl
                        }));
                    }
                }
            }

            function updateTitleAndUrl() {
                updateTitle();
                updateUrl();
            }

            function updateNavigationState() {
                if (window.ipc) {
                    const canGoBack = historyPosition > 0;
                    const canGoForward = historyPosition < historyLength - 1;

                    window.ipc.postMessage(JSON.stringify({
                        action: 'update_navigation_state',
                        canGoBack: canGoBack,
                        canGoForward: canGoForward
                    }));
                }
            }

            if (document.readyState === 'loading') {
                document.addEventListener('DOMContentLoaded', updateTitleAndUrl);
            } else {
                updateTitleAndUrl();
            }

            window.addEventListener('load', () => {
                updateTitleAndUrl();
                updateNavigationState();
            });

            window.addEventListener('beforeunload', () => {
                updateTitleAndUrl();
            });

            window.addEventListener('popstate', (event) => {
                setTimeout(() => {
                    updateTitleAndUrl();
                    updateNavigationState();
                }, 0);
                setTimeout(updateTitleAndUrl, 50);
                setTimeout(updateTitleAndUrl, 200);
            });

            window.addEventListener('pageshow', (event) => {
                updateTitleAndUrl();
                updateNavigationState();
            });

            const originalPushState = history.pushState;
            const originalReplaceState = history.replaceState;
            const originalBack = history.back;
            const originalForward = history.forward;

            history.pushState = function() {
                originalPushState.apply(this, arguments);
                historyPosition++;
                historyLength = historyPosition + 1;
                updateNavigationState();
                updateTitleAndUrl();
            };

            history.replaceState = function() {
                originalReplaceState.apply(this, arguments);
                updateNavigationState();
                updateTitleAndUrl();
            };

            history.back = function() {
                if (historyPosition > 0) {
                    historyPosition--;
                }
                updateNavigationState();
                originalBack.apply(this, arguments);
            };

            history.forward = function() {
                if (historyPosition < historyLength - 1) {
                    historyPosition++;
                }
                updateNavigationState();
                originalForward.apply(this, arguments);
            };

            const titleObserver = new MutationObserver(updateTitleAndUrl);

            if (document.querySelector('title')) {
                titleObserver.observe(
                    document.querySelector('title'),
                    { childList: true, characterData: true, subtree: true }
                );
            }

            const headObserver = new MutationObserver((mutations) => {
                mutations.forEach((mutation) => {
                    mutation.addedNodes.forEach((node) => {
                        if (node.nodeName === 'TITLE') {
                            titleObserver.observe(
                                node,
                                { childList: true, characterData: true, subtree: true }
                            );
                            updateTitleAndUrl();
                        }
                    });
                });
            });

            if (document.head) {
                headObserver.observe(document.head, { childList: true });
            }

            setInterval(updateTitleAndUrl, 1000);
            updateNavigationState();
        })();
    "#
}
