/**
 * SigmaOS: Sovereign Widget Engine
 * Inspired by Conky and Polybar.
 * USP: Highly customizable real-time desktop widgets for lattice observability.
 */

const WidgetEngine = {
    widgets: [],

    createWidget(id, title, updateFunc) {
        const container = SovereignUI.createWindow(title, 
            SovereignUI.createComponent('div', { id: `widget-${id}`, className: 'widget-content' }, ['Initializing...'])
        );
        
        const widget = { id, updateFunc, container };
        this.widgets.push(widget);
        this.startLoop(widget);
    },

    startLoop(widget) {
        setInterval(() => {
            const data = widget.updateFunc();
            const el = document.getElementById(`widget-${widget.id}`);
            if (el) el.innerHTML = data;
        }, 1000);
    }
};

if (typeof window !== 'undefined') {
    window.SigmaWidgetEngine = WidgetEngine;
}
