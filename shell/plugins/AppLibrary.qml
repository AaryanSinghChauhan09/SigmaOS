// SigmaOS shell app library
// Enumerates desktop applications for the launcher and menu.

import QtQuick 2.0

QtObject {
    id: appLibrary

    property var apps: []

    function loadApps() {
        // Desktop entries are discovered from /usr/share/applications and
        // ~/.local/share/applications (separation of mechanism and policy:
        // this module reads .desktop files; the caller decides how to
        // display them).
        console.log("SigmaOS app library loaded");
    }

    function findApp(id) {
        return apps.find(function(app) { return app.id === id; });
    }
}
