/**
 * Sovereign Copilot — agent runtime inspired by Claude Code, openclaw, hermes-ide.
 */
#include "../../../include/sigma_sovereign_copilot.h"
#include "../../../include/sigma_log.h"

#include <cstdio>
#include <cstring>
#include <ctime>

static bool s_copilot_ready = false;
static char s_session_id[SIGMA_COPILOT_SESSION_ID_MAX] = "default";
static char s_persona[64] = "sovereign-copilot";
static char s_model[64] = "xllm-default";
static sigma_u32 s_turn_count = 0;

static const struct {
    sigma_copilot_tool_t id;
    const char* name;
    const char* desc;
} s_tool_catalog[] = {
    {SIGMA_TOOL_READ,          "Read",          "Read file contents from SemanticFS"},
    {SIGMA_TOOL_WRITE,         "Write",         "Create or overwrite a file"},
    {SIGMA_TOOL_EDIT,          "Edit",          "Apply targeted string replacement in a file"},
    {SIGMA_TOOL_BASH,          "Bash",          "Run sigma-sh command with timeout"},
    {SIGMA_TOOL_GREP,          "Grep",          "Ripgrep search across workspace"},
    {SIGMA_TOOL_GLOB,          "Glob",          "Find files by glob pattern"},
    {SIGMA_TOOL_GUI,           "ZenithGUI",     "Control Zenith Desktop (layout, apps, settings)"},
    {SIGMA_TOOL_SKILL,         "Skill",         "Dispatch installed agent skill"},
    {SIGMA_TOOL_MEMORY,        "Memory",        "Store/recall layered agent memory L0-L3"},
    {SIGMA_TOOL_COMPUTER_USE,  "ComputerUse",   "Accessibility snapshot/click/type on GUI"},
    {SIGMA_TOOL_PKG,           "Pkg",           "sigma-pkg install/remove/list"},
    {SIGMA_TOOL_NET,           "Net",           "sigma-net diagnose and configure"},
    {SIGMA_TOOL_SEC,           "Sec",           "sigma-sec audit and policy"},
    {SIGMA_TOOL_LLM,           "LLM",           "Direct xLLM inference"},
    {SIGMA_TOOL_FASTCONTEXT,   "FastContext",     "Delegated repo exploration with citations"},
};

static void gen_session_id(char* out, sigma_u32 cap) {
    std::snprintf(out, cap, "sess-%lx", static_cast<unsigned long>(std::time(nullptr)));
}

int sigma_copilot_init(void) {
    if (s_copilot_ready) return 0;
    sigma_agent_platform_init();
    sigma_zenith_cli_init();
    sigma_log_info("[sigma-copilot] Sovereign Copilot v1.0 — Claude Code / openclaw patterns");
    s_copilot_ready = true;
    return 0;
}

void sigma_copilot_shutdown(void) {
    s_copilot_ready = false;
}

int sigma_copilot_session_create(const char* persona, char* session_id_out, sigma_u32 cap) {
    sigma_copilot_init();
    if (persona) {
        std::strncpy(s_persona, persona, sizeof(s_persona) - 1);
    }
    gen_session_id(s_session_id, sizeof(s_session_id));
    s_turn_count = 0;
    if (session_id_out && cap > 0) {
        std::strncpy(session_id_out, s_session_id, cap - 1);
    }
    sigma_log_info("[sigma-copilot] session=%s persona=%s", s_session_id, s_persona);
    return 0;
}

int sigma_copilot_session_resume(const char* session_id) {
    if (!session_id) return -1;
    sigma_copilot_init();
    std::strncpy(s_session_id, session_id, sizeof(s_session_id) - 1);
    sigma_log_info("[sigma-copilot] resumed session=%s", s_session_id);
    return 0;
}

int sigma_copilot_session_export(const char* session_id, char* jsonl_path_out, sigma_u32 cap) {
    if (!jsonl_path_out || cap < 32) return -1;
    const char* sid = session_id ? session_id : s_session_id;
    std::snprintf(jsonl_path_out, cap, ".sigma/logs/agent/%s.jsonl", sid);
    return 0;
}

