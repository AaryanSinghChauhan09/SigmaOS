import QtQuick
import QtQuick.Layouts
import QtQuick.Controls
import Quickshell
import Quickshell.Wayland

// SigmaOS Sovereign Desktop — Quickshell App Grid Launcher
// Modal Wayland overlay presenting searchable applications & shortcuts

PanelWindow {
    id: launcherOverlay
    anchors {
        top: true
        bottom: true
        left: true
        right: true
    }
    color: "#cc11111b"

    Rectangle {
        id: searchBox
        anchors.centerIn: parent
        width: 640
        height: 480
        radius: 16
        color: "#181825"
        border.color: "#89b4fa"
        border.width: 2

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: 20
            spacing: 16

            TextField {
                id: searchInput
                Layout.fillWidth: true
                height: 44
                placeholderText: "🔍  Type to search apps and commands..."
                color: "#cdd6f4"
                font.pixelSize: 15
                background: Rectangle {
                    color: "#313244"
                    radius: 8
                }
            }

            GridView {
                id: appGrid
                Layout.fillWidth: true
                Layout.fillHeight: true
                cellWidth: 140
                cellHeight: 110
                clip: true

                model: ListModel {
                    ListElement { name: "Terminal"; icon: "💻"; cmd: "foot" }
                    ListElement { name: "Browser"; icon: "🌐"; cmd: "firefox" }
                    ListElement { name: "Files"; icon: "📁"; cmd: "thunar" }
                    ListElement { name: "Editor"; icon: "📝"; cmd: "nvim" }
                    ListElement { name: "Obsidian"; icon: "📓"; cmd: "obsidian" }
                    ListElement { name: "Steam"; icon: "🎮"; cmd: "steam" }
                    ListElement { name: "AI Agent"; icon: "🤖"; cmd: "sigmaos-agent menu" }
                    ListElement { name: "Settings"; icon: "⚙️"; cmd: "sigmaos-doctor" }
                }

                delegate: Rectangle {
                    width: 130
                    height: 100
                    radius: 12
                    color: mouseArea.containsMouse ? "#313244" : "transparent"

                    ColumnLayout {
                        anchors.centerIn: parent
                        spacing: 6

                        Text {
                            text: model.icon
                            font.pixelSize: 32
                            Layout.alignment: Qt.AlignHCenter
                        }

                        Text {
                            text: model.name
                            font.bold: true
                            font.pixelSize: 12
                            color: "#cdd6f4"
                            Layout.alignment: Qt.AlignHCenter
                        }
                    }

                    MouseArea {
                        id: mouseArea
                        anchors.fill: parent
                        hoverEnabled: true
                        cursorShape: Qt.PointingHandCursor
                        onClicked: {
                            Quickshell.process([model.cmd])
                        }
                    }
                }
            }
        }
    }
}
