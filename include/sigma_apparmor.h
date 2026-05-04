#ifndef SIGMA_APPARMOR_H
#define SIGMA_APPARMOR_H

#include "sigma_types.h"

#ifdef __cplusplus
namespace SigmaOS {
namespace Kernel {
namespace Security {

class SovereignAppArmor {
public:
    static SovereignAppArmor& getInstance();

    void init();
    void loadProfile(const char* profile_name, const void* rules);
    void audit();

private:
    SovereignAppArmor() : m_active_profiles(0) {}
    sigma_u32 m_active_profiles;
};

} // namespace Security
} // namespace Kernel
} // namespace SigmaOS
#endif

#ifdef __cplusplus
extern "C" {
#endif

void apparmor_init(void);
void apparmor_load_profile(const char* name, const void* rules);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_APPARMOR_H */
