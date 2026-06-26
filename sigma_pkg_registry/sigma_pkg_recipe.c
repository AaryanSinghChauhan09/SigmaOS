/*
 * Sovereign .spkg recipe loader (NixOS/SlackBuilds-inspired declarative builds).
 */
#include "../include/sigma_kernel_types.h"
#include "../include/security/sigma_pkg_registry.h"

#define MAX_RECIPE_LINE 256
#define MAX_RECIPE_STEPS 8

typedef struct {
    char name[64];
    char version[16];
    char hash[72];
    CurationLevel_t curation;
    char steps[MAX_RECIPE_STEPS][128];
    sigma_u32 step_count;
    sigma_bool requires_network;
    sigma_u32 memory_mb;
} sigma_recipe_t;

static sigma_recipe_t g_last_recipe;

static void trim(char* s) {
    if (!s) return;
    while (*s == ' ' || *s == '\t') sigma_memmove(s, s + 1, sigma_strlen(s));
}

static void parse_line(const char* line) {
    char key[64];
    char value[192];
    sigma_u32 i = 0, k = 0;
    while (line[i] && line[i] != '=' && k < 63) key[k++] = line[i++];
    key[k] = '\0';
    if (line[i] != '=') return;
    i++;
    sigma_u32 v = 0;
    while (line[i] && v < 191) value[v++] = line[i++];
    value[v] = '\0';
    trim(key);
    trim(value);

    if (sigma_strcmp(key, "name") == 0) sigma_strncpy(g_last_recipe.name, value, 63);
    else if (sigma_strcmp(key, "version") == 0) sigma_strncpy(g_last_recipe.version, value, 15);
    else if (sigma_strncmp(key, "hash", 4) == 0) sigma_strncpy(g_last_recipe.hash, value, 71);
    else if (sigma_strcmp(key, "curation") == 0) {
        if (sigma_strcmp(value, "official") == 0) g_last_recipe.curation = CURATION_OFFICIAL;
        else if (sigma_strcmp(value, "community") == 0) g_last_recipe.curation = CURATION_COMMUNITY;
    } else if (sigma_strcmp(key, "steps") == 0 && g_last_recipe.step_count < MAX_RECIPE_STEPS) {
        sigma_strncpy(g_last_recipe.steps[g_last_recipe.step_count++], value, 127);
    } else if (sigma_strcmp(key, "requires_network") == 0) {
        g_last_recipe.requires_network = (value[0] == 't' || value[0] == '1');
    } else if (sigma_strcmp(key, "memory_mb") == 0) {
        g_last_recipe.memory_mb = (sigma_u32)sigma_atoi(value);
    }
}

int sigma_pkg_recipe_load_buffer(const char* text) {
    sigma_memset(&g_last_recipe, 0, sizeof(g_last_recipe));
    g_last_recipe.curation = CURATION_UNVERIFIED;
    if (!text) return -1;
    char line[MAX_RECIPE_LINE];
    sigma_u32 li = 0, ti = 0;
    while (text[ti]) {
        if (text[ti] == '\n' || text[ti] == '\r') {
            line[li] = '\0';
            if (li > 0 && line[0] != '#') parse_line(line);
            li = 0;
            ti++;
            continue;
        }
        if (li < MAX_RECIPE_LINE - 1) line[li++] = text[ti];
        ti++;
    }
    if (li > 0) {
        line[li] = '\0';
        if (line[0] != '#') parse_line(line);
    }
    if (g_last_recipe.name[0]) {
        SovereignPkg_Register(g_last_recipe.name, g_last_recipe.version, g_last_recipe.curation);
        return 0;
    }
    return -1;
}

const char* sigma_pkg_recipe_last_name(void) {
    return g_last_recipe.name[0] ? g_last_recipe.name : SIGMA_NULL;
}
