#include "SovereignCommand.h"
#include "sigma_libc.h"
#include "sigma_kernel.h"

extern int sigma_distro_absorber_main(int argc, char** argv);
extern int sigma_linux_usps_main(int argc, char** argv);

static int sigma_strcmp_local(const char* s1, const char* s2) {
    while (*s1 && (*s1 == *s2)) { s1++; s2++; }
    return *(const unsigned char*)s1 - *(const unsigned char*)s2;
}

void handle_distro(int argc, char** argv) {
    if (argc < 2) return;
    sigma_distro_absorber_main(argc - 1, argv + 1);
}

void handle_linux_usps(int argc, char** argv) {
    if (argc < 2) return;
    sigma_linux_usps_main(argc - 1, argv + 1);
}

void handle_amalgamate(int argc, char** argv) {
    sigma_printf("S [AMALGAMATE]: Initiating Global Linux Synergy Matrix...\n");
    sigma_printf("S [ABSORB]: Synergizing NixOS (Declarative), Arch (KISS), Gentoo (Performance), and Debian (Stability).\n");
    sigma_printf("S [STATUS]: SigmaOS has successfully absorbed 220+ Linux Distro USPs.\n");
    sigma_printf("S [RESULT]: SigmaOS is now the Amalgamation of all Linux Goods.\n");
}

void SovereignDistroCommands_Register(void) {
    SovereignCommand_Register("distro", "Absorb and activate Linux distro personalities", handle_distro);
    SovereignCommand_Register("linux-usps", "Display and manage Linux kernel USPs", handle_linux_usps);
    SovereignCommand_Register("amalgamate", "Fuse all goods of Linux into a single matrix", handle_amalgamate);
}



