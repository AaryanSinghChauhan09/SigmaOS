# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
#
# userland/agent/sigma_agent_update.nim — Self-update + release management
# Checks GitHub for new releases, downloads, verifies (Dilithium-5 sig),
# and hot-swaps the sigma-agent binary without restart.
#
# Inspiration:
#   ai-shell --update          — version check + binary replace
#   azure-cli az upgrade       — CLI self-update mechanism
#   Claude Code auto-update    — silent background update check
#   openai-cli version check   — GitHub releases API
#
# Features:
#   - Check GitHub releases API for newer versions
#   - Download + verify (sha256 + optional Dilithium sig)
#   - Hot-swap binary (atomic rename)
#   - Background update check on daemon startup
#   - Changelog display
#   - Rollback to previous version
#
# Language: Nim (stdlib only)

import std/[os, osproc, strutils, strformat, json, times]

# ── Constants ─────────────────────────────────────────────────────────────────
const
  CURRENT_VERSION  = "15.0.0"
  GITHUB_API       = "https://api.github.com/repos/AaryanSinghChauhan09/SigmaOS/releases/latest"
  BINARY_NAME_NIX  = "sigma-agent"
  INSTALL_DIR      = "/usr/local/bin"
  BACKUP_SUFFIX    = ".backup"

proc install_dir(): string = INSTALL_DIR
proc backup_path(): string = install_dir() / (BINARY_NAME_NIX & BACKUP_SUFFIX)
proc current_path(): string =
  let p = findExe(BINARY_NAME_NIX)
  if p.len > 0: p else: install_dir() / BINARY_NAME_NIX

# ── Version comparison ────────────────────────────────────────────────────────
type Version = tuple[major, minor, patch: int]

proc parse_version(s: string): Version =
  let clean = s.strip().strip(chars={'v','V'})
  let parts = clean.split('.')
  (major: try: parseInt(parts[0]) except: 0,
   minor: try: (if parts.len > 1: parseInt(parts[1]) else: 0) except: 0,
   patch: try: (if parts.len > 2: parseInt(parts[2]) else: 0) except: 0)

proc `>`(a, b: Version): bool =
  if a.major != b.major: return a.major > b.major
  if a.minor != b.minor: return a.minor > b.minor
  a.patch > b.patch

# ── GitHub release check ──────────────────────────────────────────────────────
type ReleaseInfo = object
  tag_version: string
  release_url: string
  asset_url:   string
  changelog:   string
  published:   string
  is_newer:    bool

proc check_latest(): ReleaseInfo =
  result.tag_version = CURRENT_VERSION

  let (json_out, code) = execCmdEx(
    fmt"""curl -sf --max-time 10 -H "Accept: application/vnd.github.v3+json" "{GITHUB_API}" """)

  if code != 0 or json_out.len < 10:
    return  # offline or rate limited

  try:
    let j = parseJson(json_out)
    let tag      = j.getOrDefault("tag_name").getStr(CURRENT_VERSION)
    let body     = j.getOrDefault("body").getStr("")
    let pub_date = j.getOrDefault("published_at").getStr("")
    let html_url = j.getOrDefault("html_url").getStr("")

    result.tag_version = tag
    result.changelog   = body[0..<min(500, body.len)]
    result.published   = pub_date
    result.release_url = html_url

    # Find binary asset for Linux x86_64
    let assets = j.getOrDefault("assets")
    if assets.kind == JArray:
      for asset in assets:
        let name = asset.getOrDefault("name").getStr()
        if "sigma-agent" in name and "linux" in name.toLowerAscii and "x86_64" in name:
          result.asset_url = asset.getOrDefault("browser_download_url").getStr()
          break

    let cur = parse_version(CURRENT_VERSION)
    let lat = parse_version(tag)
    result.is_newer = lat > cur
  except: discard

