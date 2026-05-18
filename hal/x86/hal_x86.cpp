#include "../../include/sigma_hal.h"

static void hal_x86_halt() {
    sigma_printf("[HAL: x86 standalone] CPU halt executed.\n");
}

static const hal_ops_t standalone_x86_ops = {
    hal_x86_halt,
    0, 0, 0
};

extern "C" void init_standalone_hal() {
    hal_ops = &standalone_x86_ops;
}
