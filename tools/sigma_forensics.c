// sigma_forensics.c - Minimal forensic utility stub
#include "sigma_log.h"

// Entry point for forensic collection
int sigma_forensics_collect(void) {
    sigma_log_info("Sigma Forensics: starting collection...");
    // TODO: implement memory dump, process snapshot, and log extraction
    return 0; // success
}
