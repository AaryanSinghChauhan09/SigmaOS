#!/usr/bin/env python3
# SigmaOS Sign — file/package signing and verification CLI scaffold
# Upstream: gpg (https://gnupg.org/), sigstore (https://sigstore.dev/)
"""
SigmaOS sign: sign files and packages using an in-memory key model.
Subcommands:
  sign      <file> [--key <keyid>]    Sign a file, write <file>.sig
  verify    <file> [--sig <sigfile>]  Verify a signature
  keygen    --keyid <id> --email <e>  Generate a new signing key
  keylist                             List available signing keys
  keyremove <keyid>                   Remove a key
  info      <file>                    Show file metadata and sig status
"""
import argparse
import hashlib
import os
import sys
from typing import Dict, List, Optional

# ---------------------------------------------------------------------------
# In-memory key store (no real crypto — scaffold uses SHA-256 + key-id)
# ---------------------------------------------------------------------------
class SigningKey:
    def __init__(self, keyid: str, email: str):
        self.keyid = keyid
        self.email = email
        # Fingerprint = sha256(keyid + email) — deterministic for scaffold
        self.fingerprint = hashlib.sha256(f"{keyid}:{email}".encode()).hexdigest()[:16]

    def __str__(self) -> str:
        return f"{self.keyid} <{self.email}>  fp:{self.fingerprint}"


class SignatureRecord:
    def __init__(self, file_path: str, keyid: str, file_hash: str):
        self.file_path = file_path
        self.keyid = keyid
        self.file_hash = file_hash  # SHA-256 of file content at sign time
        # Signature = sha256(file_hash + keyid) — deterministic for scaffold
        self.sig_value = hashlib.sha256(f"{file_hash}:{keyid}".encode()).hexdigest()

    def to_sig_content(self) -> str:
        return (
            f"-----BEGIN SIGMAOS SIGNATURE-----\n"
            f"KeyID: {self.keyid}\n"
            f"FileHash: {self.file_hash}\n"
            f"Signature: {self.sig_value}\n"
            f"-----END SIGMAOS SIGNATURE-----\n"
        )


KEY_STORE: Dict[str, SigningKey] = {}
SIG_DB: Dict[str, SignatureRecord] = {}


def file_sha256(path: str) -> Optional[str]:
    try:
        h = hashlib.sha256()
        with open(path, "rb") as f:
            for chunk in iter(lambda: f.read(65536), b""):
                h.update(chunk)
        return h.hexdigest()
    except OSError:
        return None


def cmd_sign(args) -> None:
    if not os.path.exists(args.file):
        print(f"[sign] File not found: {args.file}", file=sys.stderr); return
    keyid = args.key or (list(KEY_STORE.keys())[0] if KEY_STORE else None)
    if not keyid or keyid not in KEY_STORE:
        print(f"[sign] No key found. Run 'keygen' first.", file=sys.stderr); return
    fhash = file_sha256(args.file)
    rec = SignatureRecord(args.file, keyid, fhash)
    sig_path = args.file + ".sig"
    with open(sig_path, "w") as f:
        f.write(rec.to_sig_content())
    SIG_DB[args.file] = rec
    print(f"[sign] Signed {args.file}")
    print(f"       Key     : {KEY_STORE[keyid]}")
    print(f"       SHA-256 : {fhash}")
    print(f"       Sig file: {sig_path}")


def cmd_verify(args) -> None:
    sig_path = args.sig or (args.file + ".sig")
    if not os.path.exists(sig_path):
        print(f"[sign] Signature file not found: {sig_path}", file=sys.stderr); return
    lines: Dict[str, str] = {}
    with open(sig_path) as f:
        for line in f:
            if ":" in line and "BEGIN" not in line and "END" not in line:
                k, v = line.strip().split(":", 1)
                lines[k.strip()] = v.strip()
    keyid = lines.get("KeyID", "")
    stored_hash = lines.get("FileHash", "")
    stored_sig = lines.get("Signature", "")
    current_hash = file_sha256(args.file) or ""
    expected_sig = hashlib.sha256(f"{current_hash}:{keyid}".encode()).hexdigest()
    if current_hash != stored_hash:
        print(f"[sign] INVALID — file hash mismatch"); return
    if expected_sig != stored_sig:
        print(f"[sign] INVALID — signature mismatch"); return
    print(f"[sign] VALID")
    print(f"       Key     : {keyid}")
    print(f"       SHA-256 : {current_hash}")


def cmd_keygen(args) -> None:
    if args.keyid in KEY_STORE:
        print(f"[sign] Key already exists: {args.keyid}", file=sys.stderr); return
    key = SigningKey(args.keyid, args.email)
    KEY_STORE[args.keyid] = key
    print(f"[sign] Generated key: {key}")


def cmd_keylist(_args) -> None:
    if not KEY_STORE:
        print("[sign] No keys in store"); return
    for k in KEY_STORE.values():
        print(f"  {k}")


def cmd_keyremove(args) -> None:
    if args.keyid not in KEY_STORE:
        print(f"[sign] Unknown key: {args.keyid}", file=sys.stderr); return
    del KEY_STORE[args.keyid]
    print(f"[sign] Removed key: {args.keyid}")


def cmd_info(args) -> None:
    fhash = file_sha256(args.file)
    sig_path = args.file + ".sig"
    print(f"File     : {args.file}")
    print(f"SHA-256  : {fhash or '(unreadable)'}")
    print(f"Sig file : {sig_path} ({'exists' if os.path.exists(sig_path) else 'missing'})")


def build_parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(prog="sigmaos-sign", description="SigmaOS file signing")
    sub = p.add_subparsers(dest="cmd", required=True)

    s = sub.add_parser("sign", help="Sign a file"); s.add_argument("file"); s.add_argument("--key", default=None)
    v = sub.add_parser("verify", help="Verify a signature"); v.add_argument("file"); v.add_argument("--sig", default=None)
    kg = sub.add_parser("keygen", help="Generate a signing key")
    kg.add_argument("--keyid", required=True); kg.add_argument("--email", required=True)
    sub.add_parser("keylist", help="List signing keys")
    kr = sub.add_parser("keyremove", help="Remove a key"); kr.add_argument("keyid")
    i = sub.add_parser("info", help="Show file and sig metadata"); i.add_argument("file")
    return p


def main(argv: List[str] = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    {"sign": cmd_sign, "verify": cmd_verify, "keygen": cmd_keygen,
     "keylist": cmd_keylist, "keyremove": cmd_keyremove, "info": cmd_info}[args.cmd](args)
    return 0


if __name__ == "__main__":
    sys.exit(main())
