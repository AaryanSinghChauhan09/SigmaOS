// =============================================================================
// SigmaOS — tools/sovereign_depgraph — sovereign_depgraph.c
// Native C Replacement for scripts/generate_dependency_graph.py
// =============================================================================
// Replaces: scripts/generate_dependency_graph.py
// Competitor USPs Absorbed:
//   • CMake --graphviz  — build-system dep graph output
//   • Bazel query       — target dependency analysis
//   • cargo tree        — Rust crate dependency tree
// Zero external deps — outputs DOT format for Graphviz rendering
// =============================================================================

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <dirent.h>
#include <sys/stat.h>

#define MAX_PATH    1024
#define MAX_NODES   2048
#define MAX_EDGES   8192
#define MAX_NAME     128

typedef struct { char from[MAX_NAME]; char to[MAX_NAME]; } Edge;
static char  nodes[MAX_NODES][MAX_NAME];
static Edge  edges[MAX_EDGES];
static uint32_t node_count = 0, edge_count = 0;

static bool node_exists(const char* name) {
    for (uint32_t i = 0; i < node_count; i++)
        if (strcmp(nodes[i], name) == 0) return true;
    return false;
}

static void add_node(const char* name) {
    if (!node_exists(name) && node_count < MAX_NODES)
        strncpy(nodes[node_count++], name, MAX_NAME - 1);
}

static void add_edge(const char* from, const char* to) {
    if (edge_count < MAX_EDGES) {
        strncpy(edges[edge_count].from, from, MAX_NAME - 1);
        strncpy(edges[edge_count].to,   to,   MAX_NAME - 1);
        edge_count++;
    }
}

// Scan a C file for #include "..." dependencies
static void scan_includes(const char* suite_name, const char* filepath) {
    FILE* f = fopen(filepath, "r");
    if (!f) return;
    char line[512];
    while (fgets(line, sizeof(line), f)) {
        if (strncmp(line, "#include \"", 10) != 0) continue;
        char dep[MAX_NAME] = {0};
        const char* start = line + 10;
        const char* end   = strchr(start, '"');
        if (!end) continue;
        uint32_t len = (uint32_t)(end - start);
        if (len >= MAX_NAME) continue;
        strncpy(dep, start, len);
        add_node(dep);
        add_edge(suite_name, dep);
    }
    fclose(f);
}

static void walk_suite(const char* suites_root, const char* suite) {
    char path[MAX_PATH];
    snprintf(path, sizeof(path), "%s/%s", suites_root, suite);
    DIR* dir = opendir(path);
    if (!dir) return;
    add_node(suite);

    struct dirent* e;
    while ((e = readdir(dir)) != NULL) {
        if (e->d_name[0] == '.') continue;
        const char* ext = strrchr(e->d_name, '.');
        if (!ext) continue;
        if (strcmp(ext, ".c") != 0 && strcmp(ext, ".h") != 0) continue;
        char fpath[MAX_PATH];
        snprintf(fpath, sizeof(fpath), "%s/%s", path, e->d_name);
        scan_includes(suite, fpath);
    }
    closedir(dir);
}

static void emit_dot(const char* out_path) {
    FILE* f = fopen(out_path, "w");
    if (!f) { fprintf(stderr, "Cannot open %s\n", out_path); return; }
    fprintf(f, "digraph SigmaOS {\n");
    fprintf(f, "  rankdir=LR;\n");
    fprintf(f, "  node [shape=box, style=filled, fillcolor=\"#1a1a2e\", "
               "fontcolor=white, fontname=\"Courier\"];\n");
    fprintf(f, "  edge [color=\"#e94560\"];\n\n");
    for (uint32_t i = 0; i < node_count; i++)
        fprintf(f, "  \"%s\";\n", nodes[i]);
    fprintf(f, "\n");
    for (uint32_t i = 0; i < edge_count; i++)
        fprintf(f, "  \"%s\" -> \"%s\";\n", edges[i].from, edges[i].to);
    fprintf(f, "}\n");
    fclose(f);
    printf("[sigma-depgraph] %u nodes, %u edges → %s\n",
           node_count, edge_count, out_path);
}

int main(int argc, char* argv[]) {
    const char* root   = argc > 1 ? argv[1] : "kernel/suites";
    const char* output = argc > 2 ? argv[2] : "SHARD_GRAPH.dot";
    const char* suites[] = {
        "S01_Genesis","S02_ZenithUI","S03_Orchestrator","S04_HAL",
        "S05_Memory","S06_Storage","S07_Network","S08_Security",
        "S09_Intelligence","S10_System","S10_Registry", NULL
    };
    for (int i = 0; suites[i]; i++) walk_suite(root, suites[i]);
    emit_dot(output);
    return 0;
}
