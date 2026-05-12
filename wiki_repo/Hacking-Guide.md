# Hacking-Guide

1

This guide defines the industrial workflow for developing, registering, and validating new **Sovereign Shards** within the SigmaOS lattice.

1

All core shards must adhere to the **Singleton Pattern** to ensure a single source of truth for system state.

1

#include "core/sigma_types.h"

#include "hal/sigma_hal.h"

namespace SigmaOS {
namespace Kernel {
namespace MyModule {

class SovereignMyShard : public SigmaObject {
public:
    static SovereignMyShard& getInstance() {
        static SovereignMyShard instance;
        return instance;
    }
    void init() {
        sigma_log_info("[MYSHARD] Initializing Sovereign Logic...");
    }
private:
    SovereignMyShard() = default;
};

} // namespace MyModule
} // namespace Kernel
} // namespace SigmaOS

1

1

Expose your shard to the C-based kernel initialization sequence via a bridge.

1

extern "C" void myshard_init() {
    SigmaOS::Kernel::MyModule::SovereignMyShard::getInstance().init();
    usr_register_shard("SovereignMyShard", SHARD_ID_GENERIC);
}

1

1

Before submission, every shard must undergo the **Sovereign Audit**:

1

1

1. Create a `feature/` branch.
2. Submit a Pull Request targeting the `main` branch.
3. Ensure the shard is registered in the [Industrial Nexus](Industrial-Nexus) if it provides global USPs.

---
[**? Back to Home**](Home)

