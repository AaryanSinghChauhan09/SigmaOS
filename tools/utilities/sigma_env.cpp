/*
 * Σ SigmaOS — sigma_env: Sovereign Environment Inspector & Runner
 * Absorbs: GNU coreutils env(1), util-linux printenv, BusyBox env
 * Features: print environment variables, set/unset vars, run command with env
 * Zero-Dependency: No libc. Sovereign static key-value store.
 */

typedef unsigned char      u8;
typedef unsigned short     u16;
typedef unsigned int       u32;
typedef unsigned long long u64;

extern "C" void sigma_vga_puts(const char* s);
extern "C" void sigma_vga_putchar(char c);
extern "C" void sigma_vga_printf(const char* fmt, ...);

/* ─────────────── Sovereign Environment Store ─────────────── */
/* Inspired by Plan 9 and Alpine's minimal /etc/environment approach */

#define ENV_MAX_VARS  128
#define ENV_KEY_LEN   64
#define ENV_VAL_LEN   256

struct EnvVar {
    char key[ENV_KEY_LEN];
    char val[ENV_VAL_LEN];
    bool used;
};

static EnvVar env_store[ENV_MAX_VARS];
static bool   env_initialized = false;

/* ─────────────── String Helpers ─────────────── */
static u32 ev_strlen(const char* s) { u32 n=0; while(s[n]) n++; return n; }
static bool ev_streq(const char* a, const char* b) {
    while (*a && *b && *a==*b) { a++; b++; } return *a==*b;
}
static void ev_strcpy(char* d, const char* s, u32 max) {
    u32 i=0; while(i<max-1 && s[i]) { d[i]=s[i]; i++; } d[i]='\0';
}

/* ─────────────── Default Environment (SigmaOS bootstrap) ─────────────── */
static void env_bootstrap() {
    if (env_initialized) return;
    env_initialized = true;

    /* Pre-load standard SigmaOS environment variables */
    static const char* defaults[][2] = {
        { "SIGMA_VERSION", "1.0.0-zenith"   },
        { "SIGMA_ARCH",    "x86_64"          },
        { "SIGMA_KERNEL",  "sigma-kernel"    },
        { "SIGMA_SHELL",   "/bin/sigma-sh"   },
        { "PATH",          "/bin:/sbin:/usr/bin" },
        { "HOME",          "/root"           },
        { "USER",          "root"            },
        { "HOSTNAME",      "sigmaos"         },
        { "TERM",          "sigma-vt"        },
        { "LANG",          "en_US.UTF-8"     },
        { "LC_ALL",        "C"               },
        { "PWD",           "/"               },
        { "EDITOR",        "sigma-ed"        },
        { "PAGER",         "sigma-less"      },
        { "TMPDIR",        "/tmp"            },
        { nullptr, nullptr }
    };

    u32 idx = 0;
    for (u32 i = 0; defaults[i][0] != nullptr && idx < ENV_MAX_VARS; i++) {
        env_store[idx].used = true;
        ev_strcpy(env_store[idx].key, defaults[i][0], ENV_KEY_LEN);
        ev_strcpy(env_store[idx].val, defaults[i][1], ENV_VAL_LEN);
        idx++;
    }
}

/* ─────────────── Public API ─────────────── */
extern "C" const char* sigma_env_get(const char* key) {
    env_bootstrap();
    for (u32 i = 0; i < ENV_MAX_VARS; i++) {
        if (env_store[i].used && ev_streq(env_store[i].key, key))
            return env_store[i].val;
    }
    return nullptr;
}

extern "C" bool sigma_env_set(const char* key, const char* val) {
    env_bootstrap();
    /* Update existing */
    for (u32 i = 0; i < ENV_MAX_VARS; i++) {
        if (env_store[i].used && ev_streq(env_store[i].key, key)) {
            ev_strcpy(env_store[i].val, val, ENV_VAL_LEN);
            return true;
        }
    }
    /* Insert new */
    for (u32 i = 0; i < ENV_MAX_VARS; i++) {
        if (!env_store[i].used) {
            env_store[i].used = true;
            ev_strcpy(env_store[i].key, key, ENV_KEY_LEN);
            ev_strcpy(env_store[i].val, val, ENV_VAL_LEN);
            return true;
        }
    }
    return false; /* Store full */
}

extern "C" bool sigma_env_unset(const char* key) {
    env_bootstrap();
    for (u32 i = 0; i < ENV_MAX_VARS; i++) {
        if (env_store[i].used && ev_streq(env_store[i].key, key)) {
            env_store[i].used = false;
            env_store[i].key[0] = '\0';
            return true;
        }
    }
    return false;
}

/* ─────────────── Main ─────────────── */
extern "C" int sigma_env_main(int argc, char** argv) {
    env_bootstrap();

    bool opt_null    = false; /* -0: NUL-separate */
    bool opt_unset   = false; /* -u KEY: unset */
    bool opt_ignore  = false; /* -i: start with empty env */
    const char* unset_key = nullptr;

    int i = 1;
    for (; i < argc; i++) {
        if (argv[i][0] != '-') break;
        if (ev_streq(argv[i], "--")) { i++; break; }
        for (int j = 1; argv[i][j]; j++) {
            switch (argv[i][j]) {
                case '0': opt_null   = true; break;
                case 'i': opt_ignore = true; break;
                case 'u':
                    opt_unset = true;
                    if (i + 1 < argc) unset_key = argv[++i];
                    break;
            }
        }
    }

    /* -i: mark all as unused */
    if (opt_ignore) {
        for (u32 k = 0; k < ENV_MAX_VARS; k++) env_store[k].used = false;
    }

    /* -u: unset a specific key */
    if (opt_unset && unset_key) sigma_env_unset(unset_key);

    /* Parse any KEY=VALUE assignments from remaining args */
    while (i < argc && argv[i][0] != '\0') {
        /* Check if this arg contains '=' */
        const char* eq = nullptr;
        for (u32 j = 0; argv[i][j]; j++) {
            if (argv[i][j] == '=') { eq = argv[i] + j; break; }
        }
        if (eq) {
            /* KEY=VALUE pair */
            char key[ENV_KEY_LEN];
            u32 klen = (u32)(eq - argv[i]);
            if (klen >= ENV_KEY_LEN) klen = ENV_KEY_LEN - 1;
            for (u32 k = 0; k < klen; k++) key[k] = argv[i][k];
            key[klen] = '\0';
            sigma_env_set(key, eq + 1);
            i++;
        } else {
            /* This would be the command to execute — not supported in bare-metal */
            sigma_vga_puts("env: command execution not supported in bare-metal mode\n");
            break;
        }
    }

    /* If no command given, print all env vars */
    if (i >= argc) {
        for (u32 k = 0; k < ENV_MAX_VARS; k++) {
            if (!env_store[k].used) continue;
            sigma_vga_puts(env_store[k].key);
            sigma_vga_putchar('=');
            sigma_vga_puts(env_store[k].val);
            if (opt_null) sigma_vga_putchar('\0');
            else          sigma_vga_putchar('\n');
        }
    }
    return 0;
}
