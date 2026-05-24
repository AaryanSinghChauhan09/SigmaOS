/*
 * Σ SigmaOS — sigma_secure_boot: Verified Bootloader Subsystem
 * Zero-Dependency: No libc.
 * Ensures SigmaOS only boots verified binaries via cryptographic signatures.
 */

typedef unsigned char  u8;
typedef unsigned int   u32;
typedef unsigned long long u64;

extern "C" void sigma_vga_puts(const char* s);
extern "C" void sigma_vga_printf(const char* fmt, ...);
extern "C" int sigma_dilithium_verify(const u8* sig, u32 sig_len, const u8* msg, u32 msg_len, const u8* pk);
extern "C" void sigma_sha256_hash(const u8* data, u32 len, u8* hash_out); // From sigma_sha256.cpp

/* Hardcoded Root of Trust Public Key (Dilithium) */
static const u8 ROOT_PK[1312] = {0}; // Stub

/* TPM PCR state */
static u8 pcr_0[32] = {0};

static void tpm_pcr_extend(u32 pcr_idx, const u8* hash) {
    if (pcr_idx == 0) {
        // PCR[0] = SHA256(PCR[0] || hash)
        u8 buffer[64];
        for (int i = 0; i < 32; i++) buffer[i] = pcr_0[i];
        for (int i = 0; i < 32; i++) buffer[32+i] = hash[i];
        sigma_sha256_hash(buffer, 64, pcr_0);
        sigma_vga_printf("[TPM] PCR[%d] extended with new measurement.\n", pcr_idx);
    }
}

static u32 read_rollback_counter() {
    // Stub: Read from anti-rollback eFuses or secure NVRAM
    return 2;
}

static int verify_signature(const u8* binary, u32 len, const u8* signature) {
    return sigma_dilithium_verify(signature, 2420, binary, len, ROOT_PK);
}

/* 
 * Bootloader Hook: Verifies a loaded kernel or driver image before execution.
 */
extern "C" int sigma_secure_boot_verify_image(
    const char* name, 
    const u8* binary, 
    u32 len, 
    const u8* sig,
    u32 image_version) 
{
    sigma_vga_puts("[SECURE BOOT] Verifying image: ");
    sigma_vga_puts(name);
    sigma_vga_puts("\n");

    if (image_version < read_rollback_counter()) {
        sigma_vga_printf("[SECURE BOOT] FATAL: Rollback detected! Image v%d < Minimum v%d\n",
                         image_version, read_rollback_counter());
        return 0;
    }

    if (!verify_signature(binary, len, sig)) {
        sigma_vga_puts("[SECURE BOOT] FATAL: Signature verification failed!\n");
        return 0; /* Halt execution */
    }

    // Measure the verified image into TPM PCR 0
    u8 hash[32];
    sigma_sha256_hash(binary, len, hash);
    tpm_pcr_extend(0, hash);

    sigma_vga_puts("[SECURE BOOT] Image verified successfully.\n");
    return 1;
}
