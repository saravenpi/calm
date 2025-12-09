pub fn get_script() -> &'static str {
    r#"
        function createIcon(pathData, size = 16) {
            return `<svg fill="none" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="${size}" height="${size}" style="display: inline-block; vertical-align: middle;"><path d="${pathData}" fill="currentColor"/></svg>`;
        }

        const icons = {
            folder: 'M4 4h8v2h10v14H2V4h2zm16 4H10V6H4v12h16V8z',
            clipboard: 'M10 2h6v2h4v18H4V4h4V2h2zm6 4v2H8V6H6v14h12V6h-2zm-2 0V4h-4v2h4z',
            trash: 'M16 2v4h6v2h-2v14H4V8H2V6h6V2h8zm-2 2h-4v2h4V4zm0 4H6v12h12V8h-4zm-5 2h2v8H9v-8zm6 0h-2v8h2v-8z',
            close: 'M5 5h2v2H5V5zm4 4H7V7h2v2zm2 2H9V9h2v2zm2 0h-2v2H9v2H7v2H5v2h2v-2h2v-2h2v-2h2v2h2v2h2v2h2v-2h-2v-2h-2v-2h-2v-2zm2-2v2h-2V9h2zm2-2v2h-2V7h2zm0 0V5h2v2h-2z',
            check: 'M18 6h2v2h-2V6zm-2 4V8h2v2h-2zm-2 2v-2h2v2h-2zm-2 2h2v-2h-2v2zm-2 2h2v-2h-2v2zm-2 0v2h2v-2H8zm-2-2h2v2H6v-2zm0 0H4v-2h2v2z'
        };

        window.downloads = [];
        window.isVisible = false;
        window.contextMenu = null;

        window.toggleVisibility = function(visible) {
            window.isVisible = visible;
            const panel = document.getElementById('downloads-panel');
            if (visible) {
                panel.style.transform = 'translateX(0)';
                window.downloads.forEach(d => {
                    d.seen = true;
                });
                updateDownloadBadge();
            } else {
                panel.style.transform = 'translateX(100%)';
            }
        };

        setInterval(() => {
            if (window.downloads.length > 0) {
                updateTimestamps();
            }
        }, 60000);

        function formatBytes(bytes) {
            if (bytes === 0) return '0 B';
            const k = 1024;
            const sizes = ['B', 'KB', 'MB', 'GB'];
            const i = Math.floor(Math.log(bytes) / Math.log(k));
            return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i];
        }

        window.addDownload = function(id, filename, totalBytes, filePath) {
            const download = {
                id: id,
                filename: filename,
                filePath: filePath || '',
                totalBytes: totalBytes,
                downloadedBytes: 0,
                completed: false,
                failed: false,
                seen: false,
                startTime: Date.now()
            };
            window.downloads.push(download);
            updateDownloadBadge();
            renderDownloads();
        };

        function updateDownloadBadge() {
            const unseenDownloads = window.downloads.filter(d => (d.completed || d.failed) && !d.seen).length;
            const inProgressDownloads = window.downloads.filter(d => !d.completed && !d.failed);
            const hasInProgress = inProgressDownloads.length > 0;

            try {
                if (window.ipc) {
                    window.ipc.postMessage(JSON.stringify({
                        action: 'update_download_count',
                        count: unseenDownloads,
                        inProgress: hasInProgress
                    }));

                    if (hasInProgress) {
                        const latestDownload = inProgressDownloads[0];
                        if (latestDownload.totalBytes > 0) {
                            const percent = Math.round((latestDownload.downloadedBytes / latestDownload.totalBytes) * 100);
                            window.ipc.postMessage(JSON.stringify({
                                action: 'update_download_progress',
                                percent: percent
                            }));
                        } else {
                            window.ipc.postMessage(JSON.stringify({
                                action: 'update_download_progress',
                                percent: -1
                            }));
                        }
                    } else {
                        window.ipc.postMessage(JSON.stringify({
                            action: 'update_download_progress',
                            percent: -1
                        }));
                    }
                }
            } catch(e) {
                console.log('Could not update download progress:', e);
            }
        }

        window.updateDownloadProgress = function(id, downloadedBytes, totalBytes) {
            const download = window.downloads.find(d => d.id === id);
            if (download) {
                download.downloadedBytes = downloadedBytes;
                if (totalBytes > 0) {
                    download.totalBytes = totalBytes;
                }
                updateDownloadItem(download);
            }
        };

        window.completeDownload = function(id, finalFilename) {
            const download = window.downloads.find(d => d.id === id);
            if (download) {
                if (finalFilename) {
                    download.filename = finalFilename;
                }
                download.completed = true;
                download.downloadedBytes = download.totalBytes;
                download.completedTime = Date.now();
                updateDownloadBadge();
                updateDownloadItem(download);
            }
        };

        window.failDownload = function(id) {
            const download = window.downloads.find(d => d.id === id);
            if (download) {
                download.failed = true;
                download.failedTime = Date.now();
                updateDownloadBadge();
                updateDownloadItem(download);
            }
        };

        window.loadDownloadHistory = function(historyEntries) {
            if (!Array.isArray(historyEntries)) return;

            historyEntries.forEach(entry => {
                const download = {
                    id: entry.id,
                    filename: entry.filename,
                    filePath: entry.file_path,
                    totalBytes: entry.total_bytes,
                    downloadedBytes: entry.total_bytes,
                    completed: entry.completed,
                    failed: entry.failed,
                    seen: true,
                    startTime: entry.timestamp * 1000,
                    completedTime: entry.completed ? entry.timestamp * 1000 : null,
                    failedTime: entry.failed ? entry.timestamp * 1000 : null
                };
                window.downloads.push(download);
            });

            updateDownloadBadge();
            renderDownloads();
        };

        window.clearDownloadHistory = function() {
            window.downloads = [];
            window.ipc.postMessage(JSON.stringify({action: 'clear_download_history'}));
            updateDownloadBadge();
            renderDownloads();
        };

        function showContextMenu(e, download) {
            e.preventDefault();
            hideContextMenu();

            const menu = document.createElement('div');
            menu.className = 'download-context-menu';
            menu.style.left = e.clientX + 'px';
            menu.style.top = e.clientY + 'px';

            const menuItems = [];

            if (download.completed && download.filePath) {
                menuItems.push({
                    label: createIcon(icons.folder, 12) + ' Show in Finder',
                    action: () => {
                        window.ipc.postMessage(JSON.stringify({
                            action: 'reveal_in_finder',
                            filePath: download.filePath
                        }));
                        hideContextMenu();
                    }
                });

                menuItems.push({
                    label: createIcon(icons.clipboard, 12) + ' Copy Path',
                    action: () => {
                        navigator.clipboard.writeText(download.filePath);
                        hideContextMenu();
                    }
                });
            }

            menuItems.push({
                label: createIcon(icons.trash, 12) + ' Remove from List',
                action: () => {
                    const index = window.downloads.findIndex(d => d.id === download.id);
                    if (index > -1) {
                        window.downloads.splice(index, 1);
                        updateDownloadBadge();
                        const itemEl = document.querySelector(`[data-download-id="${download.id}"]`);
                        if (itemEl) itemEl.remove();
                        if (window.downloads.length === 0) renderDownloads();
                    }
                    hideContextMenu();
                }
            });

            menuItems.forEach((item, index) => {
                const menuItem = document.createElement('div');
                menuItem.className = 'context-menu-item';
                menuItem.innerHTML = item.label;
                menuItem.onclick = item.action;
                menu.appendChild(menuItem);

                if (index < menuItems.length - 1) {
                    const separator = document.createElement('div');
                    separator.className = 'context-menu-separator';
                    menu.appendChild(separator);
                }
            });

            document.body.appendChild(menu);
            window.contextMenu = menu;

            setTimeout(() => {
                document.addEventListener('click', hideContextMenu, { once: true });
            }, 0);
        }

        function hideContextMenu() {
            if (window.contextMenu) {
                window.contextMenu.remove();
                window.contextMenu = null;
            }
        }

        function updateTimestamps() {
            function formatTime(timestamp) {
                const now = Date.now();
                const diff = now - timestamp;
                const seconds = Math.floor(diff / 1000);
                if (seconds < 60) return 'just now';
                const minutes = Math.floor(seconds / 60);
                if (minutes < 60) return minutes + 'm ago';
                const hours = Math.floor(minutes / 60);
                return hours + 'h ago';
            }

            window.downloads.forEach(download => {
                if ((download.completed || download.failed) && download.id) {
                    const itemEl = document.querySelector(`[data-download-id="${download.id}"]`);
                    if (itemEl) {
                        const timeEl = itemEl.querySelector('.download-time');
                        if (timeEl) {
                            const timestamp = download.completedTime || download.failedTime;
                            if (timestamp) {
                                timeEl.textContent = formatTime(timestamp);
                            }
                        }
                    }
                }
            });
        }

        function getDownloadHtml(download) {
            let progressBarHtml = '';
            let statusText = '';
            let percentText = '';
            let timeText = '';

            function formatTime(timestamp) {
                const now = Date.now();
                const diff = now - timestamp;
                const seconds = Math.floor(diff / 1000);
                if (seconds < 60) return 'just now';
                const minutes = Math.floor(seconds / 60);
                if (minutes < 60) return minutes + 'm ago';
                const hours = Math.floor(minutes / 60);
                return hours + 'h ago';
            }

            if (download.failed) {
                statusText = '<span class="download-status">' + createIcon(icons.close, 10) + ' failed</span>';
                timeText = download.failedTime ? formatTime(download.failedTime) : '';
                progressBarHtml = `<div class="download-progress-bar">
                    <div class="download-progress-fill failed" style="width: 100%; animation: none;"></div>
                </div>`;
            } else if (download.completed) {
                statusText = '<span class="download-status">' + createIcon(icons.check, 10) + ' complete</span>';
                timeText = download.completedTime ? formatTime(download.completedTime) : '';
                progressBarHtml = `<div class="download-progress-bar">
                    <div class="download-progress-fill completed" style="width: 100%; animation: none;"></div>
                </div>`;
            } else {
                const percent = download.totalBytes > 0
                    ? Math.round((download.downloadedBytes / download.totalBytes) * 100)
                    : 0;
                const downloadedMB = formatBytes(download.downloadedBytes);
                const totalMB = download.totalBytes > 0 ? formatBytes(download.totalBytes) : '?';

                if (download.totalBytes > 0 && download.downloadedBytes > 0) {
                    statusText = `<span><span class="download-spinner"></span>${percent}%</span>`;
                    percentText = `<span>${downloadedMB} / ${totalMB}</span>`;
                    progressBarHtml = `<div class="download-progress-bar">
                        <div class="download-progress-fill" style="width: ${percent}%;"></div>
                    </div>`;
                } else {
                    statusText = '<span><span class="download-spinner"></span>downloading</span>';
                    timeText = '';
                    progressBarHtml = `<div class="download-progress-bar">
                        <div class="download-progress-fill indeterminate"></div>
                    </div>`;
                }
            }

            const folderIcon = download.completed && download.filePath
                ? `<div class="download-folder-icon" title="Show in Finder">${createIcon(icons.folder, 14)}</div>`
                : '';

            return `
                <div class="download-header">
                    <div class="download-name">${download.filename}</div>
                    ${folderIcon}
                </div>
                ${progressBarHtml}
                <div class="download-info">
                    ${statusText}
                    ${percentText ? `<span class="download-size">${percentText}</span>` : ''}
                    ${timeText ? `<span class="download-time">${timeText}</span>` : ''}
                </div>
            `;
        }

        function bindDownloadEvents(itemEl, download) {
            itemEl.addEventListener('contextmenu', (e) => showContextMenu(e, download));

            if (download.completed && download.filePath) {
                const folderIconEl = itemEl.querySelector('.download-folder-icon');
                if (folderIconEl) {
                    folderIconEl.addEventListener('click', (e) => {
                        e.stopPropagation();
                        window.ipc.postMessage(JSON.stringify({
                            action: 'reveal_in_finder',
                            filePath: download.filePath
                        }));
                    });
                }
            }
        }

        function updateDownloadItem(download) {
            const itemEl = document.querySelector(`[data-download-id="${download.id}"]`);
            if (itemEl) {
                let className = 'download-item';
                if (download.completed) className += ' completed';
                if (download.failed) className += ' failed';
                itemEl.className = className;
                itemEl.innerHTML = getDownloadHtml(download);
                bindDownloadEvents(itemEl, download);
            } else {
                renderDownloads();
            }
        }

        function renderDownloads() {
            const listEl = document.getElementById('downloads-list');
            const emptyEl = document.getElementById('downloads-empty');

            if (window.downloads.length === 0) {
                emptyEl.style.display = 'block';
                listEl.innerHTML = '';
                return;
            }

            emptyEl.style.display = 'none';

            // Only clear and rebuild if we need to sync completely or initial load
            // But strict "renderDownloads" usually implies a full rebuild.
            // For partial updates we use updateDownloadItem.
            // Check if we need to append new items or full rebuild.
            // Here we do a full rebuild for simplicity when called directly,
            // but updateDownloadProgress calls updateDownloadItem.

            listEl.innerHTML = '';
            window.downloads.slice().reverse().forEach(download => {
                const itemEl = document.createElement('div');
                let className = 'download-item';
                if (download.completed) className += ' completed';
                if (download.failed) className += ' failed';
                itemEl.className = className;
                itemEl.setAttribute('data-download-id', download.id);
                itemEl.innerHTML = getDownloadHtml(download);
                bindDownloadEvents(itemEl, download);
                listEl.appendChild(itemEl);
            });
        }

    "#
}
