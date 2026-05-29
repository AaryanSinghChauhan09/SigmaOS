/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN GETOPT
 * =========================================================================
 * Minimal, zero-dependency command line argument parser.
 * =========================================================================
 */

#include "../../include/sigma_kernel_types.h"

int sigma_optind = 1;
char* sigma_optarg = SIGMA_NULL;

/* We use the inline string functions from types or libc */
extern int sigma_strcmp(const char* a, const char* b);
extern sigma_size_t sigma_strlen(const char* s);

int sigma_getopt(int argc, char* const argv[], const char* optstring) {
    if (sigma_optind >= argc || argv[sigma_optind] == SIGMA_NULL) {
        return -1;
    }

    if (argv[sigma_optind][0] != '-' || argv[sigma_optind][1] == '\0') {
        return -1;
    }

    if (sigma_strcmp(argv[sigma_optind], "--") == 0) {
        sigma_optind++;
        return -1;
    }

    char opt = argv[sigma_optind][1];
    
    /* Find option in optstring */
    const char* ptr = optstring;
    while (*ptr != '\0' && *ptr != opt) {
        ptr++;
    }

    if (*ptr == '\0') {
        /* Unknown option */
        sigma_optind++;
        return '?';
    }

    if (*(ptr + 1) == ':') {
        /* Requires argument */
        if (argv[sigma_optind][2] != '\0') {
            sigma_optarg = &argv[sigma_optind][2];
        } else if (sigma_optind + 1 < argc) {
            sigma_optarg = argv[sigma_optind + 1];
            sigma_optind++;
        } else {
            /* Missing argument */
            sigma_optind++;
            return ':';
        }
    } else {
        sigma_optarg = SIGMA_NULL;
    }

    sigma_optind++;
    return opt;
}
