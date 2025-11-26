pub fn get_title_tracker_script() -> &'static str {
    r#"
        (function() {
            let lastTitle = '';
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
                document.addEventListener('DOMContentLoaded', updateTitle);
            } else {
                updateTitle();
            }

            window.addEventListener('load', () => {
                updateTitle();
                updateNavigationState();
            });

            window.addEventListener('beforeunload', () => {
                updateTitle();
            });

            window.addEventListener('popstate', (event) => {
                setTimeout(() => {
                    updateTitle();
                    updateNavigationState();
                }, 0);
                setTimeout(updateTitle, 50);
                setTimeout(updateTitle, 200);
            });

            window.addEventListener('pageshow', (event) => {
                updateTitle();
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
                updateTitle();
            };

            history.replaceState = function() {
                originalReplaceState.apply(this, arguments);
                updateNavigationState();
                updateTitle();
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

            const titleObserver = new MutationObserver(updateTitle);

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
                            updateTitle();
                        }
                    });
                });
            });

            if (document.head) {
                headObserver.observe(document.head, { childList: true });
            }

            setInterval(updateTitle, 1000);
            updateNavigationState();
        })();
    "#
}
