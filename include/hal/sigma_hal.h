#ifndef SIGMA_HAL_H
#define SIGMA_HAL_H

#include "../../include/core/sigma_types.h"
#include "../../include/core/SigmaOOP.hpp"

#ifdef __cplusplus
namespace SigmaOS {
namespace Kernel {
namespace HAL {

enum class DeviceType {
    STORAGE,
    NETWORK,
    DISPLAY,
    INPUT,
    UNKNOWN
};

struct DeviceDescriptor {
    char name[32];
    DeviceType type;
    sigma_u32 vendor_id;
    sigma_u32 device_id;
};

class SovereignHAL : public SigmaOS::SigmaObject, public SigmaOS::SigmaSingleton<SovereignHAL> {
    friend class SigmaOS::SigmaSingleton<SovereignHAL>;
public:
    const char* type_name() const noexcept override { return "SovereignHAL"; }

    void init();
    void probeBus();
    void registerDriver(const char* name, DeviceType type);
    
    sigma_u32 getDeviceCount() const { return m_device_count; }

private:
    SovereignHAL() : m_device_count(0) {}
    sigma_u32 m_device_count;
    DeviceDescriptor m_lattice_devices[256];
};

} // namespace HAL
} // namespace Kernel
} // namespace SigmaOS
#endif

#ifdef __cplusplus
extern "C" {
#endif

void hal_init(void);
void hal_shutdown(void);
void hal_probe(void);

/* --- CPU Control --- */
static inline void cpu_pause(void) {
    __asm__ __volatile__("pause" ::: "memory");
}

static inline void cpu_halt(void) {
    __asm__ __volatile__("hlt" ::: "memory");
}

/* --- Port I/O --- */
static inline void port_outb(sigma_u16 port, sigma_u8 val) {
    __asm__ __volatile__("outb %0, %1" : : "a"(val), "Nd"(port));
}

static inline sigma_u8 port_inb(sigma_u16 port) {
    sigma_u8 ret;
    __asm__ __volatile__("inb %1, %0" : "=a"(ret) : "Nd"(port));
    return ret;
}

/* --- Serial I/O --- */
#define COM1 0x3F8

static inline void serial_init(void) {
    port_outb(COM1 + 1, 0x00);
    port_outb(COM1 + 3, 0x80);
    port_outb(COM1 + 0, 0x03);
    port_outb(COM1 + 1, 0x00);
    port_outb(COM1 + 3, 0x03);
    port_outb(COM1 + 2, 0xC7);
    port_outb(COM1 + 4, 0x0B);
}

static inline int is_transmit_empty(void) {
    return port_inb(COM1 + 5) & 0x20;
}

static inline void serial_putc(char c) {
    while (is_transmit_empty() == 0);
    port_outb(COM1, c);
}

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_HAL_H */
