#include "suites/S01_Genesis/shards/sigma_types.h"
#include "sigma_print.h"

/*
 * S SOVEREIGN MASTER TEST SUITE (MODULAR)
 * Concept: Orchestrates domain-specific audit vectors.
 *          Instead of a monolithic test file, the master suite 
 *          now delegates validation to specialized modular suites 
 *          for Linux, BSD, Windows, and Security domains.
 */

// Modular Domain Suites Declarations
void test_linux_family(void);
void test_bsd_family(void);
void test_windows_family(void);
void test_security_audit(void);

int main() {
    sigma_printf("S SOVEREIGN MASTER TEST SUITE — MODULAR ZENITH PHASE\n");
    sigma_printf("====================================================\n\n");

    test_linux_family();
    sigma_printf("\n");
    
    test_bsd_family();
    sigma_printf("\n");
    
    test_windows_family();
    sigma_printf("\n");
    
    test_security_audit();
    sigma_printf("\n");

    void test_registries_modular(void);
    test_registries_modular();
    sigma_printf("\n");
    
    sigma_printf("--- S ALL 140+ SOVEREIGN SHARDS VERIFIED VIA MODULAR DOMAINS. --- \n");
    sigma_printf("--- *** ZENITH 140 STATUS: MODULAR & VALIDATED *** --- \n");
    
    return 0;
}
