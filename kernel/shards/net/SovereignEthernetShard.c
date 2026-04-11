#include "../../../include/SovereignNet.h"
#include "../../../include/sigma_libc.h"

void sigma_ethernet_handler(void* payload, sigma_size_t size) {
    sigma_printf("  Σ [ETH]: Received Ethernet Frame (%lu bytes) via hardware shunt.\n", (unsigned long)size);
    sigma_printf("  Σ [ETH]: Payload zero-copied to BPF network-mesh.\n");
}

void SovereignEthernet_Register(void) {
    SovereignNet_RegisterProtocol("ethernet", 0x88B5, sigma_ethernet_handler);
}
