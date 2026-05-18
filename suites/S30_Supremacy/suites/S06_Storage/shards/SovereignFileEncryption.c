// =============================================================================
// SigmaOS — S06_Storage — SovereignFileEncryption.c
// Full-Disk & Per-File Encryption Shard
// =============================================================================
// Competitor USPs Absorbed:
//   • LUKS2 (Linux)   — PBKDF2/Argon2 key derivation, header backup
//   • BitLocker (Win) — TPM-sealed key, pre-boot auth, volume master key
//   • FileVault 2 (macOS) — iCloud recovery key, HFS+ wrapper volume
//   • VeraCrypt (OSS) — plausible deniability hidden volumes
//   • APFS Encryption — per-extent encryption, no performance cliff
// Architecture:
//   • AES-256-XTS (industry standard for disk encryption)
//   • Key sealed in S08_Security biometric enclave (TPM equivalent)
//   • Argon2id KDF: memory-hard brute-force resistance
//   • Optional hidden volume at end of disk (VeraCrypt model)
//   • Per-file granular encryption via VFS open() hook
// =============================================================================

#include "core/sigma_types.h"

#include "libc/sigma_libc.h"

#define FDE_SECTOR_SIZE       512
#define FDE_KEY_LEN_BYTES      32   // AES-256
#define FDE_TWEAK_LEN_BYTES    16   // XTS tweak (sector number)
#define FDE_SALT_LEN_BYTES     32
#define FDE_ARGON2_MEM_KB   65536   // 64MB Argon2id memory parameter

// ── Encryption Header (stored in first sector) ─────────────────────────────
typedef struct {
    uint64_t magic;                       // 0x5349474D43525950 "SIGMCRYP"
    uint8_t  salt[FDE_SALT_LEN_BYTES];    // Argon2id salt
    uint8_t  master_key_encrypted[48];    // Volume master key, AES-256-GCM wrapped
    uint8_t  iv[12];                      // GCM IV for key wrapping
    uint8_t  auth_tag[16];                // GCM authentication tag
    uint32_t argon2_time_cost;
    uint32_t argon2_mem_cost_kb;
    uint8_t  has_hidden_volume;           // VeraCrypt plausible deniability
    uint64_t hidden_volume_offset;
    uint8_t  _reserved[96];
} SigmaFDEHeader;

// ── Session Key Material ──────────────────────────────────────────────────────
typedef struct {
    uint8_t  derived_key[FDE_KEY_LEN_BYTES];
    uint64_t volume_offset_sectors;
    bool     is_hidden;
    bool     is_unlocked;
} SigmaVolumeSession;

// ── Public API ────────────────────────────────────────────────────────────────

// Format a volume with FDE (destructive — creates header + key material)
bool fde_format_volume(const char* device_id, const char* passphrase,
                       bool create_hidden_volume);

// Unlock a volume: Argon2id KDF → decrypt master key → activate XTS cipher
bool fde_unlock_volume(const char* device_id, const char* passphrase,
                       SigmaVolumeSession* out_session);

// Encrypt/decrypt a single 512-byte sector using AES-256-XTS
void fde_encrypt_sector(SigmaVolumeSession* sess, uint64_t sector_lba,
                        const uint8_t* plaintext, uint8_t* ciphertext);
void fde_decrypt_sector(SigmaVolumeSession* sess, uint64_t sector_lba,
                        const uint8_t* ciphertext, uint8_t* plaintext);

// Per-file encryption: hook into VFS open() to lazily encrypt on write
bool fde_encrypt_file(const char* vfs_path, const uint8_t* file_key);
bool fde_decrypt_file(const char* vfs_path, const uint8_t* file_key);

// Lock a volume and zeroize key material from RAM
void fde_lock_volume(SigmaVolumeSession* sess);



