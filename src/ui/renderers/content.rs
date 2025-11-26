pub fn get_content_renderer() -> &'static str {
    r#"
    (function() {
        'use strict';

        const isXML = () => {
            return document.contentType === 'text/xml' ||
                   document.contentType === 'application/xml' ||
                   document.contentType === 'application/rss+xml' ||
                   document.contentType === 'application/atom+xml' ||
                   (document.documentElement && document.documentElement.tagName.toLowerCase() === 'rss') ||
                   (document.documentElement && document.documentElement.tagName.toLowerCase() === 'feed');
        };

        const isJSON = () => {
            return document.contentType === 'application/json' ||
                   document.contentType === 'text/json';
        };

        const renderRSS = () => {
            const root = document.documentElement;
            let items = [];
            let feedTitle = 'RSS Feed';
            let feedDescription = '';
            let feedLink = '';

            if (root.tagName.toLowerCase() === 'rss') {
                const channel = root.querySelector('channel');
                if (channel) {
                    feedTitle = channel.querySelector('title')?.textContent || 'RSS Feed';
                    feedDescription = channel.querySelector('description')?.textContent || '';
                    feedLink = channel.querySelector('link')?.textContent || '';

                    const itemNodes = channel.querySelectorAll('item');
                    itemNodes.forEach(item => {
                        items.push({
                            title: item.querySelector('title')?.textContent || 'Untitled',
                            link: item.querySelector('link')?.textContent || '#',
                            description: item.querySelector('description')?.textContent || '',
                            pubDate: item.querySelector('pubDate')?.textContent || '',
                            author: item.querySelector('author')?.textContent || item.querySelector('dc\\:creator')?.textContent || ''
                        });
                    });
                }
            } else if (root.tagName.toLowerCase() === 'feed') {
                feedTitle = root.querySelector('title')?.textContent || 'Atom Feed';
                feedDescription = root.querySelector('subtitle')?.textContent || '';
                const linkEl = root.querySelector('link[rel="alternate"]');
                feedLink = linkEl?.getAttribute('href') || '';

                const entryNodes = root.querySelectorAll('entry');
                entryNodes.forEach(entry => {
                    const linkEl = entry.querySelector('link[rel="alternate"]');
                    items.push({
                        title: entry.querySelector('title')?.textContent || 'Untitled',
                        link: linkEl?.getAttribute('href') || '#',
                        description: entry.querySelector('summary')?.textContent || entry.querySelector('content')?.textContent || '',
                        pubDate: entry.querySelector('published')?.textContent || entry.querySelector('updated')?.textContent || '',
                        author: entry.querySelector('author name')?.textContent || ''
                    });
                });
            }

            const html = `
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>${feedTitle}</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: #0a0a0a;
            color: #e0e0e0;
            line-height: 1.6;
            padding: 20px;
        }
        .container {
            max-width: 900px;
            margin: 0 auto;
            animation: fadeIn 0.4s ease-in-out;
        }
        @keyframes fadeIn {
            from { opacity: 0; transform: translateY(20px); }
            to { opacity: 1; transform: translateY(0); }
        }
        .feed-header {
            background: #1a1a1a;
            border: 1px solid #2a2a2a;
            border-radius: 12px;
            padding: 30px;
            margin-bottom: 30px;
            transition: all 0.3s ease;
        }
        .feed-header:hover {
            border-color: #3a3a3a;
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
        }
        .feed-title {
            font-size: 32px;
            font-weight: 700;
            color: #ffffff;
            margin-bottom: 12px;
        }
        .feed-description {
            font-size: 16px;
            color: #999;
            margin-bottom: 16px;
        }
        .feed-link {
            display: inline-block;
            color: #667eea;
            text-decoration: none;
            font-size: 14px;
            padding: 8px 16px;
            background: rgba(102, 126, 234, 0.1);
            border-radius: 6px;
            transition: all 0.2s;
        }
        .feed-link:hover {
            background: rgba(102, 126, 234, 0.2);
            transform: translateX(2px);
        }
        .feed-meta {
            margin-top: 20px;
            padding-top: 20px;
            border-top: 1px solid #2a2a2a;
            color: #666;
            font-size: 14px;
        }
        .item {
            background: #1a1a1a;
            border: 1px solid #2a2a2a;
            border-radius: 12px;
            padding: 24px;
            margin-bottom: 20px;
            transition: all 0.3s;
            animation: slideIn 0.4s ease-out;
        }
        @keyframes slideIn {
            from { opacity: 0; transform: translateX(-20px); }
            to { opacity: 1; transform: translateX(0); }
        }
        .item:hover {
            border-color: #3a3a3a;
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
            transform: translateY(-2px);
        }
        .item-title {
            font-size: 22px;
            font-weight: 600;
            margin-bottom: 12px;
        }
        .item-title a {
            color: #ffffff;
            text-decoration: none;
            transition: color 0.2s;
        }
        .item-title a:hover {
            color: #667eea;
        }
        .item-meta {
            color: #999;
            font-size: 13px;
            margin-bottom: 12px;
        }
        .item-meta span {
            margin-right: 16px;
        }
        .item-description {
            color: #ccc;
            font-size: 15px;
            line-height: 1.7;
        }
        .item-description p {
            margin-bottom: 12px;
        }
        .badge {
            background: rgba(102, 126, 234, 0.15);
            color: #667eea;
            padding: 4px 10px;
            border-radius: 4px;
            font-size: 12px;
            font-weight: 500;
            display: inline-block;
            margin-bottom: 12px;
        }
        @media (max-width: 768px) {
            body {
                padding: 12px;
            }
            .feed-header {
                padding: 20px;
            }
            .item {
                padding: 16px;
            }
            .feed-title {
                font-size: 24px;
            }
            .item-title {
                font-size: 18px;
            }
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="feed-header">
            <div class="badge">📡 RSS Feed</div>
            <h1 class="feed-title">${feedTitle}</h1>
            ${feedDescription ? `<p class="feed-description">${feedDescription}</p>` : ''}
            ${feedLink ? `<a href="${feedLink}" class="feed-link" target="_blank">Visit Website →</a>` : ''}
            <div class="feed-meta">
                ${items.length} item${items.length !== 1 ? 's' : ''} • Rendered by Calm Browser
            </div>
        </div>
        ${items.map(item => `
            <div class="item">
                <h2 class="item-title">
                    <a href="${item.link}" target="_blank">${item.title}</a>
                </h2>
                <div class="item-meta">
                    ${item.pubDate ? `<span>📅 ${new Date(item.pubDate).toLocaleDateString('en-US', { year: 'numeric', month: 'long', day: 'numeric' })}</span>` : ''}
                    ${item.author ? `<span>✍️ ${item.author}</span>` : ''}
                </div>
                ${item.description ? `<div class="item-description">${item.description}</div>` : ''}
            </div>
        `).join('')}
    </div>
</body>
</html>`;

            document.open();
            document.write(html);
            document.close();
        };

        const renderJSON = () => {
            const jsonText = document.body.textContent;
            let jsonData;
            let parseError = null;

            try {
                jsonData = JSON.parse(jsonText);
            } catch (e) {
                parseError = e.message;
            }

            const syntaxHighlight = (obj) => {
                if (typeof obj !== 'string') {
                    obj = JSON.stringify(obj, null, 2);
                }
                obj = obj.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
                return obj.replace(/("(\\u[a-zA-Z0-9]{4}|\\[^u]|[^\\"])*"(\s*:)?|\b(true|false|null)\b|-?\d+(?:\.\d*)?(?:[eE][+\-]?\d+)?)/g, function (match) {
                    let cls = 'json-number';
                    if (/^"/.test(match)) {
                        if (/:$/.test(match)) {
                            cls = 'json-key';
                        } else {
                            cls = 'json-string';
                        }
                    } else if (/true|false/.test(match)) {
                        cls = 'json-boolean';
                    } else if (/null/.test(match)) {
                        cls = 'json-null';
                    }
                    return '<span class="' + cls + '">' + match + '</span>';
                });
            };

            const html = `
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>JSON Viewer</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        body {
            font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
            background: #0a0a0a;
            color: #e0e0e0;
            padding: 20px;
            animation: fadeIn 0.4s ease-in-out;
        }
        @keyframes fadeIn {
            from { opacity: 0; }
            to { opacity: 1; }
        }
        .container {
            max-width: 1200px;
            margin: 0 auto;
        }
        .header {
            background: #1a1a1a;
            border: 1px solid #2a2a2a;
            border-radius: 12px;
            padding: 20px 30px;
            margin-bottom: 20px;
            display: flex;
            align-items: center;
            justify-content: space-between;
            transition: all 0.3s ease;
        }
        .header:hover {
            border-color: #3a3a3a;
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
        }
        .title {
            display: flex;
            align-items: center;
            gap: 12px;
        }
        .badge {
            background: rgba(102, 126, 234, 0.15);
            color: #667eea;
            padding: 6px 12px;
            border-radius: 6px;
            font-size: 13px;
            font-weight: 600;
        }
        .stats {
            color: #999;
            font-size: 13px;
        }
        .json-container {
            background: #1a1a1a;
            border: 1px solid #2a2a2a;
            border-radius: 12px;
            padding: 30px;
            overflow-x: auto;
            transition: all 0.3s ease;
        }
        .json-container:hover {
            border-color: #3a3a3a;
        }
        .error {
            background: rgba(231, 76, 60, 0.1);
            border: 1px solid rgba(231, 76, 60, 0.3);
            color: #e74c3c;
            padding: 20px;
            border-radius: 8px;
        }
        pre {
            margin: 0;
            font-size: 14px;
            line-height: 1.6;
            white-space: pre-wrap;
            word-wrap: break-word;
        }
        .json-key {
            color: #667eea;
            font-weight: 500;
        }
        .json-string {
            color: #38ef7d;
        }
        .json-number {
            color: #f39c12;
        }
        .json-boolean {
            color: #e74c3c;
            font-weight: 600;
        }
        .json-null {
            color: #999;
            font-style: italic;
        }
        .copy-btn {
            background: rgba(102, 126, 234, 0.15);
            color: #667eea;
            border: 1px solid rgba(102, 126, 234, 0.3);
            padding: 8px 16px;
            border-radius: 6px;
            cursor: pointer;
            font-size: 13px;
            font-weight: 500;
            transition: all 0.2s;
        }
        .copy-btn:hover {
            background: rgba(102, 126, 234, 0.25);
            transform: translateY(-1px);
        }
        .copy-btn:active {
            transform: translateY(0);
        }
        @media (max-width: 768px) {
            body {
                padding: 12px;
            }
            .header {
                flex-direction: column;
                align-items: flex-start;
                gap: 12px;
            }
            .json-container {
                padding: 16px;
            }
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <div class="title">
                <div class="badge">{ } JSON</div>
                <div class="stats">${parseError ? 'Parse Error' : 'Valid JSON'}</div>
            </div>
            ${!parseError ? '<button class="copy-btn" onclick="copyToClipboard()">📋 Copy</button>' : ''}
        </div>
        <div class="json-container">
            ${parseError ?
                `<div class="error"><strong>JSON Parse Error:</strong><br>${parseError}</div>` :
                `<pre>${syntaxHighlight(jsonData)}</pre>`
            }
        </div>
    </div>
    <script>
        function copyToClipboard() {
            const jsonText = ${JSON.stringify(jsonText)};
            navigator.clipboard.writeText(jsonText).then(() => {
                const btn = document.querySelector('.copy-btn');
                btn.textContent = '✓ Copied!';
                setTimeout(() => {
                    btn.textContent = '📋 Copy';
                }, 2000);
            });
        }
    </script>
</body>
</html>`;

            document.open();
            document.write(html);
            document.close();
        };

        if (document.readyState === 'loading') {
            document.addEventListener('DOMContentLoaded', () => {
                if (isXML()) {
                    renderRSS();
                } else if (isJSON()) {
                    renderJSON();
                }
            });
        } else {
            if (isXML()) {
                renderRSS();
            } else if (isJSON()) {
                renderJSON();
            }
        }
    })();
    "#
}
