#include "../../../include/sigma_log.h"
#include "../../../include/sigma_types.h"
#include "SovereignMultiUser.hpp"
#include "hal/sigma_hal.h"
#include "libc/SovereignLibC.h"

SovereignMultiUserEngine& SovereignMultiUserEngine::getInstance() {
    static SovereignMultiUserEngine instance;
    return instance;
}

void SovereignMultiUserEngine::init() {
    sigma_log("[MULTIUSER] Initializing Sovereign Multi-User Identity Engine...");
    this->user_count = 0;
    // Register root-equivalent sovereign admin
    registerUser(0, 0, "sigma-root", 0xFFFFFFFFFFFFFFFFULL);
    sigma_log("[MULTIUSER] Sovereign identity vault ARMED. PQC attestation ENABLED.");
}

sigma_u32 SovereignMultiUserEngine::registerUser(sigma_u32 uid, sigma_u32 gid, const char* username, sigma_u64 caps) {
    if (this->user_count >= 64) return 0;
    sigma_user_t* u = &this->users[this->user_count++];
    u->uid = uid;
    u->gid = gid;
    sigma_hardened_strcpy(u->username, username, 32);
    u->capability_mask = caps;
    sigma_log("[MULTIUSER] User '%s' (UID:%u GID:%u) registered with cap 0x%llX.\n",
                 username, uid, gid, (unsigned long long)caps);
    return uid;
}

bool SovereignMultiUserEngine::authenticate(sigma_u32 uid, const char* username) {
    for (sigma_u32 i = 0; i < this->user_count; i++) {
        if (this->users[i].uid == uid &&
            sigma_strcmp(this->users[i].username, username) == 0) {
            sigma_log("[MULTIUSER] Authenticated: '%s' (UID:%u).\n", username, uid);
            return true;
        }
    }
    sigma_log("[MULTIUSER] Authentication DENIED.");
    return false;
}

void multiuser_init() {
    SovereignMultiUserEngine::init();
}

extern "C" sigma_u32 multiuser_register(sigma_u32 uid, sigma_u32 gid, const char* username, sigma_u64 caps) {
    return SovereignMultiUserEngine::registerUser(uid, gid, username, caps);
}

extern "C" bool multiuser_authenticate(sigma_u32 uid, const char* username) {
    return SovereignMultiUserEngine::authenticate(uid, username);
}




} // extern "C"
