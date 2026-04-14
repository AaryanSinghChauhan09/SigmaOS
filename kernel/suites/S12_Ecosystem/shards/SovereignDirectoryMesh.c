// =============================================================================
// SigmaOS — S12_Ecosystem — SovereignDirectoryMesh.c
// Decentralized Multi-User & Group Mesh
// =============================================================================
// Competitor USPs Absorbed:
//   • Microsoft Active Directory (AD) — Group policy and user management
//   • LDAP / OpenDirectory — Hierarchical user discovery
//   • Unix Groups / Sudoers — Permission escalation and membership
// Exceeding Competitors:
//   • Mesh-based consensus: No single "Domain Controller" to fail.
//   • Dynamic Group Policy: Automatically adapts S08 ACLs across the Hive.
//   • E2EE Membership: Group rosters are encrypted via PQC (S08).
// =============================================================================

#include <sigma_types.h>


#define MAX_MESH_GROUPS     64
#define MAX_MESH_USERS      1024

typedef struct {
    uint8_t  group_uuid[16];
    char     name[64];
    uint16_t permissions_mask;
    uint32_t member_count;
} MeshGroup;

// ── Public API ────────────────────────────────────────────────────────────────

// Initialise the Directory Mesh engine
void directory_init(void);

// Register a user in the global Hive directory
void directory_register_user(const char* name, uint8_t* bio_key);

// Create a new mesh-wide group (Active Directory parity)
void directory_create_group(const char* group_name);

// Add a user to a group (Synchronizes across S12 peers)
bool directory_add_to_group(uint8_t* user_uuid, uint8_t* group_uuid);

// Check group membership (ZKP-backed for privacy)
bool directory_check_access(uint8_t* user_uuid, uint16_t required_mask);

// Synchronize directory state with S10_Registry and Hive peers
void directory_sync_mesh(void);


