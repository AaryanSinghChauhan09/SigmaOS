#ifndef SOVEREIGN_MULTI_USER_HPP
#define SOVEREIGN_MULTI_USER_HPP

#include "../../../include/sigma_types.h"

typedef struct {
    sigma_u32 uid;
    sigma_u32 gid;
    char username[32];
    sigma_u64 capability_mask;
} sigma_user_t;

class SovereignMultiUserEngine {
public:
    static SovereignMultiUserEngine& getInstance();
    void init();
    sigma_u32 registerUser(sigma_u32 uid, sigma_u32 gid, const char* username, sigma_u64 caps);
    bool authenticate(sigma_u32 uid, const char* username);

private:
    SovereignMultiUserEngine() : user_count(0) {}
    sigma_user_t users[64];
    sigma_u32 user_count;
};

extern "C" {
    void multiuser_init();
    sigma_u32 multiuser_register(sigma_u32 uid, sigma_u32 gid, const char* username, sigma_u64 caps);
    bool multiuser_authenticate(sigma_u32 uid, const char* username);
}

#endif
