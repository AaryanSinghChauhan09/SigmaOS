// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * =========================================================================
 * Σ SIGMAOS: SIGMA-AGENT CLI
 * =========================================================================
 * Sovereign AI agent for SigmaOS — Claude Code / openclaw / hermes-ide inspired.
 * Performs GUI tasks via CLI using the Zenith bridge, tool-use loop, and skills.
 *
 * Usage:
 *   sigma-agent chat "switch to tile layout and open settings"
 *   sigma-agent gui status
 *   sigma-agent gui "launch browser"
 *   sigma-agent tools
 *   sigma-agent skill list
 *   sigma-agent skill run zenith-gui '{"command":"theme cyber"}'
 *   sigma-agent session new
 *   sigma-agent repl
 * =========================================================================
 */
#include <sigma_stdio.h>
#include <cstring>
#include "../../../include/sigma_sovereign_copilot.h"
#include "../../../include/sigma_sovereign_agent.h"

static void print_usage(void) {
    sigma_printf("sigma-agent — Sovereign Copilot CLI\n\n");
    sigma_printf("Usage:\n");
    sigma_printf("  sigma-agent chat <prompt>          Natural language OS task\n");
    sigma_printf("  sigma-agent gui <zenith-command>   Direct Zenith GUI control\n");
    sigma_printf("  sigma-agent gui --list             List GUI commands\n");
    sigma_printf("  sigma-agent tools                  List agent tools\n");
    sigma_printf("  sigma-agent tool <name> <json>     Dispatch a tool\n");
    sigma_printf("  sigma-agent skill list             List installed skills\n");
    sigma_printf("  sigma-agent skill run <name> [json]\n");
    sigma_printf("  sigma-agent session new|export     Session management\n");
    sigma_printf("  sigma-agent repl                   Interactive agent loop\n");
    sigma_printf("  sigma-agent status                 Platform health\n");
    sigma_printf("\nGUI commands (sigma-agent gui ...):\n");
    sigma_printf("  start | stop | status | layout <mosaic|tile|stack>\n");
    sigma_printf("  theme <obsidian|cyber|paper> | workspace <n>\n");
    sigma_printf("  launch <settings|files|browser|terminal|dashboard>\n");
    sigma_printf("  apps | settings get|set|list | files search|tree|open\n");
    sigma_printf("  browser open <url> | dashboard | dashboard query <text>\n");
}

static void print_gui_help(void) {
    sigma_printf("Zenith GUI CLI bridge — maps GUI apps to commands:\n");
    sigma_printf("  Compositor:  start, stop, status, layout, theme, workspace\n");
    sigma_printf("  Apps:        launch, close, apps\n");
    sigma_printf("  Settings:    settings list | settings get <key> | settings set <k> <v>\n");
    sigma_printf("  Files:       files search <q> | files tree [path] | files open <path>\n");
    sigma_printf("  Browser:     browser open <url>\n");
    sigma_printf("  Dashboard:   dashboard | dashboard query <prompt>\n");
}

static int cmd_status(void) {
    char json[512];
    sigma_copilot_init();
    sigma_zenith_status(json, sizeof(json));
    sigma_printf("Sigma Agent Platform: online\n");
    sigma_printf("Zenith: %s\n", json);
    return 0;
}

static int cmd_chat(int argc, char** argv) {
    if (argc < 3) {
        sigma_printf("Usage: sigma-agent chat <prompt>\n");
        return 1;
    }
    char prompt[SIGMA_COPILOT_PROMPT_MAX];
    prompt[0] = '\0';
    for (int i = 2; i < argc; i++) {
        if (prompt[0]) std::strcat(prompt, " ");
        std::strncat(prompt, argv[i], sizeof(prompt) - std::strlen(prompt) - 1);
    }
    char response[SIGMA_COPILOT_RESPONSE_MAX];
    if (sigma_copilot_run(prompt, response, sizeof(response)) != 0) {
        sigma_printf("Agent error\n");
        return 1;
    }
    sigma_printf("%s\n", response);
    return 0;
}

static int cmd_gui(int argc, char** argv) {
    if (argc < 3) {
        print_gui_help();
        return 1;
    }
    if (sigma_strcmp(argv[2], "--list") == 0 || sigma_strcmp(argv[2], "help") == 0) {
        print_gui_help();
        return 0;
    }
    char cmd_line[SIGMA_ZENITH_QUERY_MAX];
    cmd_line[0] = '\0';
    for (int i = 2; i < argc; i++) {
        if (cmd_line[0]) std::strcat(cmd_line, " ");
        std::strncat(cmd_line, argv[i], sizeof(cmd_line) - std::strlen(cmd_line) - 1);
    }
    char output[SIGMA_ZENITH_OUTPUT_MAX];
    int rc = sigma_zenith_cli_exec(cmd_line, output, sizeof(output));
    sigma_printf("%s\n", output);
    return rc == 0 ? 0 : 1;
}

