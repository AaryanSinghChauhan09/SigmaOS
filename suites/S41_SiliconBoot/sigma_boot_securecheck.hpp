// SigmaOS — sigma-boot-securecheck: Quantum-Safe Boot Verification
// Module: sigma-boot-securecheck
// USP: Defeats UEFI Secure Boot. Cryptographically verifies kernel integrity
//      using post-quantum Montgomery reduction before yielding execution to the OS.

#ifndef SIGMA_BOOT_SECURECHECK_HPP
#define SIGMA_BOOT_SECURECHECK_HPP

#include "../S08_Security/sigma_pqc_verify.h"

namespace sigma {
namespace boot {

class QuantumSecureCheck {
public:
    static bool verify_kernel_image(const unsigned char* expected_pubkey, 
                                    const unsigned char* kernel_bin, unsigned int bin_size,
                                    const unsigned char* embedded_signature) {
        
        // Ensure no null pointers
        if (!expected_pubkey || !kernel_bin || !embedded_signature) return false;

        // Perform Quantum-safe cryptographic verification
        int status = pqc_verify(expected_pubkey, kernel_bin, bin_size, embedded_signature);
        
        if (status != 0) {
            // VERIFICATION FAILED: Halt silicon immediately to prevent rootkit execution
#if defined(__x86_64__) || defined(__i386__)
            __asm__ __volatile__("hlt\n\t" ::: "memory");
#endif
            return false; 
        }

        return true;
    }
};

} // namespace boot
} // namespace sigma

#endif /* SIGMA_BOOT_SECURECHECK_HPP */
