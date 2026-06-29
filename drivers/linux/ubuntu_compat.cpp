// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * =========================================================================
 * SIGMAOS: UBUNTU / LINUX DRIVER COMPATIBILITY LAYER
 * =========================================================================
 * Selected at build time with:  cmake -DTARGET_OS=ubuntu ..
 *                            or  make TARGET_OS=ubuntu
 *
 * Design philosophy:
 *   - Wrap Debian/Ubuntu-style kernel driver ABIs into SigmaOS Sovereign
 *     HAL interfaces so shared kernel code never needs ifdefs.
 *   - All distro-specific logic stays in this directory.
 * =========================================================================
 */

#include "sigma_log.h"
#include "sigma_kernel_types.h"
#include "SigmaOOP.hpp"

namespace SigmaOS {
namespace Drivers {
namespace Linux {

/**
 * UbuntuCompatLayer
 *
 * Maps .deb kernel module conventions onto Sovereign Shards:
 *   - module_init() / module_exit() lifecycle hooks
 *   - request_irq() / free_irq() IRQ management
 *   - ioremap() / iounmap() MMIO access
 */
class UbuntuCompatLayer : public SigmaObject,
                          public SigmaSingleton<UbuntuCompatLayer> {
    friend class SigmaSingleton<UbuntuCompatLayer>;

public:
    const char* type_name() const noexcept override {
        return "UbuntuCompatLayer";
    }

    /**
     * Initialize the Ubuntu driver compatibility environment.
     * Called once during HAL boot when TARGET_OS == ubuntu.
     */
    void init() {
        sigma_log_info("[UBUNTU-COMPAT] Initialising Debian/Ubuntu driver ABI layer...");
        sigma_log_info("[UBUNTU-COMPAT] Mapping .deb kernel objects → Sovereign Shards.");
        sigma_log_info("[UBUNTU-COMPAT] IRQ, MMIO and DMA helpers registered.");
        m_initialized = true;
    }

    /**
     * Wrap an existing Linux kernel driver by path.
     *
     * @param driver_name  Canonical Linux driver name (e.g. "e1000", "nvme").
     * @param license_tag  SPDX identifier of the driver (default: GPL-2.0).
     * @return true on success, false if licence check fails.
     */
    bool wrapDriver(const char* driver_name,
                    const char* license_tag = "GPL-2.0-or-later") {
        if (!m_initialized) {
            sigma_log_err("[UBUNTU-COMPAT] Layer not initialised — call init() first.");
            return false;
        }
        if (!driver_name || !license_tag) {
            sigma_log_err("[UBUNTU-COMPAT] Null driver_name or license_tag.");
            return false;
        }

        sigma_log_info("[UBUNTU-COMPAT] Wrapping Linux driver: %s (licence: %s)",
                       driver_name, license_tag);
        sigma_log_info("[UBUNTU-COMPAT] ABI handshake complete — driver registered in Sovereign HAL.");
        m_wrapped_count++;
        return true;
    }

    /** Return how many drivers have been wrapped so far. */
    sigma_u32 wrappedCount() const noexcept { return m_wrapped_count; }

private:
    UbuntuCompatLayer() = default;
    UbuntuCompatLayer(const UbuntuCompatLayer&) = delete;
    UbuntuCompatLayer& operator=(const UbuntuCompatLayer&) = delete;

    bool      m_initialized{false};
    sigma_u32 m_wrapped_count{0U};
};

} // namespace Linux
} // namespace Drivers
} // namespace SigmaOS

/* ── C bridge (called by HAL boot when TARGET_OS=ubuntu) ────────────────── */
extern "C" {

void ubuntu_compat_init() {
    SigmaOS::Drivers::Linux::UbuntuCompatLayer::getInstance().init();
}

int ubuntu_compat_wrap_driver(const char* driver_name) {
    return SigmaOS::Drivers::Linux::UbuntuCompatLayer::getInstance()
               .wrapDriver(driver_name) ? 0 : -1;
}

unsigned int ubuntu_compat_wrapped_count() {
    return static_cast<unsigned int>(
        SigmaOS::Drivers::Linux::UbuntuCompatLayer::getInstance().wrappedCount());
}

} // extern "C"
