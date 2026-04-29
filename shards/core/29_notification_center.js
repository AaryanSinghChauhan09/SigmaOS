/**
 * Sovereign Notification Center (v1.0)
 * Professional USP: Historical Notification Management (macOS/Windows style).
 * Bridges the gap between toast alerts and persistent system logs.
 */

class NotificationCenter extends ZenithComponent {
    constructor() {
        super('notification-center-view');
        this.history = [];
        this.init();
    }

    init() {
        console.log('Σ://UI> Notification Center Online.');
    }

    push(title, body) {
        const item = {
            id: Date.now(),
            time: new Date().toLocaleTimeString(),
            title: title,
            body: body
        };
        this.history.unshift(item);
        if (this.history.length > 50) this.history.pop();
        this.render();
    }

    render() {
        // Mock rendering logic for the center
        console.log(`Σ://NOTIFY> Center updated with ${this.history.length} items.`);
    }

    clear() {
        this.history = [];
        this.render();
    }
}

window.NotificationCenter = NotificationCenter;
