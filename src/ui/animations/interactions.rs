pub fn get_interaction_animations() -> &'static str {
    r#"
    (function() {
        'use strict';

        const style = document.createElement('style');
        style.id = 'calm-interactions';
        style.textContent = `
            button, .tab, input, select, textarea, a {
                transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1) !important;
            }

            button:active, .tab:active {
                transform: scale(0.96);
            }

            button:hover, .tab:hover {
                transform: translateY(-1px);
            }

            input:focus, select:focus, textarea:focus {
                transform: scale(1.01);
                box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
            }

            a:hover {
                transform: translateX(2px);
            }

            .ripple {
                position: absolute;
                border-radius: 50%;
                background: rgba(255, 255, 255, 0.3);
                transform: scale(0);
                animation: ripple-animation 0.6s ease-out;
                pointer-events: none;
            }

            @keyframes ripple-animation {
                to {
                    transform: scale(4);
                    opacity: 0;
                }
            }

            .shake {
                animation: shake 0.3s ease-in-out;
            }

            @keyframes shake {
                0%, 100% { transform: translateX(0); }
                25% { transform: translateX(-5px); }
                75% { transform: translateX(5px); }
            }

            .bounce-in {
                animation: bounceIn 0.4s cubic-bezier(0.68, -0.55, 0.265, 1.55);
            }

            @keyframes bounceIn {
                0% {
                    opacity: 0;
                    transform: scale(0.3);
                }
                50% {
                    transform: scale(1.05);
                }
                100% {
                    opacity: 1;
                    transform: scale(1);
                }
            }

            .slide-in-right {
                animation: slideInRight 0.3s cubic-bezier(0.4, 0, 0.2, 1);
            }

            @keyframes slideInRight {
                from {
                    opacity: 0;
                    transform: translateX(20px);
                }
                to {
                    opacity: 1;
                    transform: translateX(0);
                }
            }

            .fade-out {
                animation: fadeOut 0.3s cubic-bezier(0.4, 0, 0.2, 1) forwards;
            }

            @keyframes fadeOut {
                from {
                    opacity: 1;
                    transform: scale(1);
                }
                to {
                    opacity: 0;
                    transform: scale(0.9);
                }
            }

            .pulse {
                animation: pulse 1.5s ease-in-out infinite;
            }

            @keyframes pulse {
                0%, 100% {
                    opacity: 1;
                    transform: scale(1);
                }
                50% {
                    opacity: 0.7;
                    transform: scale(1.1);
                }
            }

            *::-webkit-scrollbar {
                width: 8px;
                height: 8px;
            }

            *::-webkit-scrollbar-track {
                background: transparent;
            }

            *::-webkit-scrollbar-thumb {
                background: rgba(255, 255, 255, 0.2);
                border-radius: 4px;
                transition: background 0.2s ease;
            }

            *::-webkit-scrollbar-thumb:hover {
                background: rgba(255, 255, 255, 0.3);
            }
        `;

        if (document.head) {
            document.head.appendChild(style);
        } else {
            document.addEventListener('DOMContentLoaded', function() {
                document.head.appendChild(style);
            });
        }

        document.addEventListener('click', function(e) {
            if (e.target.tagName === 'BUTTON' || e.target.closest('button')) {
                const button = e.target.tagName === 'BUTTON' ? e.target : e.target.closest('button');
                const ripple = document.createElement('span');
                ripple.classList.add('ripple');

                const rect = button.getBoundingClientRect();
                const size = Math.max(rect.width, rect.height);
                const x = e.clientX - rect.left - size / 2;
                const y = e.clientY - rect.top - size / 2;

                ripple.style.width = ripple.style.height = size + 'px';
                ripple.style.left = x + 'px';
                ripple.style.top = y + 'px';

                button.style.position = 'relative';
                button.style.overflow = 'hidden';
                button.appendChild(ripple);

                setTimeout(() => ripple.remove(), 600);
            }
        }, true);
    })();
    "#
}

pub fn get_audio_indicator_script() -> &'static str {
    r#"
    (function() {
        'use strict';

        let audioContext = null;
        let audioElements = [];
        let isPlaying = false;
        let checkInterval = null;
        let observer = null;

        const checkAudioState = () => {
            audioElements = Array.from(document.querySelectorAll('audio, video'));

            const hasPlayingAudio = audioElements.some(el => {
                return !el.paused && !el.muted && el.readyState >= 2;
            });

            if (hasPlayingAudio !== isPlaying) {
                isPlaying = hasPlayingAudio;
                window.__calmAudioPlaying = isPlaying;

                if (window.ipc) {
                    window.ipc.postMessage(JSON.stringify({
                        action: 'audio_state_changed',
                        playing: isPlaying
                    }));
                }
            }
        };

        checkInterval = setInterval(checkAudioState, 1000);

        document.addEventListener('play', checkAudioState, true);
        document.addEventListener('pause', checkAudioState, true);
        document.addEventListener('ended', checkAudioState, true);
        document.addEventListener('volumechange', checkAudioState, true);

        let mutationTimeout = null;
        observer = new MutationObserver(() => {
            if (mutationTimeout) clearTimeout(mutationTimeout);
            mutationTimeout = setTimeout(checkAudioState, 200);
        });

        const startObserving = () => {
            if (document.body && observer) {
                observer.observe(document.body, {
                    childList: true,
                    subtree: true,
                    attributes: false,
                    characterData: false
                });
            }
        };

        if (document.body) {
            startObserving();
        } else {
            document.addEventListener('DOMContentLoaded', startObserving);
        }

        window.__calmStopAllAudio = function() {
            if (checkInterval) {
                clearInterval(checkInterval);
                checkInterval = null;
            }

            if (observer) {
                observer.disconnect();
                observer = null;
            }

            if (mutationTimeout) {
                clearTimeout(mutationTimeout);
                mutationTimeout = null;
            }

            audioElements = Array.from(document.querySelectorAll('audio, video'));
            audioElements.forEach(el => {
                try {
                    el.pause();
                    el.currentTime = 0;
                    el.src = '';
                    el.load();
                } catch (e) {
                }
            });

            if (audioContext) {
                try {
                    audioContext.close();
                } catch (e) {
                }
                audioContext = null;
            }

            const mediaStreams = document.querySelectorAll('video, audio');
            mediaStreams.forEach(media => {
                try {
                    if (media.srcObject) {
                        const tracks = media.srcObject.getTracks();
                        tracks.forEach(track => {
                            try {
                                track.stop();
                            } catch (e) {
                            }
                        });
                        media.srcObject = null;
                    }
                } catch (e) {
                }
            });

            isPlaying = false;
            window.__calmAudioPlaying = false;
        };

        window.addEventListener('beforeunload', () => {
            if (window.__calmStopAllAudio) {
                window.__calmStopAllAudio();
            }
        });

        window.addEventListener('pagehide', () => {
            if (window.__calmStopAllAudio) {
                window.__calmStopAllAudio();
            }
        });
    })();
    "#
}
