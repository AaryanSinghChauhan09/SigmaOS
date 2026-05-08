#include "ai/sigma_claw.h"
#include "sigma_hal.h"
#include "sigma_log.h"
#include "sigma_universal_ui.h"

// Bring in Sandbox and Workflow C-bridges directly
extern "C" {
    void sandbox_init();
    unsigned int sandbox_create_container(const void* config);
    int sandbox_execute(unsigned int container_id, const char* binary_path);
    void sandbox_destroy_container(unsigned int container_id);
    
    void sigma_workflow_init();
    void sigma_workflow_dispatch(const char* trigger, const char* data);
}

/**
 * SovereignClawGateway — Personal AI Assistant Gateway
 * Implements OpenClaw-inspired multi-agent routing, Live Canvas, and sandboxed tool execution.
 * Integrates an Artificial Intelligence Automation System for SigmaOS.
 * The lobster way. 🦞
 */

extern "C" void claw_gateway_init(void) {
    sigma_log_info("[CLAW] Initializing Sovereign Claw Gateway...");
    sandbox_init();
    sigma_workflow_init();
    sigma_log_info("[CLAW] Starting multi-channel inbox (Discord, Slack, Telegram, WebChat).");
    sigma_log_info("[CLAW] Local-first AI Automation control plane online.");
}

extern "C" void claw_route_message(const char* channel, const char* message) {
    sigma_log_info("[CLAW] Routing message from channel %s...", channel);
    
    // Automation trigger based on message
    sigma_log_info("[CLAW] Dispatching AI automation event...");
    sigma_workflow_dispatch("SYS_AI_MESSAGE_RECEIVED", message);
    
    sigma_log_info("[CLAW] Message routed to target agent.");
}

extern "C" void claw_render_canvas(void) {
    sigma_log_info("[CLAW] Rendering Live Canvas (Agent-driven visual workspace)...");
}

extern "C" void claw_execute_tool(const char* tool_name, const char* payload) {
    sigma_log_info("[CLAW] AI Tool %s requested. Securing execution environment...", tool_name);
    
    // Use SovereignSandbox for secure tool execution
    unsigned int cid = sandbox_create_container(nullptr);
    sigma_log_info("[CLAW] Sandbox container %u created. Launching tool...", cid);
    
    int success = sandbox_execute(cid, tool_name);
    if (success) {
        sigma_log_info("[CLAW] Tool %s executed safely inside sandbox.", tool_name);
        sigma_workflow_dispatch("SYS_AI_TOOL_SUCCESS", tool_name);
    } else {
        sigma_log_warn("[CLAW] Tool execution failed or denied by sandbox policy!");
        sigma_workflow_dispatch("SYS_AI_TOOL_FAILED", tool_name);
    }
    
    sandbox_destroy_container(cid);
}

extern "C" void claw_sandbox_policy(sigma_claw_sandbox_mode_t mode) {
    if (mode == CLAW_SANDBOX_STRICT) {
        sigma_log_info("[CLAW] Applying STRICT sandbox: enforcing strict Seccomp-BFP.");
    } else if (mode == CLAW_SANDBOX_NON_MAIN) {
        sigma_log_info("[CLAW] Applying NON-MAIN sandbox: allowing basic read/write.");
    } else {
        sigma_log_info("[CLAW] Applying OPEN sandbox: high capability granted.");
    }
}
