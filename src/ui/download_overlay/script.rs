pub fn get_script() -> &'static str {
    r#"
        window.downloads = [];
        window.isVisible = false;

        window.toggleVisibility = function(visible) {
            window.isVisible = visible;
            const panel = document.getElementById('downloads-panel');
            if (visible) {
                panel.style.transform = 'translateX(0)';
            } else {
                panel.style.transform = 'translateX(100%)';
            }
        };

        function formatBytes(bytes) {
            if (bytes === 0) return '0 B';
            const k = 1024;
            const sizes = ['B', 'KB', 'MB', 'GB'];
            const i = Math.floor(Math.log(bytes) / Math.log(k));
            return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i];
        }

        window.addDownload = function(id, filename, totalBytes) {
            const download = {
                id: id,
                filename: filename,
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
                download.totalBytes = totalBytes;
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
                    if (seconds < 60) return seconds + 's ago';
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
                    statusText = '<span><span class="download-spinner"></span>downloading</span>';
                    timeText = '';
                    progressBarHtml = `<div class="download-progress-bar">
                        <div class="download-progress-fill indeterminate"></div>
                    </div>`;
                }

                itemEl.innerHTML = `
                    <div class="download-name">${download.filename}</div>
                    ${progressBarHtml}
                    <div class="download-info">
                        ${statusText}
                        ${timeText ? `<span class="download-time">${timeText}</span>` : ''}
                    </div>
                `;

                listEl.appendChild(itemEl);
            });
        }

    "#
}
