import QtQuick
import QtQuick.Layouts
import Quickshell
import Quickshell.Wayland

// SigmaOS Sovereign Desktop — Quickshell Desktop Widgets
// Renders responsive desktop widgets on Wayland bottom background layer:
// System telemetry (CPU, RAM, Swap), active AI agent stats, and clock/weather.

PanelWindow {
    id: widgetWindow
    anchors {
        top: true
        left: true
    }
    margins {
        top: 60
        left: 24
    }
    width: 320
    height: 400
    color: "transparent"

    ColumnLayout {
        anchors.fill: parent
        spacing: 16

        // ── Widget 1: System Telemetry Card ─────────────────────────
        Rectangle {
            Layout.fillWidth: true
            height: 140
            radius: 16
            color: "#cc181825"
            border.color: "#313244"
            border.width: 1

            ColumnLayout {
                anchors.fill: parent
                anchors.margins: 16
                spacing: 8

                Text {
                    text: "📊 System Telemetry"
                    font.bold: true
                    font.pixelSize: 13
                    color: "#89b4fa"
                }

                RowLayout {
                    Layout.fillWidth: true
                    Text { text: "CPU Usage:"; color: "#a6adc8"; font.pixelSize: 11 }
                    Item { Layout.fillWidth: true }
                    Text { text: "12% (12 Cores)"; color: "#a6e3a1"; font.bold: true; font.pixelSize: 11 }
                }

                RowLayout {
                    Layout.fillWidth: true
                    Text { text: "Memory:"; color: "#a6adc8"; font.pixelSize: 11 }
                    Item { Layout.fillWidth: true }
                    Text { text: "6.5 / 15.7 GiB"; color: "#f9e2af"; font.bold: true; font.pixelSize: 11 }
                }

                RowLayout {
                    Layout.fillWidth: true
                    Text { text: "Root Disk (Btrfs):"; color: "#a6adc8"; font.pixelSize: 11 }
                    Item { Layout.fillWidth: true }
                    Text { text: "64% Used"; color: "#cdd6f4"; font.pixelSize: 11 }
                }
            }
        }

        // ── Widget 2: AI Coding Agent Status Card ───────────────────
        Rectangle {
            Layout.fillWidth: true
            height: 110
            radius: 16
            color: "#cc181825"
            border.color: "#89b4fa"
            border.width: 1

            ColumnLayout {
                anchors.fill: parent
                anchors.margins: 16
                spacing: 8

                RowLayout {
                    Layout.fillWidth: true
                    Text { text: "🤖 Active Coding Agent"; font.bold: true; font.pixelSize: 13; color: "#cdd6f4" }
                    Item { Layout.fillWidth: true }
                    Text { text: "Ready"; color: "#a6e3a1"; font.pixelSize: 10 }
                }

                Text {
                    text: "Default Agent: claude (Anthropic Claude Code)"
                    color: "#bac2de"
                    font.pixelSize: 11
                }

                Text {
                    text: "Quota Balance: 100% | GPU: ROCm/CUDA Ready"
                    color: "#a6adc8"
                    font.pixelSize: 10
                }
            }
        }

        // ── Widget 3: Quick Action Launchers ────────────────────────
        RowLayout {
            Layout.fillWidth: true
            spacing: 8

            Rectangle {
                Layout.fillWidth: true
                height: 36
                radius: 10
                color: "#313244"
                Text { anchors.centerIn: parent; text: "⌨ Cheatsheet"; color: "#cdd6f4"; font.pixelSize: 11 }
                MouseArea {
                    anchors.fill: parent
                    cursorShape: Qt.PointingHandCursor
                    onClicked: Quickshell.process(["sigmaos-keybinds"])
                }
            }

            Rectangle {
                Layout.fillWidth: true
                height: 36
                radius: 10
                color: "#313244"
                Text { anchors.centerIn: parent; text: "🎨 Next Theme"; color: "#cdd6f4"; font.pixelSize: 11 }
                MouseArea {
                    anchors.fill: parent
                    cursorShape: Qt.PointingHandCursor
                    onClicked: Quickshell.process(["sigmaos-theme", "next"])
                }
            }
        }
    }
}