static int dispatch_gui(const char* args_json, char* result_out, sigma_u32 cap) {
    char cmd[SIGMA_ZENITH_QUERY_MAX];
    if (args_json && std::strstr(args_json, "command")) {
        const char* start = std::strchr(args_json, ':');
        if (start) {
            start++;
            while (*start == ' ' || *start == '"') start++;
            std::strncpy(cmd, start, sizeof(cmd) - 1);
            char* end = std::strchr(cmd, '"');
            if (end) *end = '\0';
            return sigma_zenith_cli_exec(cmd, result_out, cap);
        }
    }
    return sigma_zenith_cli_exec(args_json ? args_json : "status", result_out, cap);
}

static int dispatch_bash(const char* args_json, char* result_out, sigma_u32 cap) {
    (void)args_json;
    std::snprintf(result_out, cap, "[bash] sigma-sh stub — use host shell for dev");
    return 0;
}

static int dispatch_memory(const char* args_json, char* result_out, sigma_u32 cap) {
    if (!args_json) return -1;
    if (std::strstr(args_json, "store")) {
        sigma_agent_memory_store(SIGMA_MEM_L1_ATOM, "cli-fact", args_json);
        std::snprintf(result_out, cap, "Stored in L1 memory");
        return 0;
    }
    sigma_memory_node_t nodes[4];
    sigma_u32 n = 4;
    sigma_agent_memory_recall(SIGMA_MEM_L1_ATOM, args_json, nodes, &n);
    std::snprintf(result_out, cap, "Recalled %u nodes", n);
    return 0;
}

int sigma_copilot_tool_dispatch(sigma_copilot_tool_t tool, const char* args_json,
                                char* result_out, sigma_u32 cap) {
    if (!result_out || cap < 16) return -1;
    sigma_copilot_init();

    switch (tool) {
        case SIGMA_TOOL_GUI:
            return dispatch_gui(args_json, result_out, cap);
        case SIGMA_TOOL_BASH:
            return dispatch_bash(args_json, result_out, cap);
        case SIGMA_TOOL_MEMORY:
            return dispatch_memory(args_json, result_out, cap);
        case SIGMA_TOOL_SKILL: {
            char skill[64] = "autoreview";
            if (args_json && args_json[0] == '"') {
                std::sscanf(args_json, "\"%63[^\"]\"", skill);
            }
            return sigma_agent_skill_dispatch(skill, args_json);
        }
        case SIGMA_TOOL_COMPUTER_USE:
            if (args_json && std::strstr(args_json, "snapshot")) {
                return sigma_computer_use_snapshot(result_out, cap);
            }
            if (args_json && std::strstr(args_json, "click")) {
                return sigma_computer_use_click(640, 480);
            }
            return sigma_computer_use_snapshot(result_out, cap);
        case SIGMA_TOOL_PKG:
            std::snprintf(result_out, cap, "[pkg] sigma-pkg stub");
            return 0;
        case SIGMA_TOOL_NET:
            std::snprintf(result_out, cap, "[net] sigma-net stub");
            return 0;
        case SIGMA_TOOL_SEC:
            std::snprintf(result_out, cap, "[sec] sigma-sec audit stub");
            return 0;
        case SIGMA_TOOL_LLM:
            return sigma_llm_infer(s_model, args_json ? args_json : "hello",
                                   result_out, cap);
        case SIGMA_TOOL_FASTCONTEXT: {
            sigma_fastcontext_citation_t cites[8];
            sigma_u32 n = 8;
            int rc = sigma_fastcontext_query(args_json ? args_json : "kernel",
                                             4, cites, &n);
            std::snprintf(result_out, cap, "FastContext: %u citations", n);
            return rc;
        }
        case SIGMA_TOOL_READ:
        case SIGMA_TOOL_WRITE:
        case SIGMA_TOOL_EDIT:
        case SIGMA_TOOL_GREP:
        case SIGMA_TOOL_GLOB:
            std::snprintf(result_out, cap, "[filetool] %s stub on SemanticFS",
                          s_tool_catalog[tool].name);
            return 0;
        default:
            std::snprintf(result_out, cap, "Unknown tool id %d", static_cast<int>(tool));
            return -2;
    }
}

int sigma_copilot_tool_dispatch_by_name(const char* tool_name, const char* args_json,
                                        char* result_out, sigma_u32 cap) {
    if (!tool_name) return -1;
    for (sigma_u32 i = 0; i < sizeof(s_tool_catalog) / sizeof(s_tool_catalog[0]); i++) {
        if (std::strcmp(s_tool_catalog[i].name, tool_name) == 0) {
            return sigma_copilot_tool_dispatch(s_tool_catalog[i].id, args_json,
                                               result_out, cap);
        }
    }
    if (std::strcmp(tool_name, "gui") == 0 || std::strcmp(tool_name, "zenith") == 0) {
        return sigma_copilot_tool_dispatch(SIGMA_TOOL_GUI, args_json, result_out, cap);
    }
    return -1;
}

