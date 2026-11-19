import QtQuick
import QtQuick.Layouts
import Quickshell
import Quickshell.Wayland
import Quickshell.Widgets

// SigmaOS Sovereign Desktop — Quickshell Notification Server
// Renders smooth sliding toast notifications on top-right Wayland layer

PanelWindow {
    id: notificationRoot
    anchors {
        top: true
        right: true
    }
    margins {
        top: 48
        right: 16
    }
    width: 360
    height: Math.min(notificationList.contentHeight, 600)
    color: "transparent"

    ListView {
        id: notificationList
        anchors.fill: parent
        spacing: 8
        clip: true

        model: ListModel {
            id: mockModel
        }

        delegate: Rectangle {
            width: 360
            height: 72
            radius: 12
            color: "#1e1e2e"
            border.color: "#89b4fa"
            border.width: 1

            RowLayout {
                anchors.fill: parent
                anchors.margins: 12
                spacing: 12

                Text {
                    text: "🔔"
                    font.pixelSize: 20
                }

                ColumnLayout {
                    Layout.fillWidth: true
                    spacing: 4

                    Text {
                        text: model.title || "SigmaOS Notification"
                        font.bold: true
                        font.pixelSize: 13
                        color: "#cdd6f4"
                    }

                    Text {
                        text: model.body || "System event notification"
                        font.pixelSize: 11
                        color: "#a6adc8"
                        elide: Text.ElideRight
                    }
                }
            }
        }
    }
}
