#include <stdio.h>
#include <assert.h>
#include "../../include/core/SovereignDriver.h"

// --- Mock Storage Driver Implementation ---

SovereignStatus_t mock_nvme_init(void* context) {
    printf("[MOCK] NVMe initialized.\n");
    return SOVEREIGN_STATUS_OK;
}

SovereignStatus_t mock_nvme_start(void* context) {
    printf("[MOCK] NVMe polling active.\n");
    return SOVEREIGN_STATUS_OK;
}

SovereignStatus_t mock_nvme_stop(void* context) {
    printf("[MOCK] NVMe cleanly stopped.\n");
    return SOVEREIGN_STATUS_OK;
}

bool mock_nvme_health(void* context) {
    return true; // Healthy
}

// Define the Mock Driver following the Sovereign Contract
SovereignDriver_t mock_storage = {
    .name = "Mock_NVMe_Core",
    .type = DRIVER_TYPE_STORAGE,
    .version = 1,
    .init = mock_nvme_init,
    .start = mock_nvme_start,
    .stop = mock_nvme_stop,
    .check_health = mock_nvme_health
};

// --- Test Suite ---

void test_driver_initialization() {
    printf("Running: test_driver_initialization\n");
    SovereignStatus_t status = mock_storage.init(NULL);
    assert(status == SOVEREIGN_STATUS_OK);
    printf("PASS: Driver initialization.\n");
}

void test_driver_state_machine() {
    printf("Running: test_driver_state_machine\n");
    assert(mock_storage.start(NULL) == SOVEREIGN_STATUS_OK);
    assert(mock_storage.check_health(NULL) == true);
    assert(mock_storage.stop(NULL) == SOVEREIGN_STATUS_OK);
    printf("PASS: Driver state machine.\n");
}

int main() {
    printf("--- SigmaOS Sovereign Unit Tests ---\n");
    test_driver_initialization();
    test_driver_state_machine();
    printf("All Unit Tests Passed Successfully.\n");
    return 0;
}
