#ifndef SIGMA_VFS_CRYPTO_H
#define SIGMA_VFS_CRYPTO_H

#include "sigma_types.h"

// ---------------------------------------------------------
// SigmaOS Encrypted Volume Management
// Provides foundational AES-XTS stubs for secure block storage.
// ---------------------------------------------------------

namespace sigma {
namespace fs {

constexpr uint32_t SIGMA_SECTOR_SIZE = 4096;

struct CipherKey {
    uint8_t key[32]; // AES-256
    uint8_t salt[16];
};

class SovereignEncryptedVolume {
private:
    uint32_t volume_id;
    bool is_mounted;
    CipherKey active_key;

public:
    SovereignEncryptedVolume(uint32_t vol_id);
    ~SovereignEncryptedVolume();

    // Unlock and mount the volume
    bool mount(const uint8_t* passphrase, size_t pass_len);
    
    // Lock the volume
    void unmount();

    // Read an encrypted block, decrypt it, and return plaintext
    bool read_block(uint64_t lba, uint8_t* buffer);
    
    // Encrypt plaintext block and write to storage
    bool write_block(uint64_t lba, const uint8_t* buffer);
    
    bool get_mounted_status() const { return is_mounted; }
};

} // namespace fs
} // namespace sigma

#endif // SIGMA_VFS_CRYPTO_H
