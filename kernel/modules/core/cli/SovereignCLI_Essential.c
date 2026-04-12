#include "../../../../include/sigma_kernel.h"
#include "../../../../include/SovereignCLI.h"

/* ---- sigma-ls ---------------------------------------------------------- */
sigma_err_t sigma_cmd_ls(int argc, char *argv[]) {
    const char *path = (argc > 1) ? argv[1] : ".";
    sigma_printf("Σ [LS]: Listing '%s':\n", path);
    static const char *demo[] = {
        "bin/", "boot/", "dev/", "etc/", "home/", "lib/", "proc/",
        "root/", "run/", "sbin/", "sys/", "tmp/", "usr/", "var/", SIGMA_NULL
    };
    for (int i = 0; demo[i]; i++)
        sigma_printf("  %s\n", demo[i]);
    return SIGMA_OK;
}

/* ---- sigma-cat --------------------------------------------------------- */
sigma_err_t sigma_cmd_cat(int argc, char *argv[]) {
    if (argc < 2) { sigma_printf("Usage: sigma-cat <file>\n"); return SIGMA_EINVAL; }
    sigma_printf("Σ [CAT]: Reading '%s'...\n", argv[1]);
    sigma_printf("  [SigmaOS configuration placeholder content]\n");
    return SIGMA_OK;
}

/* ---- sigma-cp ---------------------------------------------------------- */
sigma_err_t sigma_cmd_cp(int argc, char *argv[]) {
    if (argc < 3) { sigma_printf("Usage: sigma-cp <src> <dst>\n"); return SIGMA_EINVAL; }
    sigma_printf("Σ [CP]: %s -> %s\n", argv[1], argv[2]);
    return SIGMA_OK;
}

/* ---- sigma-mv ---------------------------------------------------------- */
sigma_err_t sigma_cmd_mv(int argc, char *argv[]) {
    if (argc < 3) { sigma_printf("Usage: sigma-mv <src> <dst>\n"); return SIGMA_EINVAL; }
    sigma_printf("Σ [MV]: %s -> %s\n", argv[1], argv[2]);
    return SIGMA_OK;
}

/* ---- sigma-rm ---------------------------------------------------------- */
sigma_err_t sigma_cmd_rm(int argc, char *argv[]) {
    if (argc < 2) { sigma_printf("Usage: sigma-rm <file>\n"); return SIGMA_EINVAL; }
    sigma_printf("Σ [RM]: Removing '%s'\n", argv[argc - 1]);
    return SIGMA_OK;
}

/* ---- sigma-mkdir ------------------------------------------------------- */
sigma_err_t sigma_cmd_mkdir(int argc, char *argv[]) {
    if (argc < 2) { sigma_printf("Usage: sigma-mkdir <dir>\n"); return SIGMA_EINVAL; }
    sigma_printf("Σ [MKDIR]: Creating directory '%s'\n", argv[1]);
    return SIGMA_OK;
}

/* ---- sigma-stat -------------------------------------------------------- */
sigma_err_t sigma_cmd_stat(int argc, char *argv[]) {
    if (argc < 2) { sigma_printf("Usage: sigma-stat <file>\n"); return SIGMA_EINVAL; }
    sigma_printf("Σ [STAT]: File: %s\n  Size: 4096\n", argv[1]);
    return SIGMA_OK;
}

/* ---- sigma-find -------------------------------------------------------- */
sigma_err_t sigma_cmd_find(int argc, char *argv[]) {
    const char *root = (argc > 1) ? argv[1] : ".";
    sigma_printf("Σ [FIND]: Searching under '%s'...\n", root);
    return SIGMA_OK;
}

#include "../../../../include/sigma_kernel.h"
#include "../../../../include/SovereignCLI.h"
#include "../../../../include/SovereignEnvManager.h"
#include "../../../../include/SovereignDmesg.h"

/* ---- sigma-echo -------------------------------------------------------- */
sigma_err_t sigma_cmd_echo(int argc, char *argv[]) {
    for (int i = 1; i < argc; i++) {
        if (i > 1) sigma_printf(" ");
        sigma_printf("%s", argv[i]);
    }
    sigma_printf("\n");
    return SIGMA_OK;
}

/* ---- sigma-env --------------------------------------------------------- */
sigma_err_t sigma_cmd_env(int argc, char *argv[]) {
    if (argc == 1) { sigma_env_dump(&g_sigma_env); return SIGMA_OK; }
    const char *eq = sigma_strstr(argv[1], "=");
    if (eq) {
        char key[SIGMA_ENV_KEY_MAX];
        sigma_u32 klen = (sigma_u32)(eq - argv[1]);
        sigma_memcpy(key, argv[1], klen); key[klen] = '\0';
        sigma_env_set(&g_sigma_env, key, eq + 1);
        sigma_printf("Σ [ENV]: Set %s=%s\n", key, eq + 1);
    } else {
        const char *val = sigma_env_get(&g_sigma_env, argv[1]);
        sigma_printf("%s=%s\n", argv[1], val ? val : "(unset)");
    }
    return SIGMA_OK;
}

/* ---- sigma-ps ---------------------------------------------------------- */
sigma_err_t sigma_cmd_ps(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_printf("Σ [PS]: Process List Active.\n");
    return SIGMA_OK;
}

/* ---- sigma-uname ------------------------------------------------------- */
sigma_err_t sigma_cmd_uname(int argc, char *argv[]) {
    sigma_bool all = (argc > 1 && sigma_streq(argv[1], "-a"));
    sigma_printf("SigmaOS");
    if (all) sigma_printf(" sigma-host 1.0.0-sovereign x86_64 SigmaOS/GNU");
    sigma_printf("\n");
    return SIGMA_OK;
}

/* ---- sigma-dmesg ------------------------------------------------------- */
sigma_err_t sigma_cmd_dmesg(int argc, char *argv[]) {
    sigma_bool clear = (argc > 1 && sigma_streq(argv[1], "-c"));
    if (clear) { sigma_dmesg_clear(&g_sigma_dmesg); return SIGMA_OK; }
    sigma_dmesg_dump(&g_sigma_dmesg);
    return SIGMA_OK;
}

