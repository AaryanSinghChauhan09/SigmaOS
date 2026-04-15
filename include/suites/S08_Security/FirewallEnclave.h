#ifndef SIGMA_FIREWALL_ENCLAVE_H
#define SIGMA_FIREWALL_ENCLAVE_H

#include "sigma_types.h"


// SigmaOS Hardware Firewall Enclave
// Absorbs pf/iptables paradigms directly into the S08_Security networking boundary.

// Inject a new packet filter rule natively into the stack
void sec_fw_add_rule(uint32_t ip_address, uint16_t port, bool allow_traffic);

// Enable Stateful Packet Inspection explicitly utilizing NPU inference logic
void sec_fw_enable_spi(void);

// Immediately sever all non-authenticated external connections
void sec_fw_lockdown_node(void);

#endif // SIGMA_FIREWALL_ENCLAVE_H

