# Signature Verification Guidelines

## Overview
Based on security learnings from .jules/sentinel.md, this document provides guidelines for implementing cryptographic signature verification to prevent supply-chain attacks.

## Vulnerability: Sovereign Keyring Ed25519 Verification Against Supply-Chain Attacks

**Learning:** Hash verification alone proves integrity (the file hasn't been corrupted), but NOT authenticity (the file came from a trusted source). Ed25519 signature verification against a Sovereign Keyring provides both, creating a two-layer defense.

## Prevention Guidelines

### 1. Dual-Layer Verification

**DO:**
```go
import (
    "crypto/sha256"
    "crypto/ed25519"
    "encoding/hex"
)

type PackageVerification struct {
    HashValid      bool
    SignatureValid bool
    TrustedSource  bool
}

func VerifyPackage(data []byte, expectedHash string, signature []byte, publicKey ed25519.PublicKey) (*PackageVerification, error) {
    result := &PackageVerification{}
    
    // Layer 1: Hash verification (integrity)
    hash := sha256.Sum256(data)
    actualHash := hex.EncodeToString(hash[:])
    if actualHash == expectedHash {
        result.HashValid = true
    }
    
    // Layer 2: Signature verification (authenticity)
    if ed25519.Verify(publicKey, data, signature) {
        result.SignatureValid = true
        result.TrustedSource = true
    }
    
    return result, nil
}
```

**DON'T:**
```go
// Unsafe: Hash verification only
func VerifyPackage(data []byte, expectedHash string) bool {
    hash := sha256.Sum256(data)
    actualHash := hex.EncodeToString(hash[:])
    return actualHash == expectedHash
    // Vulnerable to supply-chain attacks
}
```

### 2. Sovereign Keyring Management

**DO:**
```go
const (
    KeyringPath = "/etc/sigma/keys"
)

type SovereignKeyring struct {
    keys map[string]ed25519.PublicKey
}

func LoadSovereignKeyring() (*SovereignKeyring, error) {
    keyring := &SovereignKeyring{
        keys: make(map[string]ed25519.PublicKey),
    }
    
    // Load trusted keys from secure storage
    files, err := os.ReadDir(KeyringPath)
    if err != nil {
        return nil, err
    }
    
    for _, file := range files {
        keyData, err := os.ReadFile(filepath.Join(KeyringPath, file.Name()))
        if err != nil {
            continue
        }
        
        var publicKey ed25519.PublicKey
        copy(publicKey, keyData)
        keyring.keys[file.Name()] = publicKey
    }
    
    return keyring, nil
}

func (k *SovereignKeyring) GetKey(maintainer string) (ed25519.PublicKey, error) {
    key, exists := k.keys[maintainer]
    if !exists {
        return nil, fmt.Errorf("unknown maintainer: %s", maintainer)
    }
    return key, nil
}
```

### 3. Package Transaction Verification

**DO:**
```go
type PackageTransaction struct {
    Name      string
    Version   string
    Data      []byte
    Hash      string
    Signature []byte
    Maintainer string
}

func (tx *PackageTransaction) Verify() error {
    // Load sovereign keyring
    keyring, err := LoadSovereignKeyring()
    if err != nil {
        return fmt.Errorf("failed to load keyring: %w", err)
    }
    
    // Get maintainer's public key
    publicKey, err := keyring.GetKey(tx.Maintainer)
    if err != nil {
        return fmt.Errorf("unknown maintainer: %w", err)
    }
    
    // Perform dual-layer verification
    result, err := VerifyPackage(tx.Data, tx.Hash, tx.Signature, publicKey)
    if err != nil {
        return err
    }
    
    // Reject if either layer fails
    if !result.HashValid {
        return fmt.Errorf("hash verification failed: possible corruption")
    }
    
    if !result.SignatureValid {
        return fmt.Errorf("signature verification failed: possible tampering")
    }
    
    if !result.TrustedSource {
        return fmt.Errorf("untrusted source: signature not from known maintainer")
    }
    
    return nil
}
```

**DON'T:**
```go
// Unsafe: Accept packages with unknown signatures
func (tx *PackageTransaction) Verify() error {
    if strings.HasPrefix(tx.SignatureStr, "sig:ed25519:unknown") {
        // Accept in production mode - VULNERABLE
        return nil
    }
    // ... rest of verification
}
```

### 4. Production Mode Enforcement

**DO:**
```go
const ProductionMode = true

func VerifyPackageInProduction(tx *PackageTransaction) error {
    if ProductionMode {
        // Strict enforcement in production
        if err := tx.Verify(); err != nil {
            return err
        }
        
        // Additional production checks
        if tx.Maintainer == "" {
            return fmt.Errorf("production mode requires maintainer signature")
        }
    } else {
        // Development mode: warn but allow
        log.Printf("WARNING: Skipping signature verification in development mode")
    }
    return nil
}
```

## Implementation Checklist

- [ ] Implement Ed25519 signature verification for all package transactions
- [ ] Set up Sovereign Keyring at `/etc/sigma/keys`
- [ ] Require both hash AND signature verification
- [ ] Reject packages with `sig:ed25519:unknown` in production mode
- [ ] Add key rotation mechanism for keyring updates
- [ ] Document key management procedures
- [ ] Add unit tests for signature bypass attempts

## Key Management

### Key Generation
```bash
# Generate new Ed25519 key pair
openssl genpkey -algorithm ed25519 -out private.pem
openssl pkey -in private.pem -pubout -out public.pem

# Add public key to sovereign keyring
sudo cp public.pem /etc/sigma/keys/maintainer-name.pem
```

### Key Distribution
- Public keys are stored in `/etc/sigma/keys/`
- Private keys are kept secure by maintainers
- Key rotation requires OS update signed by previous key

## References

- Original learning from: .jules/sentinel.md (2026-07-14)
- Ed25519: EdDSA for Curve25519
- CWE-494: Download of Code Without Integrity Check
- Supply Chain Security Best Practices
