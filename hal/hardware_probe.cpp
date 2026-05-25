#include "SovereignHAL.hpp"

namespace SigmaOS {
namespace HAL {

// Mock function to simulate PCI enumeration
void enumerate_pci_buses(BoardTelemetry* telemetry) {
    // In a real OS, this would scan the PCI configuration space (e.g., via port 0xCF8/0xCFC on x86)
    
    // Simulating the detection of some essential devices
    static PCI_Device mock_pci_devices[] = {
        {0x10DE, 0x1C03, 0x03, 0x00}, // NVIDIA GPU (Display Controller)
        {0x8086, 0x153A, 0x02, 0x00}, // Intel Ethernet
        {0x1022, 0x1453, 0x01, 0x06}, // AMD NVMe
        {0x8086, 0x02D2, 0x0C, 0x03}  // Intel USB 3.0
    };
    
    telemetry->pci_devices = mock_pci_devices;
    telemetry->pci_device_count = 4;
}

// Mock function to simulate USB enumeration
void enumerate_usb_buses(BoardTelemetry* telemetry) {
    // In a real OS, this would interact with the xHCI/eHCI controllers to enumerate connected USB devices
    
    // Simulating the detection of some USB peripherals
    static USB_Device mock_usb_devices[] = {
        {0x046D, 0xC52B, 0x03, 0x01}, // Logitech Unifying Receiver (HID)
        {0x05AC, 0x024F, 0x03, 0x01}, // Apple Keyboard (HID)
        {0x0BDA, 0x8153, 0xFF, 0x00}  // Realtek USB Ethernet
    };
    
    telemetry->usb_devices = mock_usb_devices;
    telemetry->usb_device_count = 3;
}

// Helper to fully populate hardware on telemetry request
void probe_all_hardware(BoardTelemetry* telemetry) {
    enumerate_pci_buses(telemetry);
    enumerate_usb_buses(telemetry);
}

} // namespace HAL
} // namespace SigmaOS
