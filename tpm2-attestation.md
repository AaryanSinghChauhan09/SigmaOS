# TPM2 Measured Boot + Remote Attestation

## Overview

SigmaOS integrates TPM2 for **measured boot** and **remote attestation**. During boot, each stage hashes its successor into Platform Configuration Registers (PCRs). The FDE (Full Disk Encryption) key is sealed to PCR values, so it only unseals if the boot chain is unmodified. `sigma-trustd` performs remote attestation by quoting TPM2 PCRs to a remote verifier.

---

## PCR Measurement Chain

| PCR | Content Measured |
|---|---|
| PCR 0 | UEFI firmware (CRTM) |
| PCR 1 | UEFI configuration |
| PCR 4 | Boot loader code (sigma-boot.efi) |
| PCR 5 | Boot loader config |
| PCR 7 | Secure Boot state |
| PCR 8 | sigma-kernel.elf hash |
| PCR 9 | initramfs hash |
| PCR 14 | sigma-init hash |

SigmaOS seals the FDE key to **PCRs 0, 4, 7, 8** using `tpm2_unseal` — any modification to firmware, bootloader, secure boot state, or kernel breaks the seal.

---

## File Layout

```
security/
├── README.md
├── sigma_trustd.rs     # remote attestation daemon

└── sigma_tpm2.rs       # TPM2 primitives wrapper

```

---

## sigma_tpm2.rs: TPM2 Primitives

```rust
//! SigmaOS TPM2 wrapper: PCR sealing, unsealing, and quoting.

use std::process::Command;

/// Read a PCR value.
pub fn pcr_read(pcr_index: u32) -> Result<Vec<u8>, Tpm2Error> {
    let output = Command::new("tpm2_pcrread")
        .arg(format!("sha256:{}", pcr_index))
        .output()?;
    if !output.status.success() {
        return Err(Tpm2Error::PcrReadFailed);
    }
    // Parse hex output from tpm2_pcrread
    parse_pcr_hex(&output.stdout)
}

/// Seal a secret to a set of PCRs.
pub fn seal_to_pcrs(
    secret: &[u8],
    pcr_list: &[u32],
    output_path: &str,
) -> Result<(), Tpm2Error> {
    let pcr_arg = pcr_list.iter()
        .map(|p| format!("sha256:{}", p))
        .collect::<Vec<_>>()
        .join("+");
    let status = Command::new("tpm2_create")
        .args([
            "--parent-context", "0x81000001",
            "--pcrs", &pcr_arg,
            "--sealing-input", "-",
            "--private", output_path,
        ])
        .stdin(std::process::Stdio::piped())
        .status()?;
    if status.success() { Ok(()) } else { Err(Tpm2Error::SealFailed) }
}

/// Unseal a secret (only succeeds if PCRs match).
pub fn unseal(private_path: &str) -> Result<Vec<u8>, Tpm2Error> {
    let output = Command::new("tpm2_unseal")
        .args(["--object-context", private_path])
        .output()?;
    if output.status.success() {
        Ok(output.stdout)
    } else {
        Err(Tpm2Error::UnsealFailed)
    }
}

/// Generate a TPM2 quote for PCRs (for remote attestation).
pub fn generate_quote(
    pcr_list: &[u32],
    nonce: &[u8],
    quote_path: &str,
    sig_path: &str,
) -> Result<(), Tpm2Error> {
    let pcr_arg = pcr_list.iter()
        .map(|p| p.to_string())
        .collect::<Vec<_>>()
        .join(",");
    let status = Command::new("tpm2_quote")
        .args([
            "--key-context", "0x81000002",
            "--pcr-list", &format!("sha256:{}", pcr_arg),
            "--qualification", &hex::encode(nonce),
            "--message", quote_path,
            "--signature", sig_path,
        ])
        .status()?;
    if status.success() { Ok(()) } else { Err(Tpm2Error::QuoteFailed) }
}

fn parse_pcr_hex(data: &[u8]) -> Result<Vec<u8>, Tpm2Error> {
    let s = std::str::from_utf8(data).map_err(|_| Tpm2Error::ParseFailed)?;
    hex::decode(s.trim()).map_err(|_| Tpm2Error::ParseFailed)
}

#[derive(Debug)]
pub enum Tpm2Error {
    PcrReadFailed,
    SealFailed,
    UnsealFailed,
    QuoteFailed,
    ParseFailed,
    Io(std::io::Error),
}
impl From<std::io::Error> for Tpm2Error {
    fn from(e: std::io::Error) -> Self { Self::Io(e) }
}
```

---

## sigma_trustd.rs: Remote Attestation Daemon

```rust
//! sigma-trustd: remote attestation daemon.
//! Quotes TPM2 PCRs and verifies the quote against a remote verifier.

mod tpm2;
use tpm2::{generate_quote, pcr_read};

pub fn attest(verifier_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Request nonce from verifier
    let nonce = sigma_curl::get_bytes(&format!("{}/nonce", verifier_url))?;

    // 2. Generate TPM2 quote
    generate_quote(&[0, 4, 7, 8], &nonce, "/tmp/quote.msg", "/tmp/quote.sig")?;

    // 3. Send quote + PCR values to verifier
    let quote   = std::fs::read("/tmp/quote.msg")?;
    let sig     = std::fs::read("/tmp/quote.sig")?;
    let pcr8    = pcr_read(8)?;
    let payload = serde_json::json!({
        "quote": base64::encode(&quote),
        "sig":   base64::encode(&sig),
        "pcr8":  hex::encode(&pcr8),
    });
    let result = sigma_curl::post_json(
        &format!("{}/verify", verifier_url),
        &payload,
    )?;
    println!("Attestation result: {}", result);
    Ok(())
}
```

---

## CLI Usage

```bash

# Perform remote attestation

sigma-trustd attest --verifier https://attest.sigmaos.dev

# Read PCR 8 (kernel hash)

sigma-tpm2 pcr-read 8

# Seal FDE key to PCRs 0,4,7,8

sigma-tpm2 seal --pcrs 0,4,7,8 --input /tmp/fde.key --output /boot/fde-sealed.bin
```

---

## Exit Criteria

- `sigma-trustd attest` produces a verified quote; verifier returns `{"status": "verified"}`.

- FDE key unseals correctly on unmodified boot; fails after kernel replacement.

- `sigma-tpm2 pcr-read 8` matches `sha256sum sigma-kernel.elf`.
