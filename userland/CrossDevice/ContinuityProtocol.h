#ifndef SIGMA_CROSS_DEVICE_H
#define SIGMA_CROSS_DEVICE_H

// SigmaOS Cross-Device Continuity Protocol
// Handoff, Universal Clipboard, and Cloud Sync via the Network Suite

// Broadcast presence for Handoff
void continuity_broadcast_presence(void);

// Sync local clipboard to the Zero-Trust mesh
void continuity_sync_clipboard(const char* mime_type, void* data, uint32_t length);

// Push IoT device state change to the network
void continuity_manage_iot_device(uint32_t device_id, uint8_t state);

#endif // SIGMA_CROSS_DEVICE_H

