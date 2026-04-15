#ifndef SIGMA_PERIPHERALS_DRIVER_H
#define SIGMA_PERIPHERALS_DRIVER_H

#include <sigma_types.h>

// SigmaOS Peripheral Hardware Drivers Module
// Handling specific zero-dependency I/O for Printers, Cameras, and Bluetooth bands.

// Initialize hardware Bluetooth radio and scan protocol
void hal_bt_init_radio(void);
void hal_bt_connect_device(const char* mac_address);

// Open an asynchronous stream to local or networked printer shards
void hal_printer_spool_document(const void* doc_blob, uint32_t doc_size);

// Allocate direct VRAM buffer for incoming camera feed parsing (used by AI Inference)
void* hal_camera_init_stream(uint32_t resolution_x, uint32_t resolution_y);

#endif // SIGMA_PERIPHERALS_DRIVER_H

