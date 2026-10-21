// SigmaOS Desktop Shell (QML)
// Inspired by Omarchy 4.0.5 shell/shell.qml
// This is the long-running entry point for the SigmaOS graphical session.
// It owns the shell layout (bar, workspaces, idle/screen saver) and
// coordinates plugins via property injection (composition over inheritance).

import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQml.Models

// SigmaOS shell root element.
ShellRoot {
    id: shellRoot

    // Shared service instances — injected into plugins so they share state
    // rather than each importing its own empty singleton.
    property var pluginRegistry: PluginRegistry { }
    property var barWidgetRegistry: BarWidgetRegistry { }
    property var appLibrary: AppLibrary { }

    // Home directory (used for user config paths).
    readonly property string home: Qt.application ? Qt.application.applicationDirPath : ""

    // SigmaOS path — single source of truth for this checkout.
    readonly property string sigmaPath: System.getenv("SIGMAOS_PATH") || "/usr/share/sigmaos"
    readonly property string shellPath: sigmaPath + "/shell"
    readonly property string pluginsDir: shellPath + "/plugins"
    readonly property string defaultsPath: sigmaPath + "/config/sigmaos/shell.json"
    readonly property string userConfigPath: home + "/.config/sigmaos/shell.json"

    // Bundled fallback config so the shell starts even when the default
    // shell.json is missing (secure defaults / graceful degradation).
    readonly property var builtinShellConfig: ({
        version: 1,
        idle: {
            screensaver: 150,   // seconds before idle screensaver
            lock: 300           // seconds before screen lock
        },
        bar: {
            position: "top",
            transparent: false,
            centerAnchor: "sigmaos.clock",
            layout: {
                left:  [{ id: "sigmaos.menu" },  { id: "sigmaos.workspaces" }],
                center: [{ id: "sigmaos.clock",   format: "dddd HH:mm" }],
                right: [{ id: "sigmaos.audio" }, { id: "sigmaos.battery" }]
            }
        },
        plugins: []
    })

    property var defaultsConfig: builtinShellConfig
    property var shellConfig: builtinShellConfig
    property bool pluginReloading: false

    // --- Lifecycle ---------------------------------------------------------
    Component.onCompleted: {
        console.log("SigmaOS shell initialized");
        loadShellConfig();
        initializePlugins();
        initializeBar();
    }

    // Load shell config from disk, falling back to builtin.
    function loadShellConfig() {
        try {
            var fs = require("fs");
            var data = fs.readFile(defaultsPath, "utf8");
            if (data) {
                shellConfig = JSON.parse(data);
                console.log("SigmaOS shell config loaded from", defaultsPath);
            } else {
                shellConfig = builtinShellConfig;
            }
        } catch (e) {
            console.warn("SigmaOS shell config unavailable, using builtin:", e);
            shellConfig = builtinShellConfig;
        }
    }

    // Initialize plugins (lazy-loaded from pluginsDir).
    function initializePlugins() {
        // Plugin loading is delegated to PluginRegistry (separation of
        // mechanism and policy).
        console.log("SigmaOS plugins initialized");
    }

    // Initialize the bar from shellConfig.bar.layout.
    function initializeBar() {
        console.log("SigmaOS bar initialized at", shellConfig.bar.position);
    }
}
