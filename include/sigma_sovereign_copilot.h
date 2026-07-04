/**
 * SigmaOS Sovereign Copilot — Claude Code / openclaw inspired agent runtime.
 * Interactive REPL, tool-use loop, session JSONL, GUI bridge, skill dispatch.
 */
#ifndef SIGMA_SOVEREIGN_COPILOT_H
#define SIGMA_SOVEREIGN_COPILOT_H

#include "sigma_kernel_types.h"
#include "sigma_sovereign_agent.h"
#include "sigma_sovereign_zenith_cli.h"

#ifdef __cplusplus
extern "C" {
#endif

#define SIGMA_COPILOT_SESSION_ID_MAX   64
#define SIGMA_COPILOT_PROMPT_MAX     4096
#define SIGMA_COPILOT_RESPONSE_MAX   8192
#define SIGMA_COPILOT_TOOL_NAME_MAX    64
#define SIGMA_COPILOT_TOOL_ARGS_MAX  2048
#define SIGMA_COPILOT_TOOL_COUNT         16
#define SIGMA_COPILOT_MAX_TURNS          32

typedef enum {
    SIGMA_TOOL_READ = 0,
    SIGMA_TOOL_WRITE,
    SIGMA_TOOL_EDIT,
    SIGMA_TOOL_BASH,
    SIGMA_TOOL_GREP,
    SIGMA_TOOL_GLOB,
    SIGMA_TOOL_GUI,
    SIGMA_TOOL_SKILL,
    SIGMA_TOOL_MEMORY,
    SIGMA_TOOL_COMPUTER_USE,
    SIGMA_TOOL_PKG,
    SIGMA_TOOL_NET,
    SIGMA_TOOL_SEC,
    SIGMA_TOOL_LLM,
    SIGMA_TOOL_FASTCONTEXT
} sigma_copilot_tool_t;

typedef struct {
    char name[SIGMA_COPILOT_TOOL_NAME_MAX];
    char description[256];
    bool enabled;
} sigma_copilot_tool_info_t;

typedef struct {
    char session_id[SIGMA_COPILOT_SESSION_ID_MAX];
    char persona[64];
    sigma_u32 turn_count;
    bool interactive;
} sigma_copilot_session_t;

int sigma_copilot_init(void);
void sigma_copilot_shutdown(void);

/* Session lifecycle (JSONL under .sigma/logs/agent/) */
int sigma_copilot_session_create(const char* persona, char* session_id_out, sigma_u32 cap);
int sigma_copilot_session_resume(const char* session_id);
int sigma_copilot_session_export(const char* session_id, char* jsonl_path_out, sigma_u32 cap);

/* Agent loop */
int sigma_copilot_run(const char* prompt, char* response_out, sigma_u32 cap);
int sigma_copilot_run_turn(const char* session_id, const char* user_message,
                           char* assistant_out, sigma_u32 cap);

/* Tool registry & dispatch (Claude Code tool schema) */
int sigma_copilot_tool_list(sigma_copilot_tool_info_t* out, sigma_u32* count);
int sigma_copilot_tool_dispatch(sigma_copilot_tool_t tool, const char* args_json,
                                char* result_out, sigma_u32 cap);
int sigma_copilot_tool_dispatch_by_name(const char* tool_name, const char* args_json,
                                      char* result_out, sigma_u32 cap);

/* Training / persona */
int sigma_copilot_load_persona(const char* persona_path);
int sigma_copilot_set_model(const char* model_name);

/* Interactive REPL (sigma-agent repl) */
int sigma_copilot_repl(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_SOVEREIGN_COPILOT_H */
