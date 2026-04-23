#!/usr/bin/env node
/**
 * Σ SIGMAOS: SOVEREIGN BUILD REPAIR (v1.0)
 * ===========================================
 * Scans the entire suites/ tree for broken includes and places
 * canonical shim headers in every directory that needs them.
 * Run once: node repair_build.js
 */
const fs = require('fs');
const path = require('path');

// --- Canonical headers content ---
const SIGMA_KERNEL_TYPES_CONTENT = `/* sigma_kernel_types.h — Sovereign canonical shim */
#ifndef SIGMA_KERNEL_TYPES_H
#define SIGMA_KERNEL_TYPES_H
typedef unsigned char      u8;
typedef unsigned short     u16;
typedef unsigned int       u32;
typedef unsigned long long u64;
typedef signed char        i8;
typedef signed short       i16;
typedef signed int         i32;
typedef signed long long   i64;
typedef unsigned long long usize;
typedef long long          isize;
typedef u64                paddr_t;
typedef u64                vaddr_t;
typedef int                bool_t;
typedef unsigned char      sigma_u8;
typedef unsigned short     sigma_u16;
typedef unsigned int       sigma_u32;
typedef unsigned long long sigma_u64;
typedef signed long long   sigma_ssize_t;
typedef unsigned long long sigma_size_t;
typedef int                sigma_bool;
typedef sigma_u32          uint32_t;
typedef sigma_u64          uint64_t;
typedef sigma_u8           uint8_t;
typedef sigma_u16          uint16_t;
#ifndef TRUE
#define TRUE  1
#define FALSE 0
#endif
#ifndef NULL
#define NULL ((void*)0)
#endif
#define PAGE_SIZE    4096ULL
#define PAGE_SHIFT   12u
#define K_OK         0
#define K_ERR_NOMEM -1
#define K_ERR_INVAL -2
typedef int k_status;

/* Jail type for virtualization shards */
typedef struct sigma_jail {
    sigma_u32 id;
    sigma_u32 flags;
    const char* namespace_root;
} sigma_jail_t;

/* Unit type for orchestration shards */
typedef struct sigma_unit {
    const char* name;
    sigma_u32 state;
} sigma_unit_t;

static inline void cpu_halt(void)  { __asm__ __volatile__("cli; hlt"); }
static inline void cpu_pause(void) { __asm__ __volatile__("pause"); }
void sigma_panic(const char* msg, u64 rip, u64 rsp);
#define SIGMA_ASSERT(cond, msg) do { if (!(cond)) sigma_panic(msg, 0, 0); } while(0)
#endif /* SIGMA_KERNEL_TYPES_H */
`;

const SIGMA_LIBC_CONTENT = `/* sigma_libc.h — Sovereign canonical shim */
#ifndef SIGMA_LIBC_H
#define SIGMA_LIBC_H
#include "sigma_kernel_types.h"
void          sigma_exit(int code);
sigma_ssize_t sigma_write(int fd, const void* buf, sigma_size_t count);
sigma_ssize_t sigma_read(int fd, void* buf, sigma_size_t count);
int           sigma_open(const char* filename, int flags, int mode);
int           sigma_close(int fd);
void*         sigma_mmap(void* addr, sigma_size_t length, int prot, int flags, int fd, sigma_u64 offset);
sigma_size_t  sigma_strlen(const char* s);
int           sigma_streq(const char* s1, const char* s2);
void          sigma_print(const char* str);
void          sigma_print_num(sigma_u64 val);
void          sigma_print_hex(sigma_u64 val);
void          sigma_printf(const char* format, ...);
void*         sigma_malloc(sigma_size_t size);
void          sigma_free(void* ptr);
void          sigma_log(const char* msg);
#endif /* SIGMA_LIBC_H */
`;

const SOVEREIGN_LIBC_H_CONTENT = `/* SovereignLibC.h — Sovereign canonical shim */
#ifndef SOVEREIGN_LIBC_H
#define SOVEREIGN_LIBC_H
#include "sigma_kernel_types.h"
#include "sigma_libc.h"
#endif
`;

const SIGMA_OOP_HPP_CONTENT = `/* SigmaOOP.hpp — Sovereign canonical shim */
#ifndef SIGMA_OOP_HPP
#define SIGMA_OOP_HPP
#include "sigma_kernel_types.h"
#include "sigma_libc.h"
namespace SigmaOS {
typedef sigma_u32 sigma_status;
#define SIGMA_OK    0x00000000U
#define SIGMA_ERROR 0xFFFFFFFFU
class SigmaMemory {
public:
    static void* allocate(sigma_u64 length) {
        return sigma_mmap(0, length, 3, 0x22, -1, 0);
    }
};
class SigmaObject {
public:
    virtual ~SigmaObject() = default;
    virtual const char* type_name() const noexcept = 0;
};
inline void sigma_log(const char* msg) {
    ::sigma_print("[SIGMA_LOG]: ");
    ::sigma_print(msg);
    ::sigma_print("\\n");
}
} // namespace SigmaOS
#endif /* SIGMA_OOP_HPP */
`;

const HEADERS = {
    'sigma_kernel_types.h': SIGMA_KERNEL_TYPES_CONTENT,
    'sigma_libc.h':         SIGMA_LIBC_CONTENT,
    'SovereignLibC.h':      SOVEREIGN_LIBC_H_CONTENT,
    'SigmaOOP.hpp':         SIGMA_OOP_HPP_CONTENT,
};

let dirsFixed = 0;
let filesPlaced = 0;

// --- Patterns that need shim headers ---
const NEEDS = {
    'sigma_kernel_types.h': /["<](?:\.\.\/)*sigma_kernel_types\.h[">]/,
    'sigma_libc.h':         /["<](?:\.\.\/)*sigma_libc\.h[">]/,
    'SovereignLibC.h':      /["<](?:\.\.\/)*SovereignLibC\.h[">]/,
    'SigmaOOP.hpp':         /["<](?:\.\.\/)*SigmaOOP\.hpp[">]/,
};

function walkDir(dir) {
    let entries;
    try { entries = fs.readdirSync(dir, { withFileTypes: true }); }
    catch(e) { return; }

    // Check if this dir has any C/C++/ASM sources that need headers
    const needed = new Set();
    for (const e of entries) {
        if (!e.isFile()) continue;
        const ext = path.extname(e.name);
        if (!['.c', '.cpp', '.h', '.hpp'].includes(ext)) continue;
        const content = fs.readFileSync(path.join(dir, e.name), 'utf8');
        for (const [hdr, pattern] of Object.entries(NEEDS)) {
            if (pattern.test(content)) needed.add(hdr);
        }
    }

    if (needed.size > 0) {
        dirsFixed++;
        for (const hdr of needed) {
            const dest = path.join(dir, hdr);
            if (!fs.existsSync(dest)) {
                fs.writeFileSync(dest, HEADERS[hdr]);
                console.log(`  [SHIM] ${dest}`);
                filesPlaced++;
            }
        }
    }

    for (const e of entries) {
        if (e.isDirectory() && !e.name.startsWith('.')) {
            walkDir(path.join(dir, e.name));
        }
    }
}

console.log('Σ://REPAIR> Scanning suites/ for broken includes...');
walkDir('suites');
walkDir('cli');
walkDir('userland');
walkDir('core');
console.log(`Σ://REPAIR> Done. ${dirsFixed} directories patched, ${filesPlaced} shims placed.`);
