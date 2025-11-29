pub fn get_script() -> &'static str {
    r#"
        window.downloads = [];
        window.isVisible = false;
        window.contextMenu = null;

        window.toggleVisibility = function(visible) {
            window.isVisible = visible;
            const panel = document.getElementById('downloads-panel');
            if (visible) {
                panel.style.transform = 'translateX(0)';
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
                startTime: Date.now()
            };
            window.downloads.push(download);
            updateDownloadBadge();
            renderDownloads();
        };

        function updateDownloadBadge() {
            const activeDownloads = window.downloads.filter(d => !d.completed && !d.failed).length;

            try {
                if (window.parent && window.parent.updateDownloadCount) {
                    window.parent.updateDownloadCount(activeDownloads);
                }
            } catch(e) {
                console.log('Could not update parent download count:', e);
            }
        }

        window.updateDownloadProgress = function(id, downloadedBytes, totalBytes) {
            const download = window.downloads.find(d => d.id === id);
            if (download) {
                download.downloadedBytes = downloadedBytes;
                if (totalBytes > 0) {
                    download.totalBytes = totalBytes;
                }
                renderDownloads();
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
                renderDownloads();
            }
        };

        window.failDownload = function(id) {
            const download = window.downloads.find(d => d.id === id);
            if (download) {
                download.failed = true;
                download.failedTime = Date.now();
                updateDownloadBadge();
                renderDownloads();
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
                    label: '📁 Show in Finder',
                    action: () => {
                        window.ipc.postMessage(JSON.stringify({
                            action: 'reveal_in_finder',
                            filePath: download.filePath
                        }));
                        hideContextMenu();
                    }
                });

                menuItems.push({
                    label: '📋 Copy Path',
                    action: () => {
                        navigator.clipboard.writeText(download.filePath);
                        hideContextMenu();
                    }
                });
            }

            menuItems.push({
                label: '🗑️ Remove from List',
                action: () => {
                    const index = window.downloads.findIndex(d => d.id === download.id);
                    if (index > -1) {
                        window.downloads.splice(index, 1);
                        updateDownloadBadge();
                        renderDownloads();
                    }
                    hideContextMenu();
                }
            });

            menuItems.forEach((item, index) => {
                const menuItem = document.createElement('div');
                menuItem.className = 'context-menu-item';
                menuItem.textContent = item.label;
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

        function renderDownloads() {
            const listEl = document.getElementById('downloads-list');
            const emptyEl = document.getElementById('downloads-empty');

            if (window.downloads.length === 0) {
                emptyEl.style.display = 'block';
                return;
            }

            emptyEl.style.display = 'none';

            listEl.innerHTML = '';
            window.downloads.forEach(download => {
                const itemEl = document.createElement('div');
                let className = 'download-item';
                if (download.completed) className += ' completed';
                if (download.failed) className += ' failed';
                itemEl.className = className;
                itemEl.setAttribute('data-download-id', download.id);

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
                    statusText = '<span>✗ failed</span>';
                    timeText = download.failedTime ? formatTime(download.failedTime) : '';
                    progressBarHtml = `<div class="download-progress-bar">
                        <div class="download-progress-fill failed" style="width: 100%; animation: none;"></div>
                    </div>`;
                } else if (download.completed) {
                    statusText = '<span>✓ complete</span>';
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
                    ? `<div class="download-folder-icon" title="Show in Finder">📁</div>`
                    : '';

                itemEl.innerHTML = `
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

                listEl.appendChild(itemEl);
            });
        }

    "#
}
