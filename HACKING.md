# HACKING.md — Writing a New Sovereign Shard

This guide walks through writing, registering, and testing a new kernel shard for SigmaOS.

## Step 1: Create the Shard File

All shards live in `kernel/core/`. Create `SovereignMyFeature.cpp`:


```cpp

#include "sigma_types.h"

#include "sigma_hal.h"

#include "SovereignLibC.h"

/**

- SovereignMyFeature — What this shard does.

- Algorithm: ALGORITHM_NAME

- USP: What makes this sovereign vs. legacy Linux equivalent.
 */

class SovereignMyFeatureEngine {
public:
    static SovereignMyFeatureEngine& getInstance() {
        static SovereignMyFeatureEngine instance;
        return instance;
    }

    void init() {
        sigma_log("[MYFEATURE] Initializing...");
    }

private:
    SovereignMyFeatureEngine() {}
};

extern "C" void myfeature_init() {
    SovereignMyFeatureEngine::getInstance().init();
}




```

## Step 2: Register in SovereignUSR

After your shard is initialized, register it:


```c

usr_register_shard("SovereignMyFeature", 0x00FF);




```

## Step 3: Run Static Analysis


```bash

cppcheck --enable=warning,style kernel/core/SovereignMyFeature.cpp




```

## Step 4: Build


```bash

python3 tools/sigma-build.py




```

## Step 5: Submit a PR

Ensure your PR description references:

- The `IDEAS_BACKLOG.md` item or `ROADMAP.md` milestone it closes

- The `cppcheck` output (zero warnings)

- A wiki page update in `SigmaOS.wiki/`
