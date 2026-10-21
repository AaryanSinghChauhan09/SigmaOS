// SigmaOS shell bar widget registry
// Manages bar widgets (menu, workspaces, clock, audio, battery, etc.).

import QtQuick 2.0

QtObject {
    id: barWidgetRegistry

    property var widgets: {}

    function register(id, widget) {
        widgets[id] = widget;
    }

    function get(id) {
        return widgets[id];
    }

    function list() {
        return Object.keys(widgets);
    }
}
