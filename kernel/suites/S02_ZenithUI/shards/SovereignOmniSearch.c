// =============================================================================
// SigmaOS — S02_ZenithUI — SovereignOmniSearch.c
// Universal Cross-Registry / Cross-Hive / Cross-Brain Search
// =============================================================================
// Beyond the Leaders:
//   • Apple Spotlight — Local files, contacts, apps.
//   • Windows Search — Local/Cloud files and web results.
//   • Sigma OmniSearch — SEARCH EVERYTHING. Indexes the Registry v2 (S10), 
//     Hive BlockStore (S06), Mesh Memory (S05), and SoulMolding Traits (S16).
// Result: Instantly find a Registry key, a remote Hive file, or a 
//         cognitive prediction from your own Digital Twin.
// =============================================================================

#include <sigma_types.h>


#define MAX_SEARCH_RESULTS 256

typedef enum {
    SRC_FILE     = 0,
    SRC_REGISTRY = 1,
    SRC_NETWORK  = 2,
    SRC_COGNITIVE= 3
} SearchSource;

typedef struct {
    char         result_path[256];
    SearchSource source;
    float        relevance; // From S13 Sentience
} SearchItem;

// ── Public API ────────────────────────────────────────────────────────────────

// Initialise the OmniSearch engine (Starts background indexing)
void omnisearch_init(void);

// Perform a global Hive search (Bypasses local VFS)
uint32_t omnisearch_query(const char* query, SearchItem* out_results);

// Hook into S10 Registry state-changes for real-time indexing
void omnisearch_hook_registry(void);

// Hook into S16 SoulMolding for cognitive-match search
void omnisearch_hook_brain(void);

// Display results in the spatial Z-Space Compositor (S02)
void omnisearch_project_to_ui(void);

// Sync search indexes across the Hive mesh (Distributed Search S12)
void omnisearch_sync_mesh(void);



