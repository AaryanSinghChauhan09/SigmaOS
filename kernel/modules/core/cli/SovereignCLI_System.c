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
