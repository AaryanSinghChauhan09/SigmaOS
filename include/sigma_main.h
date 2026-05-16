#ifndef SIGMA_MAIN_H
#define SIGMA_MAIN_H

#include "./sigma_kernel_types.h"

#ifdef __cplusplus
namespace SigmaOS {
namespace Kernel {
namespace System {

class SovereignKernelMain {
public:
    static SovereignKernelMain& getInstance();
    void ignite();

private:
    SovereignKernelMain() {}
};

} // namespace System
} // namespace Kernel
} // namespace SigmaOS
#endif

#ifdef __cplusplus
extern "C" {
#endif

void sigma_kernel_main(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_MAIN_H */
