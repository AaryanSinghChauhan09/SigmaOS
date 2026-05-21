#include "sigma_kernel_types.h"
#include "sigma_slab.h"

// Basic Network Interface Abstraction (inspired by Linux net_device)

#define MAX_NET_INTERFACES 4

typedef struct {
    char name[16];
    uint8_t mac_addr[6];
    uint32_t ip_addr;
    uint32_t netmask;
    uint32_t gateway;
    int is_up;
    int (*xmit)(void* data, size_t len);
} sigma_net_dev_t;

static sigma_net_dev_t* interfaces[MAX_NET_INTERFACES];
static int num_interfaces = 0;

void sigma_net_core_init(void) {
    for (int i = 0; i < MAX_NET_INTERFACES; i++) {
        interfaces[i] = NULL;
    }
}

int sigma_net_register_device(const char* name, uint8_t* mac, int (*xmit_func)(void*, size_t)) {
    if (num_interfaces >= MAX_NET_INTERFACES) return -1;
    
    sigma_net_dev_t* dev = (sigma_net_dev_t*)kmalloc(sizeof(sigma_net_dev_t));
    if (!dev) return -1;
    
    int i = 0;
    while(name[i] && i < 15) {
        dev->name[i] = name[i];
        i++;
    }
    dev->name[i] = '\0';
    
    if (mac) {
        for(int j=0; j<6; j++) dev->mac_addr[j] = mac[j];
    }
    
    dev->ip_addr = 0;
    dev->netmask = 0;
    dev->gateway = 0;
    dev->is_up = 0;
    dev->xmit = xmit_func;
    
    interfaces[num_interfaces++] = dev;
    return num_interfaces - 1; // return device index
}

// Dummy loopback xmit
static int loopback_xmit(void* data, size_t len) {
    (void)data; (void)len;
    // In a real loopback, we would queue this directly to the receive path
    return 0; 
}

void init_loopback_net(void) {
    uint8_t lo_mac[6] = {0,0,0,0,0,0};
    int id = sigma_net_register_device("lo", lo_mac, loopback_xmit);
    if (id >= 0) {
        interfaces[id]->ip_addr = 0x7F000001; // 127.0.0.1
        interfaces[id]->netmask = 0xFF000000; // 255.0.0.0
        interfaces[id]->is_up = 1;
    }
}
