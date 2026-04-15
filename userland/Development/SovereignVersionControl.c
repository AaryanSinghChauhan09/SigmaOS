// =============================================================================
// SigmaOS — userland/Development — SovereignVersionControl.c
// Built-in Version Control Integration Shard
// =============================================================================
// Competitor USPs Absorbed:
//   • Git (Linux/OSS) — DAG commit model, content-addressed object store
//   • APFS Snapshots  — instant O(1) volume snapshots as "commits"
//   • Perforce Helix  — atomic changelists, large binary file handling
//   • Pijul            — patch-based theory (conflict-free merges)
// Architecture:
//   • Kernel-level VFS watcher generates change events on each write
//   • Auto-snapshot mode: commit a VFS snapshot on every file close (APFS)
//   • Standard Git protocol compatibility via pack-protocol implementation
//   • Sovereign object store uses S06_Storage block encryption by default
// =============================================================================

#include "suites/S01_Genesis/shards/sigma_types.h"


#define VCS_MAX_REPO_PATH  512
#define VCS_SHA_LEN         20   // SHA-1 / 160-bit object hash (Git compat)
#define VCS_MSG_LEN        256

// ── Commit Object ─────────────────────────────────────────────────────────────
typedef struct {
    uint8_t  hash[VCS_SHA_LEN];
    uint8_t  parent_hash[VCS_SHA_LEN];
    uint8_t  tree_hash[VCS_SHA_LEN];     // Root tree snapshot
    char     author[64];
    uint64_t timestamp_unix;
    char     message[VCS_MSG_LEN];
} VCSCommit;

// ── Repository Handle ─────────────────────────────────────────────────────────
typedef struct {
    char     root_path[VCS_MAX_REPO_PATH];
    uint8_t  head_hash[VCS_SHA_LEN];
    bool     auto_snapshot;  // Commit on every file-close event (APFS model)
    bool     encrypted;      // Encrypt object store via S06_Storage FDE
} VCSRepository;

// ── Public API ────────────────────────────────────────────────────────────────

// Initialise a new sovereign repository at a VFS path
VCSRepository* vcs_init(const char* path, bool encrypted);

// Stage all modified files and create a commit object
VCSCommit* vcs_commit(VCSRepository* repo, const char* message);

// Create a lightweight branch pointer
bool vcs_branch(VCSRepository* repo, const char* branch_name);

// Three-way merge two branches (patch-theory conflict resolution)
bool vcs_merge(VCSRepository* repo, const char* src_branch, const char* dst_branch);

// Push to a remote via SSH (reuses S07 SovereignSSH)
bool vcs_push(VCSRepository* repo, const char* remote_url);

// Enable/disable APFS-style auto-snapshot on file operations
void vcs_set_auto_snapshot(VCSRepository* repo, bool enabled);

