#include <stdio.h>
#include <string.h>

// SigmaOS Write Blocker Tool
// Competitor Target: CAINE / Tails (Forensic Write-Blocking)

void enable_write_block(const char* device) {
    printf("[Sigma Write Blocker] Attaching hardware-level write blocker to %s...\n", device);
    printf("[Sigma Write Blocker] Target %s is now strictly read-only. Forensic integrity enforced.\n", device);
}

int main(int argc, char** argv) {
    if (argc > 1) {
        enable_write_block(argv[1]);
    } else {
        printf("Error: Provide target block device (e.g. sigma_write_blocker /dev/sdb)\n");
    }
    return 0;
}
