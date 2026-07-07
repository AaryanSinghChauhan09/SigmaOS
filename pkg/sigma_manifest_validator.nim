# SigmaOS sigma_manifest_validator — Nim implementation
# Replaces pkg/sigma-manifest-validator.js (JavaScript).
# Compiles to a native binary (Nim → C → native); zero JS dependency.
# Validates .sigma package manifests against required schema fields.

import std/[os, strutils, json]

type
  ValidationResult = object
    valid:    bool
    errors:   seq[string]

const REQUIRED_FIELDS = [
  "name", "version", "architecture", "signature_alg",
  "epoch", "size_bytes", "sha256"
]

proc validateManifest(path: string): ValidationResult =
  result.valid = true

  if not fileExists(path):
    result.valid = false
    result.errors.add("File not found: " & path)
    return

  let raw = readFile(path)
  let node = parseJson(raw)

  for field in REQUIRED_FIELDS:
    if node.kind != JObject or not node.hasKey(field):
      result.valid = false
      result.errors.add("Missing required field: " & field)

  # Validate specific constraints
  if node.kind == JObject:
    if node.hasKey("signature_alg"):
      let alg = node["signature_alg"].getStr()
      if alg notin ["Ed25519", "ECDSA-P256"]:
        result.valid = false
        result.errors.add("Invalid signature_alg: " & alg & " (expected Ed25519 or ECDSA-P256)")

    if node.hasKey("architecture"):
      let arch = node["architecture"].getStr()
      if arch notin ["x86_64", "aarch64", "riscv64"]:
        result.valid = false
        result.errors.add("Unsupported architecture: " & arch)

proc main =
  let args = commandLineParams()
  if args.len == 0:
    echo "Usage: sigma_manifest_validator <manifest.sigma.json>"
    quit(1)

  for path in args:
    echo "Validating: ", path
    let res = validateManifest(path)
    if res.valid:
      echo "  ✓ Valid manifest"
    else:
      echo "  ✗ Invalid manifest:"
      for err in res.errors:
        echo "    - ", err
    echo ""

main()
