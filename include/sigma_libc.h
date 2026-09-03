#ifndef SIGMA_LIBC_H
#define SIGMA_LIBC_H

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>
#include <string.h>
#include <stdio.h>
#include <stdlib.h>

#include "sigma_kernel_types.h"

#define sigma_memset memset
#define sigma_memcpy memcpy
#define sigma_printf printf

#endif /* SIGMA_LIBC_H */
