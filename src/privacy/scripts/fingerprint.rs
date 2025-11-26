pub fn get_canvas_fingerprint_protection() -> &'static str {
    r#"
    (function() {
        'use strict';

        const originalToDataURL = HTMLCanvasElement.prototype.toDataURL;
        const originalGetImageData = CanvasRenderingContext2D.prototype.getImageData;

        const noise = () => Math.floor(Math.random() * 3) - 1;

        const addNoise = (data) => {
            for (let i = 0; i < data.length; i += 4) {
                data[i] = Math.max(0, Math.min(255, data[i] + noise()));
                data[i+1] = Math.max(0, Math.min(255, data[i+1] + noise()));
                data[i+2] = Math.max(0, Math.min(255, data[i+2] + noise()));
            }
        };

        HTMLCanvasElement.prototype.toDataURL = function() {
            const context = this.getContext('2d');
            if (context && this.width > 0 && this.height > 0) {
                const imageData = context.getImageData(0, 0, this.width, this.height);
                addNoise(imageData.data);
                context.putImageData(imageData, 0, 0);
            }
            return originalToDataURL.apply(this, arguments);
        };

        CanvasRenderingContext2D.prototype.getImageData = function() {
            const imageData = originalGetImageData.apply(this, arguments);
            addNoise(imageData.data);
            return imageData;
        };
    })();
    "#
}

pub fn get_webgl_fingerprint_protection() -> &'static str {
    r#"
    (function() {
        'use strict';

        const getParameterProxyHandler = {
            apply: function(target, thisArg, args) {
                const param = args[0];
                const noise = () => Math.random() * 0.0001;

                if (param === 37445) {
                    return 'Intel Inc.';
                }
                if (param === 37446) {
                    return 'Intel Iris OpenGL Engine';
                }
                if (param === 7936) {
                    return 'WebKit';
                }
                if (param === 7937) {
                    return 'WebKit WebGL';
                }
                if (param === 35724) {
                    return 16 + Math.floor(Math.random() * 16);
                }

                const result = target.apply(thisArg, args);
                if (typeof result === 'number') {
                    return result + noise();
                }
                return result;
            }
        };

        const config = { configurable: true, enumerable: true, writable: true };

        ['webgl', 'webgl2', 'experimental-webgl', 'experimental-webgl2'].forEach(contextType => {
            const getContext = HTMLCanvasElement.prototype.getContext;
            HTMLCanvasElement.prototype.getContext = function(type, ...args) {
                const context = getContext.apply(this, [type, ...args]);
                if (context && type.includes('webgl')) {
                    context.getParameter = new Proxy(context.getParameter, getParameterProxyHandler);
                }
                return context;
            };
        });
    })();
    "#
}

pub fn get_audio_fingerprint_protection() -> &'static str {
    r#"
    (function() {
        'use strict';

        const audioContext = window.AudioContext || window.webkitAudioContext;
        if (audioContext) {
            const originalGetChannelData = AudioBuffer.prototype.getChannelData;
            AudioBuffer.prototype.getChannelData = function() {
                const channelData = originalGetChannelData.apply(this, arguments);
                for (let i = 0; i < channelData.length; i++) {
                    channelData[i] = channelData[i] + Math.random() * 0.0001 - 0.00005;
                }
                return channelData;
            };

            const OriginalAnalyser = window.AnalyserNode || window.webkitAnalyserNode;
            if (OriginalAnalyser) {
                const originalGetFloatFrequencyData = OriginalAnalyser.prototype.getFloatFrequencyData;
                OriginalAnalyser.prototype.getFloatFrequencyData = function(array) {
                    originalGetFloatFrequencyData.apply(this, arguments);
                    for (let i = 0; i < array.length; i++) {
                        array[i] = array[i] + Math.random() * 0.1 - 0.05;
                    }
                    return array;
                };
            }
        }
    })();
    "#
}

pub fn get_font_fingerprint_protection() -> &'static str {
    r#"
    (function() {
        'use strict';

        const standardFonts = [
            'Arial', 'Courier New', 'Georgia', 'Times New Roman',
            'Trebuchet MS', 'Verdana', 'serif', 'sans-serif', 'monospace'
        ];

        if (document.fonts && document.fonts.check) {
            const originalCheck = document.fonts.check;
            document.fonts.check = function(font) {
                const fontFamily = font.match(/['"](.*?)['"]/);
                if (fontFamily && !standardFonts.includes(fontFamily[1])) {
                    return false;
                }
                return originalCheck.apply(this, arguments);
            };
        }
    })();
    "#
}
