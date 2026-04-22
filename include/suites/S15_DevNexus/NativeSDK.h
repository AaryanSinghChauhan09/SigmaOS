#ifndef SIGMA_NATIVE_SDK_H
#define SIGMA_NATIVE_SDK_H

#include <stdint.h>

// SDK: Register a userspace handler for a Sovereign Event
int sigma_sdk_register_event_handler(uint32_t event_id, void (*handler)(void* data));

// SDK: Allocate memory in the secure userspace enclave
void* sigma_sdk_malloc(uint32_t size);

// SDK: Send telemetry to Zenith UI
int sigma_sdk_send_telemetry(const char* key, const char* value);

#endif // SIGMA_NATIVE_SDK_H
