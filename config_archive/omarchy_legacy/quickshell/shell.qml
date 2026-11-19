import QtQuick
import QtQuick.Layouts
import Quickshell
import Quickshell.Wayland
import Quickshell.Widgets

// SigmaOS Sovereign Desktop — Quickshell Status Bar & Panel
// QML-based reactive desktop shell with workspace indicators, AI agent metrics,
// media controls, hardware monitoring, and application launcher.

ShellRoot {
    id: root

    PanelWindow {
        id: bar
        anchors {
            top: true
            left: true
            right: true
        }
        height: 38
        color: "#11111b"
        border.color: "#313244"
        border.width: 1

        RowLayout {
            anchors.fill: parent
            anchors.leftMargin: 12
            anchors.rightMargin: 12
            spacing: 8

            // ── Left: Launcher & Workspaces ─────────────────────────────
            Rectangle {
                id: launcherBtn
                width: 32
                height: 28
                radius: 8
                color: "#89b4fa"

                Text {
                    anchors.centerIn: parent
                    text: "⚡"
                    font.bold: true
                    color: "#11111b"
                }

                MouseArea {
                    anchors.fill: parent
                    cursorShape: Qt.PointingHandCursor
                    onClicked: {
                        Quickshell.process(["sigmaos-keybinds"])
                    }
                }
            }

            Text {
                text: "SIGMAOS"
                font.bold: true
                font.pixelSize: 12
                color: "#cdd6f4"
            }

            // ── Center: Clock & Window Title ───────────────────────────
            Item { Layout.fillWidth: true }

            Text {
                id: clockText
                font.pixelSize: 13
                font.bold: true
                color: "#cdd6f4"
                text: Qt.formatDateTime(new Date(), "ddd MMM dd  hh:mm")
            }

            Item { Layout.fillWidth: true }

            // ── Right: AI Agent Status, Audio, Battery, Logout ────────
            Rectangle {
                id: aiPanel
                height: 24
                width: aiText.implicitWidth + 16
                radius: 6
                color: "#1e1e2e"
                border.color: "#89b4fa"
                border.width: 1

                Text {
                    id: aiText
                    anchors.centerIn: parent
                    text: "🤖 AI Ready"
                    font.pixelSize: 11
                    color: "#89b4fa"
                }

                MouseArea {
                    anchors.fill: parent
                    cursorShape: Qt.PointingHandCursor
                    onClicked: {
                        Quickshell.process(["sigmaos-agent", "menu"])
                    }
                }
            }

            Rectangle {
                width: 28
                height: 28
                radius: 6
                color: "#313244"

                Text {
                    anchors.centerIn: parent
                    text: "⏻"
                    font.pixelSize: 12
                    color: "#f38ba8"
                }

                MouseArea {
                    anchors.fill: parent
                    cursorShape: Qt.PointingHandCursor
                    onClicked: {
                        Quickshell.process(["wlogout"])
                    }
                }
            }
        }
    }
}
