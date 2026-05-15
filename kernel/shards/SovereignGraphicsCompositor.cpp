#include "../../include/Lattice.h"
#include "../../include/sigma_log.h"
#include "../../include/libc/SovereignLibC.h"
#include "../../include/sigma_log.h"
/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN GRAPHICS COMPOSITOR (v21.0)
 * =========================================================================
 * Refactored into modular graphics shards for high-performance visualization.
 * =========================================================================
 */

#include "kernel/drivers/graphics/compositor.hpp"
#include "../../include/sigma_log.h"
#include "kernel/drivers/graphics/widget_orchestrator.hpp"
#include "../../include/sigma_log.h"

extern "C" void _start(void) {
    SigmaOS::Graphics::SovereignGraphicsCompositor compositor;
    SigmaOS::Graphics::SovereignWidgetOrchestrator widgets;

    compositor.CommitFrameShard("DESKTOP_MAIN", "0xFF00FF");
    compositor.ExecuteAlphaBlend("NOTIFICATION_LAYER");
    compositor.ApplyGlassmorphism("TASKBAR_SHARD", 20);
    compositor.RasterizeSpringMotion("APP_LAUNCHER_VFX");
    
    widgets.ProjectToZenithUI();
    
    sigma_exit(0);
}

int main() {
    sigma_log_info("[SIGMA_GRAPHICS]: Initiating Sovereign Compositor Nexus...\n");
    _start();
    return 0;
}


