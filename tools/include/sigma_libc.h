/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN HOST BRIDGE
 * =========================================================================
 * Maps sigma_ calls to standard C for host-based tools (audit, test, wiki).
 * =========================================================================
 */

#ifndef SIGMA_HOST_BRIDGE_H
#define SIGMA_HOST_BRIDGE_H

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "SovereignCommon.h"
#include <stdbool.h>

#define sigma_printf(...)    printf(__VA_ARGS__)
#define sigma_malloc(s)      malloc(s)
#define sigma_free(p)        free(p)
#define sigma_strcmp(a,b)    strcmp(a,b)
#define sigma_strncmp(a,b,n) strncmp(a,b,n)
#define sigma_strlen(s)      strlen(s)
#define sigma_strncpy(d,s,n) strncpy(d,s,n)
#define sigma_memcpy(d,s,n)  memcpy(d,s,n)
#define sigma_memset(d,v,n)  memset(d,v,n)

typedef uint64_t sigma_u64;
typedef uint32_t sigma_u32;
typedef uint8_t  sigma_u8;
typedef size_t   sigma_size_t;
typedef int      sigma_err_t;

#define SIGMA_OK    0
#define SIGMA_NULL  NULL

#endif
