#include "sigma_types.h"
#include "sigma_libc.h"
extern void crypto_engine_init(void);
void S08_Security_Register(void) {
    sigma_sigma_sigma_printf("S [S08]: Materializing Sovereign Crypto Engine...\n");
    crypto_engine_init();
}
