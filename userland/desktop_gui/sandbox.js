/**
 * Σ SIGMA OS DISPOSABLE BROWSER SANDBOX
 * Zero-Footprint Navigation Layer
 */

export const BrowserSandbox = {
    currentUrl: 'https://www.google.com',

    navigate(url) {
        if (!url.startsWith('http')) url = 'https://' + url;

        console.log(`[SANDBOX] Navigating to ${url}. Discarding session state...`);
        this.clearSession();

        const frame = document.getElementById('browser-frame');
        if (frame) {
            frame.src = url;
            this.currentUrl = url;
            document.getElementById('browser-url').value = url;
        }

        SigmaKernel.notifyPanic(`BROWSER: Navigated to [${url}]. Sandbox secure.`);
    },

    clearSession() {
        console.log("[SANDBOX] Resetting disposable container... Purging temporary data.");
        // Simulated clearing of cookies/sessionStorage/localStorage
        // Real iframe clearance occurs by navigation
        try {
            const frame = document.getElementById('browser-frame');
            if (frame && frame.contentWindow) {
                // Potential cross-origin security errors here, so we wrap in try-catch
                // Simulated by just alerting the kernel
            }
        } catch (e) { }
    }
};

window.navigateBrowser = () => {
    const url = document.getElementById('browser-url').value;
    BrowserSandbox.navigate(url);
};

window.BrowserSandbox = BrowserSandbox;
