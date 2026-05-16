/*
 * =============================================================================
 * Σ SIGMAOS: RASPBERRY PI 3 (BCM2837) UART DRIVER
 * =============================================================================
 * Minimal serial driver for the Broadcom BCM2837 SoC (Raspberry Pi 3).
 * Essential for Edge/IoT debugging and headless deployment.
 *
 * Standard: C11, freestanding
 * =============================================================================
 */

#include "../../../../include/sigma_kernel_types.h"
#include "sigma/hal_contract.h"

#ifdef SIGMA_ARCH_AARCH64

/* BCM2837 Mini UART registers (MMIO base depends on device tree, assuming default) */
#define MMIO_BASE       0x3F000000
#define AUX_ENABLES     (MMIO_BASE + 0x215004)
#define AUX_MU_IO_REG   (MMIO_BASE + 0x215040)
#define AUX_MU_IER_REG  (MMIO_BASE + 0x215044)
#define AUX_MU_IIR_REG  (MMIO_BASE + 0x215048)
#define AUX_MU_LCR_REG  (MMIO_BASE + 0x21504C)
#define AUX_MU_MCR_REG  (MMIO_BASE + 0x215050)
#define AUX_MU_LSR_REG  (MMIO_BASE + 0x215054)
#define AUX_MU_CNTL_REG (MMIO_BASE + 0x215060)
#define AUX_MU_BAUD_REG (MMIO_BASE + 0x215068)

/* GPIO registers for pin muxing */
#define GPFSEL1         (MMIO_BASE + 0x200004)
#define GPPUD           (MMIO_BASE + 0x200094)
#define GPPUDCLK0       (MMIO_BASE + 0x200098)

static inline void mmio_write(u32 reg, u32 val) {
    *(volatile u32*)(usize)reg = val;
}

static inline u32 mmio_read(u32 reg) {
    return *(volatile u32*)(usize)reg;
}

static void delay(u32 count) {
    volatile u32 i;
    for (i = 0; i < count; i++) { __asm__ volatile("nop"); }
}

static k_status bcm_uart_init(u32 baud) {
    (void)baud; /* For simplicity, assuming system clock defaults for 115200 */
    
    /* 1. Enable Mini UART */
    mmio_write(AUX_ENABLES, 1);
    
    /* 2. Disable interrupts */
    mmio_write(AUX_MU_IER_REG, 0);
    
    /* 3. Disable Tx/Rx during config */
    mmio_write(AUX_MU_CNTL_REG, 0);
    
    /* 4. Configure 8-bit mode */
    mmio_write(AUX_MU_LCR_REG, 3);
    
    /* 5. Set RTS high */
    mmio_write(AUX_MU_MCR_REG, 0);
    
    /* 6. Clear FIFOs */
    mmio_write(AUX_MU_IIR_REG, 0xC6);
    
    /* 7. Set baud rate register (approx 115200 for 250MHz sys clock) */
    mmio_write(AUX_MU_BAUD_REG, 270);
    
    /* 8. Map UART to GPIO pins 14 (TX) and 15 (RX) */
    u32 r = mmio_read(GPFSEL1);
    r &= ~((7 << 12) | (7 << 15)); /* clear gpio 14, 15 */
    r |=  ((2 << 12) | (2 << 15)); /* set alt5 */
    mmio_write(GPFSEL1, r);
    
    /* Disable pull-up/down */
    mmio_write(GPPUD, 0);
    delay(150);
    mmio_write(GPPUDCLK0, (1 << 14) | (1 << 15));
    delay(150);
    mmio_write(GPPUDCLK0, 0);
    
    /* 9. Enable Tx/Rx */
    mmio_write(AUX_MU_CNTL_REG, 3);
    
    return K_OK;
}

static void bcm_uart_write_byte(u8 c) {
    /* Wait until transmitter is empty */
    while (!(mmio_read(AUX_MU_LSR_REG) & 0x20)) { __asm__ volatile("nop"); }
    mmio_write(AUX_MU_IO_REG, c);
}

static void bcm_uart_write_string(const char* s) {
    while (*s) {
        if (*s == '\n') bcm_uart_write_byte('\r');
        bcm_uart_write_byte(*s++);
    }
}

static u8 bcm_uart_read_byte(void) {
    while (!(mmio_read(AUX_MU_LSR_REG) & 0x01)) { __asm__ volatile("nop"); }
    return (u8)(mmio_read(AUX_MU_IO_REG) & 0xFF);
}

static bool_t bcm_uart_data_available(void) {
    return (mmio_read(AUX_MU_LSR_REG) & 0x01) != 0;
}

static const SigmaSerialOps g_bcm_uart_ops = {
    .init           = bcm_uart_init,
    .write_byte     = bcm_uart_write_byte,
    .write_string   = bcm_uart_write_string,
    .read_byte      = bcm_uart_read_byte,
    .data_available = bcm_uart_data_available,
};

void hal_register_bcm_uart(void) {
    hal_register_serial(&g_bcm_uart_ops);
}

#endif /* SIGMA_ARCH_AARCH64 */