static int cmd_tools(void) {
    sigma_copilot_tool_info_t tools[SIGMA_COPILOT_TOOL_COUNT];
    sigma_u32 n = SIGMA_COPILOT_TOOL_COUNT;
    sigma_copilot_tool_list(tools, &n);
    sigma_printf("Agent tools (%u):\n", n);
    for (sigma_u32 i = 0; i < n; i++) {
        sigma_printf("  %-14s %s\n", tools[i].name, tools[i].description);
    }
    return 0;
}

static int cmd_tool(int argc, char** argv) {
    if (argc < 3) {
        sigma_printf("Usage: sigma-agent tool <name> [args-json]\n");
        return 1;
    }
    const char* args = (argc > 3) ? argv[3] : "{}";
    char result[SIGMA_COPILOT_RESPONSE_MAX];
    int rc = sigma_copilot_tool_dispatch_by_name(argv[2], args, result, sizeof(result));
    sigma_printf("%s\n", result);
    return rc == 0 ? 0 : 1;
}

static int cmd_skill(int argc, char** argv) {
    if (argc < 3) {
        sigma_printf("Usage: sigma-agent skill list|run <name> [json]\n");
        return 1;
    }
    if (sigma_strcmp(argv[2], "list") == 0) {
        sigma_agent_skill_t skills[SIGMA_SKILL_MAX];
        sigma_u32 n = SIGMA_SKILL_MAX;
        sigma_agent_skill_list(skills, &n);
        sigma_printf("Skills (%u):\n", n);
        for (sigma_u32 i = 0; i < n; i++) {
            sigma_printf("  %-20s %s %s\n", skills[i].name, skills[i].description,
                         skills[i].installed ? "[installed]" : "");
        }
        return 0;
    }
    if (sigma_strcmp(argv[2], "run") == 0 && argc >= 4) {
        const char* args = (argc > 4) ? argv[4] : "{}";
        int rc = sigma_agent_skill_dispatch(argv[3], args);
        sigma_printf("Skill dispatch %s: %s\n", argv[3], rc == 0 ? "ok" : "failed");
        return rc == 0 ? 0 : 1;
    }
    return 1;
}

static int cmd_session(int argc, char** argv) {
    if (argc < 3) {
        sigma_printf("Usage: sigma-agent session new|export\n");
        return 1;
    }
    if (sigma_strcmp(argv[2], "new") == 0) {
        char sid[SIGMA_COPILOT_SESSION_ID_MAX];
        sigma_copilot_session_create("sovereign-copilot", sid, sizeof(sid));
        sigma_printf("Session: %s\n", sid);
        return 0;
    }
    if (sigma_strcmp(argv[2], "export") == 0) {
        char path[256];
        sigma_copilot_session_export(nullptr, path, sizeof(path));
        sigma_printf("Export: %s\n", path);
        return 0;
    }
    return 1;
}

int main(int argc, char** argv) {
    sigma_printf("==========================================\n");
    sigma_printf(" Σ SIGMA-AGENT — Sovereign Copilot CLI\n");
    sigma_printf("==========================================\n");

    if (argc < 2) {
        print_usage();
        return 1;
    }

    if (sigma_strcmp(argv[1], "chat") == 0)    return cmd_chat(argc, argv);
    if (sigma_strcmp(argv[1], "gui") == 0)     return cmd_gui(argc, argv);
    if (sigma_strcmp(argv[1], "tools") == 0)   return cmd_tools();
    if (sigma_strcmp(argv[1], "tool") == 0)    return cmd_tool(argc, argv);
    if (sigma_strcmp(argv[1], "skill") == 0)   return cmd_skill(argc, argv);
    if (sigma_strcmp(argv[1], "session") == 0) return cmd_session(argc, argv);
    if (sigma_strcmp(argv[1], "repl") == 0) {
        sigma_copilot_repl();
        sigma_printf("REPL ready — use sigma-agent chat for single-turn on host builds.\n");
        return 0;
    }
    if (sigma_strcmp(argv[1], "status") == 0)  return cmd_status();
    if (sigma_strcmp(argv[1], "help") == 0 || sigma_strcmp(argv[1], "--help") == 0) {
        print_usage();
        return 0;
    }

    sigma_printf("Unknown command: %s\n", argv[1]);
    print_usage();
    return 1;
}
