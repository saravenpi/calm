pub fn get_overlay_html() -> &'static str {
    r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }

        html, body {
            width: 100vw;
            height: 100vh;
            background: transparent !important;
            overflow: hidden;
        }

        body {
            display: flex;
            align-items: center;
            justify-content: center;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
        }

        #input-container {
            width: 600px;
            max-width: 90%;
            background: rgba(20, 20, 20, 0.85);
            padding: 28px 32px;
            border-radius: 16px;
            backdrop-filter: blur(60px);
            box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
            border: 1px solid rgba(255, 255, 255, 0.15);
        }

        #url-input {
            width: 100%;
            padding: 18px 22px;
            font-size: 17px;
            font-weight: 400;
            background: rgba(255, 255, 255, 0.08);
            border: 1px solid rgba(255, 255, 255, 0.2);
            border-radius: 10px;
            color: #fff;
            outline: none;
            transition: all 0.2s ease;
        }

        #url-input:focus {
            background: rgba(255, 255, 255, 0.12);
            border-color: rgba(255, 255, 255, 0.3);
            box-shadow: 0 0 0 3px rgba(255, 255, 255, 0.1);
        }

        #url-input::placeholder {
            color: rgba(255, 255, 255, 0.5);
        }

        .hint {
            margin-top: 14px;
            font-size: 12px;
            font-weight: 400;
            color: rgba(255, 255, 255, 0.35);
            text-align: center;
            letter-spacing: 0.2px;
        }
    </style>
</head>
<body>
    <div id="input-container">
        <input
            type="text"
            id="url-input"
            placeholder="Enter URL or search..."
            autocomplete="off"
            autofocus
        />
        <div class="hint">Enter to open • Esc to cancel</div>
    </div>

    <script>
        const input = document.getElementById('url-input');

        input.focus();

        input.addEventListener('keydown', (e) => {
            if (e.key === 'Enter') {
                const value = input.value.trim();
                window.ipc.postMessage(JSON.stringify({
                    action: 'navigate',
                    url: value
                }));
            } else if (e.key === 'Escape') {
                window.ipc.postMessage(JSON.stringify({
                    action: 'cancel'
                }));
            }
        });

        window.addEventListener('blur', () => {
            window.ipc.postMessage(JSON.stringify({
                action: 'cancel'
            }));
        });
    </script>
</body>
</html>"#
}
