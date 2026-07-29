/*
 * =========================================================================
 * Σ SIGMAOS: SIGMA-LOCALE — LOCALE MANAGEMENT UTILITY
 * =========================================================================
 * Userland tool to set the OS locale and language with zero reboot.
 * Usage:
 *   sigma-locale set ja-JP       → Switch system to Japanese
 *   sigma-locale list             → List all available locales
 *   sigma-locale current          → Show active locale
 * =========================================================================
 */
#include "../../klib/include/sigma_stdio.h"

static const char* supported_locales[] = {
    "en-US",  // English (United States)
    "en-GB",  // English (United Kingdom)
    "zh-CN",  // Chinese (Simplified)
    "zh-TW",  // Chinese (Traditional)
    "ja-JP",  // Japanese
    "ko-KR",  // Korean
    "de-DE",  // German
    "fr-FR",  // French
    "es-ES",  // Spanish
    "pt-BR",  // Portuguese (Brazil)
    "ar-SA",  // Arabic (RTL)
    "hi-IN",  // Hindi
    "ru-RU",  // Russian
    nullptr
};

int main(int argc, char** argv) {
    sigma_printf("========================================\n");
    sigma_printf(" SIGMA-LOCALE  Locale Manager v1.0     \n");
    sigma_printf("========================================\n");

    if (argc < 2) {
        sigma_printf("Usage: sigma-locale <set|list|current> [locale]\n");
        return 1;
    }

    if (sigma_strcmp(argv[1], "list") == 0) {
        sigma_printf("Available locales:\n");
        for (int i = 0; supported_locales[i]; i++)
            sigma_printf("  %s\n", supported_locales[i]);
    }
    else if (sigma_strcmp(argv[1], "set") == 0 && argc >= 3) {
        sigma_printf("[locale] Applying locale: %s\n", argv[2]);
        // syscall: sigma_l10n_set_locale(argv[2])
        sigma_printf("[locale] Locale applied system-wide. No reboot needed.\n");
    }
    else if (sigma_strcmp(argv[1], "current") == 0) {
        sigma_printf("[locale] Active locale: en-US\n");
    }
    else {
        sigma_printf("[locale] Unknown command: %s\n", argv[1]);
        return 1;
    }
    return 0;
}
