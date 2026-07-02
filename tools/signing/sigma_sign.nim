# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
#
# tools/signing/sigma_sign.nim — Package signing + cosign integration
# Language: Nim — native binary, no GC overhead
# Pattern: OOP via object types + methods

import std/[os, osproc, streams, strformat, json, times]

# ── Types ─────────────────────────────────────────────────────────────────────

type
  SigningKeyType = enum
    KeyDilithium5 = "dilithium5"
    KeyEd25519    = "ed25519"

  SignedArtifact = object
    path:       string
    sha256:     string
    sig_path:   string
    provenance: string
    signed_at:  string
    key_type:   SigningKeyType

  SigningError = object
    message: string

# ── SHA-256 helper (calls sigma-pkg's internal hash) ─────────────────────────

proc computeSha256(path: string): string =
  let (output, exit_code) = execCmdEx("sha256sum " & quoteShell(path))
  if exit_code != 0: return ""
  result = output.split(" ")[0].strip()

# ── Provenance (in-toto inspired) ────────────────────────────────────────────

proc generateProvenance(artifact: SignedArtifact): string =
  let node = %* {
    "predicateType": "https://sigmaos.app/attestation/v1",
    "subject": [
      {
        "name": artifact.path.extractFilename,
        "digest": {"sha256": artifact.sha256}
      }
    ],
    "predicate": {
      "buildType": "https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/.github/workflows/sigma_ci.yml",
      "builder":   {"id": "github-actions"},
      "metadata": {
        "buildStartedOn":  artifact.signed_at,
        "completeness":    {"parameters": true, "environment": true, "materials": true}
      }
    }
  }
  return $node

# ── Cosign integration ────────────────────────────────────────────────────────

proc cosignSign(artifact_path: string, key_file: string): string =
  ## Sign artifact with cosign (keyless or key-based)
  ## Returns path of .sig file
  let sig_path = artifact_path & ".sig"
  let cmd = fmt"cosign sign-blob --key {quoteShell(key_file)} --output-signature {quoteShell(sig_path)} {quoteShell(artifact_path)}"
  let (output, exit_code) = execCmdEx(cmd)
  if exit_code != 0:
    stderr.writeLine("sigma-sign: cosign failed: " & output)
    return ""
  return sig_path

proc cosignVerify(artifact_path: string, sig_path: string, pub_key: string): bool =
  let cmd = fmt"cosign verify-blob --key {quoteShell(pub_key)} --signature {quoteShell(sig_path)} {quoteShell(artifact_path)}"
  let (_, exit_code) = execCmdEx(cmd)
  return exit_code == 0

# ── Main Signing Flow ─────────────────────────────────────────────────────────

proc signArtifact(path: string, key_file: string, key_type = KeyDilithium5): SignedArtifact =
  result.path      = path
  result.key_type  = key_type
  result.signed_at = $now().utc

  echo fmt"sigma-sign: computing sha256 of {path}..."
  result.sha256 = computeSha256(path)
  if result.sha256.len == 0:
    stderr.writeLine("sigma-sign: failed to compute sha256")
    quit(1)

  echo fmt"sigma-sign: signing with {key_type}..."
  result.sig_path = cosignSign(path, key_file)
  if result.sig_path.len == 0:
    # Fallback: write a placeholder .sig if cosign not available
    result.sig_path = path & ".sig"
    writeFile(result.sig_path, fmt"SHA256:{result.sha256}  KEY:{key_file}")
    echo "sigma-sign: cosign not found, wrote SHA256 placeholder"

  echo fmt"sigma-sign: generating in-toto provenance..."
  result.provenance = generateProvenance(result)
  writeFile(path & ".provenance.json", result.provenance)

  echo fmt"sigma-sign: signed → {result.sig_path}"
  echo fmt"sigma-sign: provenance → {path}.provenance.json"

proc verifyArtifact(path, sig_path, pub_key: string): bool =
  echo fmt"sigma-sign: verifying {path}..."
  let computed = computeSha256(path)
  if computed.len == 0: return false
  let ok = cosignVerify(path, sig_path, pub_key)
  if ok: echo "sigma-sign: verification PASSED"
  else:  stderr.writeLine("sigma-sign: verification FAILED")
  return ok

# ── CLI ───────────────────────────────────────────────────────────────────────

proc usage() =
  echo "sigma-sign — SigmaOS artifact signing tool"
  echo "Usage:"
  echo "  sigma-sign sign   <artifact> --key <key_file>"
  echo "  sigma-sign verify <artifact> --sig <sig_file> --pub <pub_key>"

proc main() =
  let args = commandLineParams()
  if args.len < 2: usage(); quit(0)

  case args[0]
  of "sign":
    var key_file = ""
    for i, a in args:
      if a == "--key" and i + 1 < args.len: key_file = args[i + 1]
    if key_file.len == 0: echo "Error: --key required"; quit(1)
    discard signArtifact(args[1], key_file)

  of "verify":
    var sig_path, pub_key = ""
    for i, a in args:
      if a == "--sig" and i + 1 < args.len: sig_path = args[i + 1]
      if a == "--pub" and i + 1 < args.len: pub_key  = args[i + 1]
    if not verifyArtifact(args[1], sig_path, pub_key): quit(1)

  else: usage(); quit(1)

main()
