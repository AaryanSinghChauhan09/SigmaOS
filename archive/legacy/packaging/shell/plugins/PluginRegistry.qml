// SigmaOS shell plugin registry
// Provides dependency injection for shell plugins (inversion of control).

import QtQuick 2.0

QtObject {
    id: pluginRegistry

    // Registered plugins by ID.
    property var plugins: {}

    // Register a plugin instance.
    function register(id, plugin) {
        plugins[id] = plugin;
        console.log("SigmaOS shell plugin registered:", id);
    }

    // Retrieve a registered plugin.
    function get(id) {
        return plugins[id];
    }

    // List all registered plugin IDs.
    function list() {
        return Object.keys(plugins);
    }
}
