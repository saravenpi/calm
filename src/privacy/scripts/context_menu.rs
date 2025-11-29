pub fn get_script() -> &'static str {
    r#"
        (function() {
            function initContextMenu() {
                if (!document.getElementById('calm-font-face') && document.head) {
                    const style = document.createElement('style');
                    style.id = 'calm-font-face';
                    style.textContent = `
                        @font-face {
                            font-family: 'gohu';
                            src: url('data:application/font-woff2;charset=utf-8;base64,d09GMgABAAAAABEYAA4AAAAAJKAAABDEAAEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAGhYbHhyBbAZgAIEICghWCYM8EQwKgdRgg7hzC4NAAAE2AiQDhx4EIAWDAAeFPQyBZxu8IqOQHkO2HySh4MbHmL/M3v+TQLAbdqUBAhsOACi1kcDKiixZqgMYVqyqKju0e3//b7Pb1SIRqC5SN6ErCDmT0DSReCdh0kgkn2Dz/P//n3m/+cXsO7PvzO07c+/M3DszJJKS5kkk0kmkk0gkkdwk0kmke+/eJvfuPQe9e5vce/emL7lJokTy3ntPIin33ntP7r33nvz/f5tdYKuqKqmqomor87+ft3e21N7ZN2v2zZp9s2bfrNk3a/bNmn2zZt+s2Tdr9s2afbNm36zZN2v2zZp9s2bfrNk3a/bNmn2zZt+s2Tdr9s2afbNm36zZN2v2zZp9s2bfrNk3a/bNmn2zZt+s2Tdr9s2afbNm36zZN2v2zZp9s2bfrNk3a/bNmn2zZt+s2Tdr9v0HAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA') format('woff2');
                            font-weight: normal;
                            font-style: normal;
                        }
                    `;
                    document.head.appendChild(style);
                }

            let contextMenu = null;
            let currentTarget = null;

            function createContextMenu() {
                const menu = document.createElement('div');
                menu.id = 'calm-context-menu';
                menu.style.cssText = `
                    position: fixed;
                    background: #000000;
                    border: 2px solid #ffffff;
                    color: #ffffff;
                    font-family: 'gohu', monospace;
                    font-size: 11px;
                    z-index: 999999;
                    min-width: 200px;
                    display: none;
                    padding: 0;
                    image-rendering: pixelated;
                    image-rendering: crisp-edges;
                `;
                return menu;
            }

            function createMenuItem(text, onClick) {
                const item = document.createElement('div');
                item.textContent = text;
                item.style.cssText = `
                    padding: 8px;
                    cursor: pointer;
                    user-select: none;
                    border-bottom: 1px solid #333333;
                    background: transparent;
                    transition: background 0.1s linear;
                `;
                item.onmouseover = () => item.style.background = '#333333';
                item.onmouseout = () => item.style.background = 'transparent';
                item.onclick = onClick;
                return item;
            }

            function hideContextMenu() {
                if (contextMenu) {
                    contextMenu.style.display = 'none';
                }
                currentTarget = null;
            }

            function showContextMenu(x, y, items) {
                if (!contextMenu) {
                    contextMenu = createContextMenu();
                    document.body.appendChild(contextMenu);
                }

                contextMenu.innerHTML = '';
                items.forEach((item, index) => {
                    if (index === items.length - 1) {
                        item.style.borderBottom = 'none';
                    }
                    contextMenu.appendChild(item);
                });

                const maxX = window.innerWidth - contextMenu.offsetWidth - 10;
                const maxY = window.innerHeight - contextMenu.offsetHeight - 10;
                contextMenu.style.left = Math.max(0, Math.min(x, maxX)) + 'px';
                contextMenu.style.top = Math.max(0, Math.min(y, maxY)) + 'px';
                contextMenu.style.display = 'block';
            }

            function downloadImage(url, filename) {
                console.log('[Context Menu] Downloading image:', url, 'as', filename);

                fetch(url)
                    .then(response => response.blob())
                    .then(blob => {
                        const blobUrl = URL.createObjectURL(blob);
                        const a = document.createElement('a');
                        a.href = blobUrl;
                        a.download = filename || 'image';

                        if (window.__calmDownloadFilenames) {
                            window.__calmDownloadFilenames.set(blobUrl, a.download);
                        }

                        document.body.appendChild(a);
                        a.click();
                        document.body.removeChild(a);

                        setTimeout(() => URL.revokeObjectURL(blobUrl), 100);
                        console.log('[Context Menu] Download triggered');
                    })
                    .catch(err => {
                        console.error('[Context Menu] Download failed:', err);
                    });
            }

            function getImageFilename(src) {
                try {
                    const url = new URL(src);
                    let filename = url.pathname.split('/').pop();

                    if (!filename || filename.length === 0) {
                        filename = 'image';
                    }

                    filename = decodeURIComponent(filename);

                    if (!filename.match(/\.(jpg|jpeg|png|gif|webp|svg|bmp|ico)$/i)) {
                        const format = src.match(/\.(jpg|jpeg|png|gif|webp|svg|bmp|ico)/i);
                        if (format) {
                            filename += format[0];
                        } else {
                            filename += '.png';
                        }
                    }

                    return filename;
                } catch (e) {
                    return 'image.png';
                }
            }

            function copyImageToClipboard(img) {
                const src = img.src || img.currentSrc;
                fetch(src)
                    .then(response => response.blob())
                    .then(blob => {
                        navigator.clipboard.write([
                            new ClipboardItem({ [blob.type]: blob })
                        ]).then(() => {
                            console.log('[Context Menu] Image copied to clipboard');
                        }).catch(err => {
                            console.error('[Context Menu] Failed to copy image:', err);
                        });
                    })
                    .catch(err => {
                        console.error('[Context Menu] Failed to fetch image:', err);
                    });
            }

            function copyLink(url) {
                navigator.clipboard.writeText(url).then(() => {
                    console.log('[Context Menu] Link copied to clipboard');
                }).catch(err => {
                    console.error('[Context Menu] Failed to copy link:', err);
                });
            }

            document.addEventListener('contextmenu', function(e) {
                e.preventDefault();
                hideContextMenu();

                console.log('[Context Menu] Right-click detected on:', e.target.tagName);

                const target = e.target;

                let img = null;
                if (target.tagName === 'IMG') {
                    img = target;
                } else if (target.tagName === 'PICTURE') {
                    img = target.querySelector('img');
                }

                const link = target.closest('a');
                const hasSelection = window.getSelection().toString().length > 0;

                const items = [];

                if (img) {
                    console.log('[Context Menu] Image detected:', img.src);
                    const src = img.src || img.currentSrc;
                    currentTarget = img;

                    items.push(createMenuItem('Save Image As...', () => {
                        const filename = getImageFilename(src);
                        downloadImage(src, filename);
                        hideContextMenu();
                    }));

                    items.push(createMenuItem('Copy Image', () => {
                        copyImageToClipboard(img);
                        hideContextMenu();
                    }));

                    items.push(createMenuItem('Copy Image Address', () => {
                        copyLink(src);
                        hideContextMenu();
                    }));

                    items.push(createMenuItem('Open Image in New Tab', () => {
                        window.open(src, '_blank');
                        hideContextMenu();
                    }));
                } else if (link) {
                    console.log('[Context Menu] Link detected:', link.href);
                    const href = link.href;

                    items.push(createMenuItem('Open Link in New Tab', () => {
                        window.open(href, '_blank');
                        hideContextMenu();
                    }));

                    items.push(createMenuItem('Copy Link Address', () => {
                        copyLink(href);
                        hideContextMenu();
                    }));
                } else if (hasSelection) {
                    items.push(createMenuItem('Copy', () => {
                        document.execCommand('copy');
                        hideContextMenu();
                    }));
                }

                if (items.length === 0) {
                    items.push(createMenuItem('Reload', () => {
                        window.location.reload();
                        hideContextMenu();
                    }));
                }

                items.push(createMenuItem('Inspect Element', () => {
                    if (window.ipc) {
                        window.ipc.postMessage(JSON.stringify({
                            action: 'inspect_element',
                            x: e.clientX,
                            y: e.clientY
                        }));
                    }
                    hideContextMenu();
                }));

                if (items.length > 0) {
                    console.log('[Context Menu] Showing menu with', items.length, 'items at', e.pageX, e.pageY);
                    showContextMenu(e.pageX, e.pageY, items);
                }
            }, true);

            document.addEventListener('click', hideContextMenu);
            document.addEventListener('scroll', hideContextMenu);
            window.addEventListener('resize', hideContextMenu);

            console.log('[Context Menu] Context menu handler initialized');
            }

            if (document.readyState === 'loading') {
                document.addEventListener('DOMContentLoaded', initContextMenu);
            } else {
                initContextMenu();
            }
        })();
    "#
}
