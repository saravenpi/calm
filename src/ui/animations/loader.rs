pub fn get_loading_animation() -> &'static str {
    r#"
    (function() {
        'use strict';

        if (!document.getElementById('calm-loader-style')) {
            const style = document.createElement('style');
            style.id = 'calm-loader-style';
            style.textContent = `
                #calm-loader {
                    position: fixed;
                    top: 0;
                    left: 0;
                    width: 100%;
                    height: 100%;
                    background: #0a0a0a;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    z-index: 999999;
                    opacity: 1;
                    transition: opacity 0.4s cubic-bezier(0.4, 0, 0.2, 1);
                }
                #calm-loader.hidden {
                    opacity: 0;
                    pointer-events: none;
                }
                .wave-container {
                    position: relative;
                    width: 200px;
                    height: 200px;
                }
                .wave {
                    position: absolute;
                    border: 2px solid rgba(255, 255, 255, 0.3);
                    border-radius: 50%;
                    top: 50%;
                    left: 50%;
                    transform: translate(-50%, -50%);
                    animation: pulse 2s ease-in-out infinite;
                    image-rendering: pixelated;
                    image-rendering: crisp-edges;
                }
                .wave:nth-child(1) { width: 20px; height: 20px; animation-delay: 0s; }
                .wave:nth-child(2) { width: 40px; height: 40px; animation-delay: 0.15s; }
                .wave:nth-child(3) { width: 60px; height: 60px; animation-delay: 0.3s; }
                .wave:nth-child(4) { width: 80px; height: 80px; animation-delay: 0.45s; }
                .wave:nth-child(5) { width: 100px; height: 100px; animation-delay: 0.6s; }
                .wave:nth-child(6) { width: 120px; height: 120px; animation-delay: 0.75s; }
                .wave:nth-child(7) { width: 140px; height: 140px; animation-delay: 0.9s; }
                @keyframes pulse {
                    0%, 100% {
                        opacity: 0.2;
                        transform: translate(-50%, -50%) scale(0.95);
                        border-color: rgba(255, 255, 255, 0.2);
                    }
                    50% {
                        opacity: 1;
                        transform: translate(-50%, -50%) scale(1.05);
                        border-color: rgba(255, 255, 255, 0.8);
                    }
                }
                .loader-text {
                    position: absolute;
                    top: 60%;
                    left: 50%;
                    transform: translate(-50%, -50%);
                    color: rgba(255, 255, 255, 0.8);
                    font-size: 13px;
                    font-family: 'gohu', monospace;
                    letter-spacing: 2px;
                    animation: fade 2s ease-in-out infinite;
                }
                @keyframes fade {
                    0%, 100% { opacity: 0.4; }
                    50% { opacity: 1; }
                }
            `;
            document.head.appendChild(style);
        }

        const existingLoader = document.getElementById('calm-loader');
        if (existingLoader) {
            existingLoader.classList.remove('hidden');
        } else {
            const loader = document.createElement('div');
            loader.id = 'calm-loader';
            loader.innerHTML = `
                <div class="wave-container">
                    <div class="wave"></div>
                    <div class="wave"></div>
                    <div class="wave"></div>
                    <div class="wave"></div>
                    <div class="wave"></div>
                    <div class="wave"></div>
                    <div class="wave"></div>
                    <div class="loader-text">CALM</div>
                </div>
            `;

            const addLoader = () => {
                if (document.body && !document.getElementById('calm-loader')) {
                    document.body.appendChild(loader);
                }
            };

            if (document.readyState === 'loading') {
                document.addEventListener('DOMContentLoaded', addLoader);
            } else {
                addLoader();
            }
        }

        const hideLoader = () => {
            const loader = document.getElementById('calm-loader');
            if (loader) {
                setTimeout(() => {
                    loader.classList.add('hidden');
                    setTimeout(() => {
                        if (loader.parentNode) {
                            loader.remove();
                        }
                    }, 400);
                }, 100);
            }
        };

        window.addEventListener('load', hideLoader);

        if (document.readyState === 'complete') {
            hideLoader();
        }

        setTimeout(hideLoader, 10000);
    })();
    "#
}

pub fn get_navigation_loader() -> &'static str {
    r#"
    (function() {
        'use strict';

        window.__calmShowLoader = function() {
            let loader = document.getElementById('calm-loader');
            if (!loader) {
                const style = document.getElementById('calm-loader-style');
                if (style) {
                    loader = document.createElement('div');
                    loader.id = 'calm-loader';
                    loader.innerHTML = `
                        <div class="wave-container">
                            <div class="wave"></div>
                            <div class="wave"></div>
                            <div class="wave"></div>
                            <div class="wave"></div>
                            <div class="wave"></div>
                            <div class="wave"></div>
                            <div class="wave"></div>
                            <div class="loader-text">CALM</div>
                        </div>
                    `;
                    document.body.appendChild(loader);
                }
            } else {
                loader.classList.remove('hidden');
            }
        };

        window.__calmHideLoader = function() {
            const loader = document.getElementById('calm-loader');
            if (loader) {
                loader.classList.add('hidden');
            }
        };
    })();
    "#
}
