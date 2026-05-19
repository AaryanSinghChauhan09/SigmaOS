#include "libc/sigma_libc.h"
#include "libc/SovereignLibC.h"
#include "sigma_kernel_types.h"
#include "sigma_cap_manager.h"
#include "sigma_log.h"
#include "ai/sigma_claw.h"
#include "security/sigma_sandbox.h"

// Σ SIGMAOS: SOVEREIGN CLAW AUTOMATION (S66)
// Inspired by OpenClaw Architecture
// Responsibility: Autonomous orchestration and intent-driven skill execution.

#ifdef __cplusplus
extern "C" {
#endif

namespace sigma {

struct ClawIntent {
    const char* goal;
    sigma_u32 priority;
    bool requires_sandboxing;
};

class SovereignClaw {
private:
    sigma_claw_sandbox_mode_t m_sandbox_mode = CLAW_SANDBOX_STRICT;
    bool m_voice_wake = false;
    bool m_live_canvas = false;
    bool m_initialized = false;

public:
    static SovereignClaw& getInstance() {
        static SovereignClaw instance;
        return instance;
    }

    void initialize() {
        if (m_initialized) return;
        sigma_printf("[Claw-Agent] Initializing autonomous agent state...\n");
        m_sandbox_mode = CLAW_SANDBOX_STRICT;
        m_voice_wake = true;
        m_live_canvas = true;
        m_initialized = true;
        sigma_printf("[Claw-Agent] Security Sandbox policy set to STRICT. Initialization COMPLETE.\n");
    }

    void set_sandbox_mode(sigma_claw_sandbox_mode_t mode) {
        m_sandbox_mode = mode;
        const char* mode_str = "STRICT";
        if (mode == CLAW_SANDBOX_NON_MAIN) mode_str = "NON_MAIN";
        else if (mode == CLAW_SANDBOX_OPEN) mode_str = "OPEN";
        sigma_printf("[Claw-Agent] Sandbox Policy altered dynamically to: %s\n", mode_str);
    }

    void process_intent(ClawIntent intent) {
        sigma_printf("[Claw] Decomposing Intent: \"%s\"\n", intent.goal);
        
        // Step 1: Reasoning (Goal Decomposition)
        if (sigma_strstr(intent.goal, "Update system")) {
            execute_skill("sigma-snap create \"Auto-Claw Backup\"");
            execute_skill("sigma-update --all");
            execute_skill("sigma-health --audit");
        } else if (sigma_strstr(intent.goal, "Security audit")) {
            execute_skill("sigma-sec attest");
            execute_skill("sigma-sec audit");
        } else {
            sigma_printf("[Claw] Reasoning Layer: Goal not fully understood. Requesting context...\n");
            execute_skill("sigma-sys");
        }
    }

    void execute_skill(const char* skill_cmd) {
        // Step 2: Capability Verification (Zero-Trust)
        auto token = cap_manager.request_token(SIGMA_CAP_EXEC_SKILL);
        
        if (token.is_valid()) {
            sigma_printf("[Claw] Executing Skill: %s (Capability Verified)\n", skill_cmd);
            
            // Execute under sandbox context if policy is strict
            if (m_sandbox_mode == CLAW_SANDBOX_STRICT) {
                sigma_sandbox_config_t config;
                config.container_id = 66;
                config.network_access = false;
                config.fs_access = true;
                config.memory_limit = 512 * 1024 * 1024;

                sandbox_create_container(&config);
                sandbox_execute(66, skill_cmd);
                sandbox_destroy_container(66);
            } else {
                sigma_printf("[Claw] Running skill outside sandbox context.\n");
            }
        } else {
            sigma_printf("[SECURITY] Claw Skill Execution DENIED: Invalid Capability Token.\n");
        }
    }

    void persist_context(const char* key, const char* value) {
        // Step 3: Write-Ahead Memory (Persistence)
        sigma_printf("[Claw] Saving Context: [%s] -> %s\n", key, value);
    }

    void handle_message(const char* channel, const char* message) {
        sigma_printf("[Claw] Routing Message from [%s]: %s\n", channel, message);
        ClawIntent intent;
        intent.goal = message;
        intent.priority = 1;
        intent.requires_sandboxing = true;
        process_intent(intent);
    }

    void render_canvas() {
        if (!m_live_canvas) return;
        sigma_printf("[Claw-Canvas] Rendering Live Agent Workspace frame...\n");
    }
};

} // namespace sigma

/* --- C Linkage Bridging --- */

void claw_gateway_init(void) {
    sigma::SovereignClaw::getInstance().initialize();
}

void claw_route_message(const char* channel, const char* message) {
    sigma::SovereignClaw::getInstance().handle_message(channel, message);
}

void claw_render_canvas(void) {
    sigma::SovereignClaw::getInstance().render_canvas();
}

void claw_execute_tool(const char* tool_name, const char* payload) {
    sigma_printf("[Claw-Tool] Executing tool [%s] with payload [%s]\n", tool_name, payload);
    sigma::SovereignClaw::getInstance().execute_skill(tool_name);
}

void claw_sandbox_policy(sigma_claw_sandbox_mode_t mode) {
    sigma::SovereignClaw::getInstance().set_sandbox_mode(mode);
}

void claw_daemon_init() {
    claw_gateway_init();
    sigma_printf("[S66] Sovereign Claw Gateway Online.\n");
}

#ifdef __cplusplus
}
#endif
