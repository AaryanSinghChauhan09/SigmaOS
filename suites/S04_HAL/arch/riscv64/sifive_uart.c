/*
 * =============================================================================
 * Σ SIGMAOS: RISC-V SIFIVE UART DRIVER
 * =============================================================================
 * Serial driver for the SiFive UART (used by QEMU 'virt' machine and 
 * SiFive boards). Essential for RISC-V Edge/IoT deployment.
 *
 * Standard: C11, freestanding
 * =============================================================================
 */

#include "../../../../include/core/sigma_kernel_types.h"
#include "sigma/hal_contract.h"

#ifdef SIGMA_ARCH_RISCV64

/* Default SiFive UART base address on QEMU RISC-V virt machine */
#define SIFIVE_UART_BASE    0x10000000

#define UART_REG_TXDATA     0x00
#define UART_REG_RXDATA     0x04
#define UART_REG_TXCTRL     0x08
#define UART_REG_RXCTRL     0x0C
#define UART_REG_DIV        0x18

static inline void mmio_write(u32 offset, u32 val) {
    *(volatile u32*)(usize)(SIFIVE_UART_BASE + offset) = val;
}

static inline u32 mmio_read(u32 offset) {
    return *(volatile u32*)(usize)(SIFIVE_UART_BASE + offset);
}

static k_status sifive_uart_init(u32 baud) {
    (void)baud; /* Assuming firmware already configured the baud rate div */
    
    /* Enable TX and RX */
    mmio_write(UART_REG_TXCTRL, 1);
    mmio_write(UART_REG_RXCTRL, 1);
    
    return K_OK;
}

static void sifive_uart_write_byte(u8 c) {
    /* Wait until TX queue is not full (bit 31 is full flag) */
    while (mmio_read(UART_REG_TXDATA) & (1u << 31)) {
        __asm__ volatile("nop");
    }
    mmio_write(UART_REG_TXDATA, c);
}

static void sifive_uart_write_string(const char* s) {
    while (*s) {
        if (*s == '\n') sifive_uart_write_byte('\r');
        sifive_uart_write_byte(*s++);
    }
}

static u8 sifive_uart_read_byte(void) {
    u32 val;
    /* Wait until RX queue is not empty (bit 31 is empty flag) */
    while ((val = mmio_read(UART_REG_RXDATA)) & (1u << 31)) {
        __asm__ volatile("nop");
    }
    return (u8)(val & 0xFF);
}

static bool_t sifive_uart_data_available(void) {
    return (mmio_read(UART_REG_RXDATA) & (1u << 31)) == 0;
}

static const SigmaSerialOps g_sifive_uart_ops = {
    .init           = sifive_uart_init,
    .write_byte     = sifive_uart_write_byte,
    .write_string   = sifive_uart_write_string,
    .read_byte      = sifive_uart_read_byte,
    .data_available = sifive_uart_data_available,
};

void hal_register_sifive_uart(void) {
    hal_register_serial(&g_sifive_uart_ops);
}

#endif /* SIGMA_ARCH_RISCV64 */
