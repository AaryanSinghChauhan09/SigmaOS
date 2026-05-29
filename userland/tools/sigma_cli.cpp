/**
 * SigmaOS modular CLI — profiles, aliases, and automation entrypoints.
 * Competitor inspiration: fish abbreviations, zsh profiles, nushell config.
 */
#include <sigma_libc.h>

#define MAX_ALIASES 64
#define MAX_PROFILES 8

struct SigmaAlias {
    char name[32];
    char expansion[128];
    sigma_bool active;
};

struct SigmaProfile {
    char name[32];
    char theme[32];
    char wm_layout[32];
    sigma_u32 gap_inner;
    sigma_u32 gap_outer;
};

static SigmaAlias g_aliases[MAX_ALIASES];
static sigma_u32 g_alias_count;
static SigmaProfile g_profiles[MAX_PROFILES];
static sigma_u32 g_profile_count;
static sigma_u32 g_active_profile;

static void init_defaults() {
    if (g_profile_count == 0) {
        SigmaProfile p = {};
        sigma_strncpy(p.name, "desktop", 31);
        sigma_strncpy(p.theme, "zenith-dark", 31);
        sigma_strncpy(p.wm_layout, "master-stack", 31);
        p.gap_inner = 4;
        p.gap_outer = 8;
        g_profiles[g_profile_count++] = p;

        SigmaProfile m = {};
        sigma_strncpy(m.name, "minimal", 31);
        sigma_strncpy(m.theme, "zenith-mono", 31);
        sigma_strncpy(m.wm_layout, "monocle", 31);
        m.gap_inner = 0;
        m.gap_outer = 0;
        g_profiles[g_profile_count++] = m;
        g_active_profile = 0;
    }

    if (g_alias_count == 0) {
        SigmaAlias a = {};
        sigma_strncpy(a.name, "ll", 31);
        sigma_strncpy(a.expansion, "sigma-ls -lah", 127);
        a.active = SIGMA_TRUE;
        g_aliases[g_alias_count++] = a;

        SigmaAlias b = {};
        sigma_strncpy(b.name, "pods", 31);
        sigma_strncpy(b.expansion, "sigma-pod list", 127);
        b.active = SIGMA_TRUE;
        g_aliases[g_alias_count++] = b;
    }
}

static void print_usage() {
    sys_print("sigma-cli — modular command/profile tool\n");
    sys_print("Usage:\n");
    sys_print("  sigma-cli profile list|show|use <name>\n");
    sys_print("  sigma-cli alias list|add <name> <cmd>\n");
    sys_print("  sigma-cli update\n");
    sys_print("  sigma-cli branch-check\n");
    sys_print("  sigma-cli automation <backup|update|update-check|recovery-check|wiki-sync>\n");
}

static void profile_list() {
    for (sigma_u32 i = 0; i < g_profile_count; ++i) {
        const char* mark = (i == g_active_profile) ? "*" : " ";
        sys_print("%s %s (theme=%s, layout=%s, gaps=%u/%u)\n",
                  mark, g_profiles[i].name, g_profiles[i].theme,
                  g_profiles[i].wm_layout, g_profiles[i].gap_inner, g_profiles[i].gap_outer);
    }
}

static void profile_use(const char* name) {
    for (sigma_u32 i = 0; i < g_profile_count; ++i) {
        if (sigma_strcmp(g_profiles[i].name, name) == 0) {
            g_active_profile = i;
            sys_print("[cli] Active profile: %s\n", name);
            sys_print("[cli] Apply via ~/.sigma_profile and zenith theme IPC.\n");
            return;
        }
    }
    sys_print("[cli] Unknown profile: %s\n", name);
}

static void alias_list() {
    for (sigma_u32 i = 0; i < g_alias_count; ++i) {
        if (!g_aliases[i].active) continue;
        sys_print("%s -> %s\n", g_aliases[i].name, g_aliases[i].expansion);
    }
}

static void alias_add(const char* name, const char* cmd) {
    if (g_alias_count >= MAX_ALIASES) {
        sys_print("[cli] Alias table full.\n");
        return;
    }
    SigmaAlias a = {};
    sigma_strncpy(a.name, name, 31);
    sigma_strncpy(a.expansion, cmd, 127);
    a.active = SIGMA_TRUE;
    g_aliases[g_alias_count++] = a;
    sys_print("[cli] Alias added: %s -> %s\n", name, cmd);
}

extern "C" int main(int argc, char** argv) {
    init_defaults();

    if (argc < 2) {
        print_usage();
        return 1;
    }

    if (sigma_strcmp(argv[1], "profile") == 0) {
        if (argc < 3) { print_usage(); return 1; }
        if (sigma_strcmp(argv[2], "list") == 0) profile_list();
        else if (sigma_strcmp(argv[2], "show") == 0) profile_list();
        else if (sigma_strcmp(argv[2], "use") == 0 && argc >= 4) profile_use(argv[3]);
        else print_usage();
        return 0;
    }

    if (sigma_strcmp(argv[1], "alias") == 0) {
        if (argc < 3) { print_usage(); return 1; }
        if (sigma_strcmp(argv[2], "list") == 0) alias_list();
        else if (sigma_strcmp(argv[2], "add") == 0 && argc >= 5) alias_add(argv[3], argv[4]);
        else print_usage();
        return 0;
    }

    if (sigma_strcmp(argv[1], "update") == 0) {
        sys_print("[cli] Invoke host: scripts/sigma_automation.sh update\n");
        return 0;
    }

    if (sigma_strcmp(argv[1], "branch-check") == 0) {
        sys_print("[cli] Invoke host: scripts/ci_branch_check.sh\n");
        return 0;
    }

    if (sigma_strcmp(argv[1], "automation") == 0) {
        if (argc < 3) { print_usage(); return 1; }
        sys_print("[cli] Invoke host script: scripts/sigma_automation.sh %s\n", argv[2]);
        return 0;
    }

    print_usage();
    return 1;
}
