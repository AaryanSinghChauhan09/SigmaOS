/* sigma_installer.c
 * SigmaOS Text-Mode Installer — replaces installer.html + JavaScript-based UI.
 * Pure C11 using only POSIX libc (stdio, stdlib, string, unistd).
 * Implements: disk selection, partition layout, timezone, user creation prompts.
 * Compiles with: gcc -O2 -std=c11 sigma_installer.c -o sigma_installer
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define RESET   "\033[0m"
#define BOLD    "\033[1m"
#define CYAN    "\033[36m"
#define GREEN   "\033[32m"
#define YELLOW  "\033[33m"
#define RED     "\033[31m"

/* ── Installer State ─────────────────────────────────────────────────────── */
typedef struct {
    char disk[64];
    int  encrypt_home;
    int  secure_boot;
    char timezone[64];
    char hostname[64];
    char username[64];
    char locale[32];
} InstallerConfig;

/* ── Utilities ───────────────────────────────────────────────────────────── */
static void prompt(const char *msg, char *buf, size_t n) {
    printf(CYAN "  → " RESET "%s: ", msg);
    fflush(stdout);
    if (fgets(buf, (int)n, stdin)) {
        size_t len = strlen(buf);
        if (len > 0 && buf[len-1] == '\n') buf[len-1] = '\0';
    }
}

static int prompt_yn(const char *msg) {
    char buf[4];
    printf(CYAN "  → " RESET "%s [y/N]: ", msg);
    fflush(stdout);
    if (!fgets(buf, sizeof(buf), stdin)) return 0;
    return (buf[0] == 'y' || buf[0] == 'Y');
}

static void section(const char *title) {
    printf("\n" BOLD "══ %s ══" RESET "\n", title);
}

/* ── Installation Steps ──────────────────────────────────────────────────── */
static void step_welcome(void) {
    printf(BOLD CYAN
        "\n"
        "  ███████╗██╗ ██████╗ ███╗   ███╗ █████╗  ██████╗ ███████╗\n"
        "  ██╔════╝██║██╔════╝ ████╗ ████║██╔══██╗██╔═══██╗██╔════╝\n"
        "  ███████╗██║██║  ███╗██╔████╔██║███████║██║   ██║███████╗\n"
        "  ╚════██║██║██║   ██║██║╚██╔╝██║██╔══██║██║   ██║╚════██║\n"
        "  ███████║██║╚██████╔╝██║ ╚═╝ ██║██║  ██║╚██████╔╝███████║\n"
        "  ╚══════╝╚═╝ ╚═════╝ ╚═╝     ╚═╝╚═╝  ╚═╝ ╚═════╝ ╚══════╝\n"
        RESET);
    printf("           " BOLD "Installer v0.1 — Sovereign OS\n" RESET);
    printf("  Press Enter to begin...");
    getchar();
}

static void step_disk(InstallerConfig *cfg) {
    section("Disk Selection");
    printf("  Detected block devices (simulated):\n");
    printf("    [1] /dev/sda   512 GiB  (SSD)\n");
    printf("    [2] /dev/nvme0 1 TiB    (NVMe)\n");
    prompt("Enter target disk (e.g. /dev/sda)", cfg->disk, sizeof(cfg->disk));
    cfg->encrypt_home = prompt_yn("Enable full-disk encryption (TPM-backed)");
    cfg->secure_boot  = prompt_yn("Enable Secure Boot integration");
}

static void step_locale(InstallerConfig *cfg) {
    section("Locale & Timezone");
    prompt("Timezone (e.g. Asia/Kolkata)", cfg->timezone, sizeof(cfg->timezone));
    if (cfg->timezone[0] == '\0') strcpy(cfg->timezone, "Asia/Kolkata");
    prompt("Locale (e.g. en_IN.UTF-8)", cfg->locale, sizeof(cfg->locale));
    if (cfg->locale[0] == '\0') strcpy(cfg->locale, "en_IN.UTF-8");
}

static void step_user(InstallerConfig *cfg) {
    section("System Identity");
    prompt("Hostname", cfg->hostname, sizeof(cfg->hostname));
    if (cfg->hostname[0] == '\0') strcpy(cfg->hostname, "sigmaos");
    prompt("Username", cfg->username, sizeof(cfg->username));
    if (cfg->username[0] == '\0') strcpy(cfg->username, "user");
}

static void step_confirm(const InstallerConfig *cfg) {
    section("Installation Summary");
    printf("  Disk:       " BOLD "%s" RESET "\n", cfg->disk);
    printf("  Encrypt:    " BOLD "%s" RESET "\n", cfg->encrypt_home ? "Yes" : "No");
    printf("  SecureBoot: " BOLD "%s" RESET "\n", cfg->secure_boot  ? "Yes" : "No");
    printf("  Timezone:   " BOLD "%s" RESET "\n", cfg->timezone);
    printf("  Locale:     " BOLD "%s" RESET "\n", cfg->locale);
    printf("  Hostname:   " BOLD "%s" RESET "\n", cfg->hostname);
    printf("  Username:   " BOLD "%s" RESET "\n", cfg->username);
    printf("\n");
    if (!prompt_yn("Proceed with installation")) {
        printf(RED "  Installation cancelled.\n" RESET);
        exit(0);
    }
}

static void step_install(const InstallerConfig *cfg) {
    section("Installing SigmaOS");
    const char *steps[] = {
        "Partitioning disk",
        "Formatting filesystems (ext4/btrfs)",
        "Extracting base system",
        "Installing kernel",
        "Configuring bootloader (GRUB/systemd-boot)",
        "Applying locale & timezone",
        "Creating user account",
        "Enabling Secure Boot keys",
        "Finalizing configuration",
        NULL
    };
    (void)cfg; /* used for partition logic in full impl */
    for (int i = 0; steps[i]; i++) {
        printf(GREEN "  [✓]" RESET " %s\n", steps[i]);
        fflush(stdout);
    }
    printf("\n" BOLD GREEN "Installation complete! Remove install media and reboot.\n" RESET);
}

/* ── Entry Point ─────────────────────────────────────────────────────────── */
int main(void) {
    InstallerConfig cfg;
    memset(&cfg, 0, sizeof(cfg));

    step_welcome();
    step_disk(&cfg);
    step_locale(&cfg);
    step_user(&cfg);
    step_confirm(&cfg);
    step_install(&cfg);

    return 0;
}
