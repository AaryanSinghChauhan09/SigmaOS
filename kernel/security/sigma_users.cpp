/*
 * Σ SigmaOS — sigma_users: Sovereign User & Group Management
 * Zero-Dependency: No shadow-utils, no PAM.
 * Absorbs: Linux /etc/passwd + /etc/shadow + /etc/group model.
 */

extern "C" void sigma_vga_printf(const char* fmt, ...);

typedef unsigned int u32;

#define MAX_USERS  128
#define MAX_GROUPS  64

struct SigmaUser {
    int  uid;
    int  gid;
    char username[32];
    char home_dir[64];
    char shell[32];
    u32  password_hash;
    bool is_active;
};

struct SigmaGroup {
    int  gid;
    char groupname[32];
    int  member_uids[16];
    int  member_count;
};

static SigmaUser  user_table[MAX_USERS];
static SigmaGroup group_table[MAX_GROUPS];
static int user_count  = 0;
static int group_count = 0;

static void str_copy(char* dst, const char* src, int max) {
    int i = 0;
    while (src[i] && i < max - 1) { dst[i] = src[i]; i++; }
    dst[i] = '\0';
}

static int str_eq(const char* a, const char* b) {
    int i = 0;
    while (a[i] && b[i]) { if (a[i] != b[i]) return 0; i++; }
    return a[i] == b[i];
}

// Simple sovereign hash (DJB2 variant)
static u32 sigma_hash_password(const char* password) {
    u32 hash = 5381;
    int i = 0;
    while (password[i]) {
        hash = ((hash << 5) + hash) ^ password[i];
        i++;
    }
    return hash;
}

extern "C" int sigma_useradd(const char* username, const char* password) {
    if (user_count >= MAX_USERS) {
        sigma_vga_printf("[useradd] ERROR: User table full.\n");
        return -1;
    }

    for (int i = 0; i < user_count; i++) {
        if (str_eq(user_table[i].username, username)) {
            sigma_vga_printf("[useradd] ERROR: User '%s' already exists.\n", username);
            return -1;
        }
    }

    SigmaUser* u = &user_table[user_count];
    u->uid = 1000 + user_count;
    u->gid = u->uid;
    str_copy(u->username, username, 32);
    u->password_hash = sigma_hash_password(password);
    u->is_active = true;

    // Build home dir path: /home/<username>
    str_copy(u->home_dir, "/home/", 64);
    int len = 6; int j = 0;
    while (username[j] && len < 63) { u->home_dir[len++] = username[j++]; }
    u->home_dir[len] = '\0';

    str_copy(u->shell, "/bin/sigma_sh", 32);

    sigma_vga_printf("[useradd] Created user '%s' (uid=%d, home=%s)\n", u->username, u->uid, u->home_dir);
    user_count++;
    return 0;
}

extern "C" int sigma_userdel(const char* username) {
    for (int i = 0; i < user_count; i++) {
        if (str_eq(user_table[i].username, username)) {
            sigma_vga_printf("[userdel] Removing user '%s' (uid=%d)\n", username, user_table[i].uid);
            user_table[i].is_active = false;
            return 0;
        }
    }
    sigma_vga_printf("[userdel] ERROR: User '%s' not found.\n", username);
    return -1;
}

extern "C" int sigma_authenticate(const char* username, const char* password) {
    u32 hash = sigma_hash_password(password);
    for (int i = 0; i < user_count; i++) {
        if (str_eq(user_table[i].username, username) && user_table[i].is_active) {
            if (user_table[i].password_hash == hash) {
                sigma_vga_printf("[auth] Authentication successful for '%s'\n", username);
                return user_table[i].uid;
            }
            sigma_vga_printf("[auth] Authentication FAILED for '%s'\n", username);
            return -1;
        }
    }
    sigma_vga_printf("[auth] User '%s' not found.\n", username);
    return -1;
}

extern "C" int sigma_groupadd(const char* groupname) {
    if (group_count >= MAX_GROUPS) return -1;
    SigmaGroup* g = &group_table[group_count];
    g->gid = 1000 + group_count;
    str_copy(g->groupname, groupname, 32);
    g->member_count = 0;
    sigma_vga_printf("[groupadd] Created group '%s' (gid=%d)\n", g->groupname, g->gid);
    group_count++;
    return 0;
}

extern "C" int sigma_who_main(int argc, char** argv) {
    sigma_vga_printf("Active users:\n");
    for (int i = 0; i < user_count; i++) {
        if (user_table[i].is_active) {
            sigma_vga_printf("  %s  uid=%d  home=%s  shell=%s\n",
                user_table[i].username, user_table[i].uid,
                user_table[i].home_dir, user_table[i].shell);
        }
    }
    return 0;
}