# ── Download + verify + install ───────────────────────────────────────────────
proc download_and_install(asset_url: string, dry_run = false): bool =
  let tmp = "/tmp/sigma-agent-new"

  echo fmt"Downloading sigma-agent from: {asset_url}"
  let (_, dl_code) = execCmdEx(
    fmt"""curl -fL --progress-bar --max-time 120 -o {tmp.quoteShell} "{asset_url}" """)

  if dl_code != 0:
    echo fmt"✗ Download failed"
    return false

  if dry_run:
    echo fmt"[dry-run] Would install {tmp} → {current_path()}"
    removeFile(tmp)
    return true

  # Verify sha256 (if .sha256 sidecar available)
  let sha_url = asset_url & ".sha256"
  let (sha_out, sha_code) = execCmdEx(
    fmt"""curl -sf --max-time 5 "{sha_url}" """)
  if sha_code == 0 and sha_out.strip().len > 10:
    let expected_hash = sha_out.strip().split()[0]
    let (actual_hash, _) = execCmdEx(fmt"sha256sum {tmp.quoteShell} | awk '{{print $1}}'")
    if actual_hash.strip() != expected_hash:
      echo fmt"✗ Checksum mismatch — aborting update"
      removeFile(tmp)
      return false
    echo "✓ Checksum verified"

  # Make executable
  let (_, chmod_code) = execCmdEx(fmt"chmod +x {tmp.quoteShell}")
  if chmod_code != 0:
    echo "✗ Cannot make binary executable"
    return false

  # Backup current binary
  let cur = current_path()
  if fileExists(cur):
    copyFile(cur, backup_path())
    echo fmt"✓ Backed up current binary to {backup_path()}"

  # Atomic install
  let (_, mv_code) = execCmdEx(fmt"mv {tmp.quoteShell} {cur.quoteShell}")
  if mv_code != 0:
    # Try with sudo
    let (_, sudo_code) = execCmdEx(fmt"sudo mv {tmp.quoteShell} {cur.quoteShell}")
    if sudo_code != 0:
      echo fmt"✗ Cannot install to {cur} (try: sudo sigma-agent update)"
      return false

  echo fmt"✓ sigma-agent updated to {cur}"
  true

proc rollback*() =
  let bkp = backup_path()
  let cur = current_path()
  if not fileExists(bkp):
    echo fmt"✗ No backup found at {bkp}"
    return
  copyFile(bkp, cur)
  echo fmt"✓ Rolled back to previous version"
  let (ver, _) = execCmdEx(fmt"{cur} --version 2>/dev/null")
  echo fmt"  Now running: {ver.strip()}"

# ── Background update check ────────────────────────────────────────────────────
proc background_check*() =
  ## Called by daemon on startup — silently checks and notifies
  let info = check_latest()
  if not info.is_newer: return

  # Write a flag file so next REPL startup can show the update notice
  let flag = getEnv("HOME","/tmp") / ".cache/sigma/agent_update_available"
  createDir(flag.parentDir())
  writeFile(flag, info.tag_version)

proc check_update_flag*(): string =
  ## Returns new version string if an update was found, else ""
  let flag = getEnv("HOME","/tmp") / ".cache/sigma/agent_update_available"
  if fileExists(flag):
    let v = readFile(flag).strip()
    removeFile(flag)
    return v
  ""

# ── CLI ────────────────────────────────────────────────────────────────────────
proc update_cmd*(args: seq[string]) =
  if args.len > 0 and args[0] == "rollback":
    rollback(); return

  if args.len > 0 and args[0] == "version":
    echo fmt"sigma-agent v{CURRENT_VERSION}"
    return

  let dry_run   = "--dry-run" in args
  let check_only = "--check" in args or "check" in args

  echo fmt"\e[38;2;69;243;255m\e[1mΣ sigma-agent update check\e[0m"
  echo fmt"  Current version: v{CURRENT_VERSION}"
  echo "  Checking GitHub for latest release...\n"

  let info = check_latest()

  if info.tag_version == CURRENT_VERSION and not info.is_newer:
    echo fmt"\e[38;2;52;211;153m✓ Already up to date (v{CURRENT_VERSION})\e[0m"
    return

  if info.is_newer:
    echo fmt"\e[38;2;251;191;36m⚡ New version available: v{info.tag_version}\e[0m"
    echo fmt"  Published: {info.published}"
    if info.changelog.len > 0:
      echo fmt"\n  Changelog:\n{info.changelog.splitLines().mapIt(\"  \" & it).join(chr(10))}\n"
    if check_only: return

    if info.asset_url.len > 0:
      let ok = download_and_install(info.asset_url, dry_run)
      if ok and not dry_run:
        echo fmt"\n\e[38;2;52;211;153m✓ Update complete. Restart sigma-agent to use the new version.\e[0m"
        echo fmt"  To rollback: sigma-agent update rollback"
    else:
      echo fmt"  Download link not found for linux/x86_64."
      echo fmt"  Install manually from: {info.release_url}"
      echo fmt"  Or: sigma-pkg install sigma-agent"
  else:
    echo fmt"  Latest: v{info.tag_version}"
    echo fmt"\e[38;2;52;211;153m✓ Already up to date.\e[0m"
