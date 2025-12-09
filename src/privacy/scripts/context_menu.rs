pub fn get_script() -> &'static str {
    r#"
        (function() {
            console.log('[Context Menu] Script loading...');
            
            function initContextMenu() {
                console.log('[Context Menu] initContextMenu() called');
                try {
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
                } catch (e) {
                    console.error('[Context Menu] Failed to inject styles:', e);
                }

            let contextMenu = null;
            let currentTarget = null;

            const icons = {
                save: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path d="M19 21H5a1 1 0 0 1-1-1v-1h16v1a1 1 0 0 1-1 1zM5 3a1 1 0 0 0-1 1v1h16V4a1 1 0 0 0-1-1H5zm-1 3v10h16V6H4zm8 8-4-4h3V7h2v3h3l-4 4z"/></svg>',
                copy: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path d="M20 8h-2V5c0-1.1-.9-2-2-2h-3V1h-2v2H8c-1.1 0-2 .9-2 2v3H4v2h2v3H4v2h2v3c0 1.1.9 2 2 2h3v2h2v-2h3c1.1 0 2-.9 2-2v-3h2v-2h-2v-3h2V8zM8 5h8v14H8V5z"/></svg>',
                link: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path d="M9 7H7v2h2V7zm0 4H7v2h2v-2zm0 4H7v2h2v-2zm10-8h-2v2h2V7zm0 4h-2v2h2v-2zm0 4h-2v2h2v-2zM6 3v2H5c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2h-1V3h-2v2H8V3H6zm13 16H5V7h14v12z"/></svg>',
                open: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5v2H5v14h14v-5h2v5a2 2 0 0 1-2 2zM21 3h-7v2h4v0l-9 9 1 1 1 1 9-9v4h2V3z"/></svg>',
                reload: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path d="M12 4V2l-4 4 4 4V8a6 6 0 1 1-6 6H4a8 8 0 1 0 8-8z"/></svg>',
                inspect: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path d="M20 3H4c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 2v3H4V5h16zM4 19v-9h6v9H4zm8 0v-9h8v9h-8z"/></svg>',
                image: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5l4-8 3 4 2-2 5 6z"/></svg>'
            };

            function createContextMenu() {
                const menu = document.createElement('div');
                menu.id = 'calm-context-menu';
                menu.style.cssText = `
                    position: fixed;
                    background: #101010;
                    border: 2px solid #666666;
                    color: #ffffff;
                    font-family: 'gohu', monospace;
                    font-size: 11px;
                    z-index: 2147483647;
                    min-width: 180px;
                    display: none;
                    padding: 4px;
                    image-rendering: pixelated;
                    image-rendering: crisp-edges;
                `;
                return menu;
            }

            function createMenuItem(text, onClick, iconSvg = null) {
                const item = document.createElement('div');
                item.style.cssText = `
                    padding: 8px;
                    cursor: pointer;
                    user-select: none;
                    background: #1a1a1a;
                    border: 1px solid #333333;
                    margin-bottom: 4px;
                    transition: none;
                    display: flex;
                    align-items: center;
                    gap: 8px;
                `;

                if (iconSvg) {
                    const iconContainer = document.createElement('div');
                    iconContainer.innerHTML = iconSvg;
                    iconContainer.style.cssText = `
                        width: 16px;
                        height: 16px;
                        flex-shrink: 0;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                    `;
                    iconContainer.querySelector('svg').style.cssText = `
                        width: 16px;
                        height: 16px;
                        fill: currentColor;
                    `;
                    item.appendChild(iconContainer);
                }

                const textSpan = document.createElement('span');
                textSpan.textContent = text;
                item.appendChild(textSpan);

                item.onmouseover = () => {
                    item.style.background = '#ffffff';
                    item.style.color = '#000000';
                    item.style.borderColor = '#ffffff';
                };
                item.onmouseout = () => {
                    item.style.background = '#1a1a1a';
                    item.style.color = '#ffffff';
                    item.style.borderColor = '#333333';
                };
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
                        item.style.marginBottom = '0';
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
                    }, icons.save));

                    items.push(createMenuItem('Copy Image', () => {
                        copyImageToClipboard(img);
                        hideContextMenu();
                    }, icons.image));

                    items.push(createMenuItem('Copy Image Address', () => {
                        copyLink(src);
                        hideContextMenu();
                    }, icons.link));

                    items.push(createMenuItem('Open Image in New Tab', () => {
                        window.open(src, '_blank');
                        hideContextMenu();
                    }, icons.open));
                } else if (link) {
                    console.log('[Context Menu] Link detected:', link.href);
                    const href = link.href;

                    items.push(createMenuItem('Open Link in New Tab', () => {
                        window.open(href, '_blank');
                        hideContextMenu();
                    }, icons.open));

                    items.push(createMenuItem('Copy Link Address', () => {
                        copyLink(href);
                        hideContextMenu();
                    }, icons.link));
                } else if (hasSelection) {
                    items.push(createMenuItem('Copy', () => {
                        document.execCommand('copy');
                        hideContextMenu();
                    }, icons.copy));
                }

                if (items.length === 0) {
                    items.push(createMenuItem('Reload', () => {
                        window.location.reload();
                        hideContextMenu();
                    }, icons.reload));
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
                }, icons.inspect));

                if (items.length > 0) {
                    console.log('[Context Menu] Showing menu with', items.length, 'items at', e.clientX, e.clientY);
                    showContextMenu(e.clientX, e.clientY, items);
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
