#include "libc/sigma_libc.h"

// ---------------------------------------------------------
// SigmaOS User Authentication & Access Control
// ---------------------------------------------------------

#define MAX_USERS 64
#define MAX_USERNAME_LEN 32
#define MAX_HASH_LEN 64

typedef struct {
    int uid;
    char username[MAX_USERNAME_LEN];
    char password_hash[MAX_HASH_LEN]; // Mock SHA-256 hash
    int role; // 0: Admin, 1: User, 2: Guest
} user_record_t;

static user_record_t user_db[MAX_USERS];
static int num_users = 0;

void auth_init() {
    // Add default root user
    user_db[0].uid = 0;
    strncpy(user_db[0].username, "root", MAX_USERNAME_LEN);
    strncpy(user_db[0].password_hash, "default_hash", MAX_HASH_LEN); // Needs proper hashing
    user_db[0].role = 0;
    num_users = 1;
}

int auth_verify(const char* username, const char* password) {
    // Simple mock verification
    for (int i = 0; i < num_users; i++) {
        // Assume strncmp exists
        if (strncmp(user_db[i].username, username, MAX_USERNAME_LEN) == 0) {
            // In a real system, hash 'password' and compare with password_hash
            return user_db[i].uid; // Success
        }
    }
    return -1; // Fail
}

int auth_check_permission(int uid, int required_role) {
    for (int i = 0; i < num_users; i++) {
        if (user_db[i].uid == uid) {
            if (user_db[i].role <= required_role) {
                return 1; // Permitted
            }
            return 0; // Denied
        }
    }
    return 0; // User not found
}
