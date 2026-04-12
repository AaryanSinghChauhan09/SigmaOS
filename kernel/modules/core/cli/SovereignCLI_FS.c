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