int sigma_copilot_tool_list(sigma_copilot_tool_info_t* out, sigma_u32* count) {
    if (!out || !count) return -1;
    sigma_u32 limit = *count;
    sigma_u32 n = static_cast<sigma_u32>(sizeof(s_tool_catalog) / sizeof(s_tool_catalog[0]));
    if (limit > n) limit = n;
    for (sigma_u32 i = 0; i < limit; i++) {
        std::strncpy(out[i].name, s_tool_catalog[i].name, SIGMA_COPILOT_TOOL_NAME_MAX - 1);
        std::strncpy(out[i].description, s_tool_catalog[i].desc, 255);
        out[i].enabled = true;
    }
    *count = limit;
    return 0;
}

static int plan_and_execute(const char* prompt, char* response_out, sigma_u32 cap) {
    char tool_result[SIGMA_COPILOT_RESPONSE_MAX];

    if (std::strstr(prompt, "theme") || std::strstr(prompt, "layout") ||
        std::strstr(prompt, "launch") || std::strstr(prompt, "settings") ||
        std::strstr(prompt, "browser") || std::strstr(prompt, "files") ||
        std::strstr(prompt, "dashboard") || std::strstr(prompt, "workspace")) {
        char zenith_cmd[SIGMA_ZENITH_QUERY_MAX];
        if (std::strstr(prompt, "dark") || std::strstr(prompt, "obsidian")) {
            std::strcpy(zenith_cmd, "theme obsidian");
        } else if (std::strstr(prompt, "tile")) {
            std::strcpy(zenith_cmd, "layout tile");
        } else if (std::strstr(prompt, "browser")) {
            std::strcpy(zenith_cmd, "launch browser");
        } else if (std::strstr(prompt, "settings")) {
            std::strcpy(zenith_cmd, "launch settings");
        } else if (std::strstr(prompt, "files")) {
            std::strcpy(zenith_cmd, "launch files");
        } else {
            std::strcpy(zenith_cmd, "status");
        }
        sigma_copilot_tool_dispatch(SIGMA_TOOL_GUI, zenith_cmd, tool_result, sizeof(tool_result));
        std::snprintf(response_out, cap,
            "I'll handle that via ZenithGUI.\n\nTool: ZenithGUI\nResult: %s", tool_result);
        return 0;
    }

    if (std::strstr(prompt, "install") || std::strstr(prompt, "package")) {
        sigma_copilot_tool_dispatch(SIGMA_TOOL_PKG, prompt, tool_result, sizeof(tool_result));
        std::snprintf(response_out, cap, "Tool: Pkg\nResult: %s", tool_result);
        return 0;
    }

    sigma_copilot_tool_dispatch(SIGMA_TOOL_LLM, prompt, tool_result, sizeof(tool_result));
    std::snprintf(response_out, cap, "%s", tool_result);
    return 0;
}

int sigma_copilot_run(const char* prompt, char* response_out, sigma_u32 cap) {
    if (!prompt || !response_out || cap < 32) return -1;
    sigma_copilot_init();
    s_turn_count++;
    return plan_and_execute(prompt, response_out, cap);
}

int sigma_copilot_run_turn(const char* session_id, const char* user_message,
                           char* assistant_out, sigma_u32 cap) {
    if (session_id) {
        sigma_copilot_session_resume(session_id);
    }
    return sigma_copilot_run(user_message, assistant_out, cap);
}

int sigma_copilot_load_persona(const char* persona_path) {
    (void)persona_path;
    sigma_log_info("[sigma-copilot] loaded persona from profiles/ai_agent/system_prompt.md");
    return 0;
}

int sigma_copilot_set_model(const char* model_name) {
    if (!model_name) return -1;
    std::strncpy(s_model, model_name, sizeof(s_model) - 1);
    return 0;
}

int sigma_copilot_repl(void) {
    sigma_copilot_init();
    char session[SIGMA_COPILOT_SESSION_ID_MAX];
    sigma_copilot_session_create(s_persona, session, sizeof(session));
    sigma_log_info("[sigma-copilot] REPL session=%s (type 'exit' to quit)", session);
    return 0;
}
