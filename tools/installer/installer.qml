import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

ApplicationWindow {
    id: window
    width: 1024
    height: 720
    visible: true
    title: qsTr("SigmaOS Omarchy-Inspired Visual System Installer")
    color: "#1a1b26"

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 32
        spacing: 20

        Text {
            text: "SigmaOS Installation Setup"
            font.pixelSize: 28
            font.bold: true
            color: "#7aa2f7"
        }

        Text {
            text: "Opinionated, Batteries-Included Sovereign Operating System Environment"
            font.pixelSize: 14
            color: "#a9b1d6"
        }

        Rectangle {
            Layout.fillWidth: true
            height: 1
            color: "#414868"
        }

        // Dual-Boot & Partition Detection Section
        GroupBox {
            title: "Storage & Dual-Boot Configuration"
            Layout.fillWidth: true
            background: Rectangle { color: "#24283b"; radius: 8 }

            ColumnLayout {
                spacing: 10
                RadioButton {
                    text: "Auto-detect existing OS & Configure Dual-Boot (Btrfs / LVM2 / ZFS)"
                    checked: true
                }
                RadioButton {
                    text: "Erase disk and install SigmaOS as Sovereign Primary OS"
                }
            }
        }

        // Development Tools & Curated Suite
        GroupBox {
            title: "Curated Batteries-Included Software Selection"
            Layout.fillWidth: true
            background: Rectangle { color: "#24283b"; radius: 8 }

            RowLayout {
                spacing: 20
                CheckBox { text: "Neovim & Helix Editors"; checked: true }
                CheckBox { text: "Rust & LLVM Toolchain"; checked: true }
                CheckBox { text: "Wayland Zenith Desktop"; checked: true }
                CheckBox { text: "TUI Dashboard Suite"; checked: true }
            }
        }

        Item { Layout.fillHeight: true }

        RowLayout {
            Layout.fillWidth: true

            Button {
                text: "Quit"
                onClicked: Qt.quit()
            }

            Item { Layout.fillWidth: true }

            Button {
                text: "Begin Installation"
                highlighted: true
                onClicked: console.log("Installing SigmaOS...")
            }
        }
    }
}
