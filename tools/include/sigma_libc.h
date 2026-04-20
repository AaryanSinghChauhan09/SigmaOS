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
#include <stdbool.h>
#include <stdint.h>
#include <unistd.h>
#include "suites/S01_Genesis/shards/SovereignCommon.h"

// I/O Mapping
#define sigma_printf(...)    printf(__VA_ARGS__)
#define sigma_open(p, m)     fopen(p, m)
#define sigma_close(f)       fclose(f)
#define sigma_read(b,s,c,f)  fread(b,s,c,f)
#define sigma_write(b,s,c,f) fwrite(b,s,c,f)
#define sigma_sprintf(s,f,...) sprintf(s,f,##__VA_ARGS__)
#define sigma_snprintf(s,n,f,...) snprintf(s,n,f,##__VA_ARGS__)
#define sigma_fprintf(f,fm,...) fprintf(f,fm,##__VA_ARGS__)
#define sigma_exit(c)        exit(c)
#define sigma_getcwd(b, s)    getcwd(b, s)

// Memory Mapping
#define sigma_malloc(s)      malloc(s)
#define sigma_free(p)        free(p)
#define sigma_memset(d,v,n)  memset(d,v,n)
#define sigma_memcpy(d,s,n)  memcpy(d,s,n)

// String Mapping
#define sigma_strcmp(a,b)    strcmp(a,b)
#define sigma_strncmp(a,b,n) strncmp(a,b,n)
#define sigma_strlen(s)      strlen(s)
#define sigma_strcpy(d,s)    strcpy(d,s)
#define sigma_strncpy(d,s,n) strncpy(d,s,n)
#define sigma_strrchr(s,c)   strrchr(s,c)
#define sigma_strstr(h,n)    strstr(h,n)
#define sigma_strncat(d,s,n) strncat(d,s,n)

// Sovereign Types are derived from SovereignCommon.h
// No redefinitions here to avoid conflicts.

#define SIGMA_OK    0

// Kernel Compatibility Aliases
// Recursive Kernel Compatibility Aliases
#define sigma_sigma_sigma_printf sigma_printf
#define sigma_sigma_printf       sigma_printf
#define sigma_sigma_malloc       sigma_malloc
#define sigma_sigma_free         sigma_free
#define sigma_sigma_memset       sigma_memset
#define sigma_sigma_memcpy       sigma_memcpy
#define sigma_sigma_strlen       sigma_strlen
#define sigma_sigma_strcmp       sigma_strcmp
#define sigma_sigma_strcpy       sigma_strcpy
#define sigma_sigma_strncpy      sigma_strncpy
#define sigma_sigma_strrchr      sigma_strrchr

#endif
