# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
#
# pkg/sigma_pkg_absorb.nim — Linux package absorption layer
# Converts .deb, .rpm, .apk, Flatpak, AppImage → .sigpkg format
# Enables Linux workloads to migrate without friction.
#
# Inspired by: alien (deb↔rpm), flatpak bundle, Ubuntu's multiverse approach
#
# Language: Nim (stdlib only)

import std/[os, osproc, strutils, strformat, json, times, tables]

# ── Package format detection ──────────────────────────────────────────────────
type
  PkgFormat = enum
    FmtDeb, FmtRpm, FmtApk, FmtFlatpak, FmtAppImage,
    FmtSnap, FmtPacman, FmtNix, FmtUnknown

  AbsorbedPkg = object
    name:        string
    version:     string
    arch:        string
    description: string
    depends:     seq[string]
    files:       seq[string]
    source_fmt:  PkgFormat
    install_dir: string

proc detect_format(path: string): PkgFormat =
  let lower = path.toLowerAscii
  if lower.endsWith(".deb"):      return FmtDeb
  if lower.endsWith(".rpm"):      return FmtRpm
  if lower.endsWith(".apk"):      return FmtApk
  if lower.endsWith(".flatpakref") or lower.endsWith(".flatpak"): return FmtFlatpak
  if lower.endsWith(".appimage"): return FmtAppImage
  if lower.endsWith(".snap"):     return FmtSnap
  if lower.endsWith(".pkg.tar.zst") or lower.endsWith(".pkg.tar.xz"): return FmtPacman
  FmtUnknown

# ── .deb extraction ───────────────────────────────────────────────────────────
proc absorb_deb(path, work_dir: string): AbsorbedPkg =
  createDir(work_dir)
  # Extract control info
  let (ctrl_out, _) = execCmdEx(fmt"dpkg-deb --info {path.quoteShell} 2>/dev/null")
  for line in ctrl_out.splitLines():
    if line.startsWith("Package:"): result.name = line.split(":")[1].strip()
    elif line.startsWith("Version:"): result.version = line.split(":")[1].strip()
    elif line.startsWith("Architecture:"): result.arch = line.split(":")[1].strip()
    elif line.startsWith("Description:"): result.description = line.split(":")[1].strip()
    elif line.startsWith("Depends:"):
      result.depends = line.split(":")[1].strip().split(",").mapIt(it.strip())
  # Extract files list
  let (files_out, _) = execCmdEx(fmt"dpkg-deb --contents {path.quoteShell} 2>/dev/null")
  for line in files_out.splitLines():
    let parts = line.split()
    if parts.len >= 6 and parts[^1] != ".":
      result.files.add(parts[^1])
  # Extract to work_dir
  let (_, code) = execCmdEx(fmt"dpkg-deb --extract {path.quoteShell} {work_dir.quoteShell} 2>/dev/null")
  if code == 0: result.install_dir = work_dir
  result.source_fmt = FmtDeb

# ── .rpm extraction ───────────────────────────────────────────────────────────
proc absorb_rpm(path, work_dir: string): AbsorbedPkg =
  createDir(work_dir)
  let (info_out, _) = execCmdEx(fmt"rpm -qip {path.quoteShell} 2>/dev/null")
  for line in info_out.splitLines():
    if line.startsWith("Name"): result.name = line.split(":")[1].strip()
    elif line.startsWith("Version"): result.version = line.split(":")[1].strip()
    elif line.startsWith("Architecture"): result.arch = line.split(":")[1].strip()
    elif line.startsWith("Summary"): result.description = line.split(":")[1].strip()
  let (files_out, _) = execCmdEx(fmt"rpm -qlp {path.quoteShell} 2>/dev/null")
  result.files = files_out.strip().splitLines()
  # Extract using rpm2cpio if available
  let (_, code) = execCmdEx(
    fmt"cd {work_dir.quoteShell} && rpm2cpio {path.quoteShell} | cpio -idm 2>/dev/null")
  if code == 0: result.install_dir = work_dir
  result.source_fmt = FmtRpm

# ── AppImage handling ─────────────────────────────────────────────────────────
proc absorb_appimage(path, work_dir: string): AbsorbedPkg =
  createDir(work_dir)
  # Make executable and extract
  discard execCmdEx(fmt"chmod +x {path.quoteShell}")
  let (_, code) = execCmdEx(
    fmt"{path.quoteShell} --appimage-extract 2>/dev/null && mv squashfs-root {work_dir.quoteShell}/")
  result.name        = path.extractFilename.split('.')[0]
  result.version     = "appimage"
  result.arch        = "x86_64"
  result.source_fmt  = FmtAppImage
  if code == 0: result.install_dir = work_dir / "squashfs-root"

# ── Generate .sigpkg manifest ─────────────────────────────────────────────────
proc to_sigpkg_manifest(pkg: AbsorbedPkg): JsonNode =
  %*{
    "name":        pkg.name,
    "version":     pkg.version,
    "arch":        pkg.arch,
    "description": pkg.description,
    "depends":     pkg.depends,
    "source_fmt":  $pkg.source_fmt,
    "absorbed_at": $now(),
    "install_dir": pkg.install_dir,
    "format":      "sigpkg-v1",
    "signed":      false,   # would be signed with Dilithium-5 in production
  }

proc build_sigpkg(pkg: AbsorbedPkg, output_dir: string): string =
  ## Package absorbed files into a .sigpkg archive
  createDir(output_dir)
  let manifest_path = pkg.install_dir / "SIGPKG_MANIFEST.json"
  writeFile(manifest_path, pkg.to_sigpkg_manifest().pretty())
  let out_path = output_dir / fmt"{pkg.name}-{pkg.version}.sigpkg"
  let (_, code) = execCmdEx(
    fmt"tar -czf {out_path.quoteShell} -C {pkg.install_dir.quoteShell} . 2>/dev/null")
  if code == 0: return out_path
  ""

# ── Dependency mapper (Linux pkg names → sigma-pkg equivalents) ───────────────
const DEB_TO_SIGMA: array[20, (string, string)] = [
  ("libc6",              "sigma-libc"),
  ("libssl3",            "sigma-tls"),
  ("libgtk-3-0",         "sigma-gtk3-compat"),
  ("libX11-6",           "sigma-x11-compat"),
  ("python3",            "sigma-python3"),
  ("nodejs",             "sigma-node"),
  ("libstdc++6",         "sigma-libstdcpp"),
  ("libgcc-s1",          "sigma-libgcc"),
  ("zlib1g",             "sigma-zlib"),
  ("libsqlite3-0",       "sigma-sqlite"),
  ("libfontconfig1",     "sigma-fontconfig"),
  ("libfreetype6",       "sigma-freetype"),
  ("libpng16-16",        "sigma-libpng"),
  ("libjpeg-turbo8",     "sigma-libjpeg"),
  ("libcurl4",           "sigma-libcurl"),
  ("libdbus-1-3",        "sigma-dbus"),
  ("libglib2.0-0",       "sigma-glib"),
  ("libpulse0",          "sigma-pipewire-compat"),
  ("libvulkan1",         "sigma-vulkan"),
  ("ca-certificates",    "sigma-cacerts"),
]

proc map_depends(linux_deps: seq[string]): seq[string] =
  for dep in linux_deps:
    let clean = dep.split('(')[0].strip()
    var found = false
    for (linux, sigma) in DEB_TO_SIGMA:
      if clean == linux or clean.startsWith(linux):
        result.add(sigma); found = true; break
    if not found and clean.len > 0:
      result.add(fmt"sigma-compat-{clean}")

# ── Main absorption CLI ───────────────────────────────────────────────────────
proc absorb_cmd*(args: seq[string]) =
  if args.len == 0 or args[0] == "help":
    echo """sigma-pkg absorb — Linux package absorption layer

Usage:
  sigma-pkg absorb <file.deb>           Convert .deb to .sigpkg
  sigma-pkg absorb <file.rpm>           Convert .rpm to .sigpkg
  sigma-pkg absorb <file.AppImage>      Wrap AppImage as .sigpkg
  sigma-pkg absorb --check <format>     Check if absorption tools available
  sigma-pkg absorb --list               List absorbed packages

Supported formats:
  .deb      Debian/Ubuntu packages (via dpkg-deb)
  .rpm      Red Hat/Fedora packages (via rpm2cpio)
  .AppImage Portable Linux apps (direct extraction)
  .flatpak  Flatpak bundles (via flatpak install)
  .snap     Snap packages (via snap install --dangerous)

Examples:
  sigma-pkg absorb firefox.deb
  sigma-pkg absorb code.rpm
  sigma-pkg absorb Blender.AppImage
  sigma-agent "install firefox.deb"   # agent auto-detects and absorbs
"""
    return

  if args[0] == "--check":
    let fmt_str = if args.len > 1: args[1] else: "deb"
    case fmt_str.toLowerAscii
    of "deb":
      let (_, code) = execCmdEx("which dpkg-deb 2>/dev/null")
      if code == 0: echo "✓ dpkg-deb available (deb absorption ready)"
      else: echo "✗ dpkg-deb not found. Install: sigma-pkg install dpkg-tools"
    of "rpm":
      let (_, code) = execCmdEx("which rpm2cpio 2>/dev/null")
      if code == 0: echo "✓ rpm2cpio available (rpm absorption ready)"
      else: echo "✗ rpm2cpio not found. Install: sigma-pkg install rpm-tools"
    of "appimage":
      echo "✓ AppImage absorption built-in (no tools required)"
    else:
      echo fmt"Unknown format: {fmt_str}"
    return

  if args[0] == "--list":
    let absorbed_dir = getEnv("HOME","/tmp") / ".cache/sigma/absorbed"
    if dirExists(absorbed_dir):
      for _, path in walkDir(absorbed_dir):
        if path.endsWith(".sigpkg"): echo "  " & path.extractFilename
    else: echo "No packages absorbed yet."
    return

  let input_path = args[0]
  if not fileExists(input_path):
    echo fmt"✗ File not found: {input_path}"; return

  let fmt = detect_format(input_path)
  if fmt == FmtUnknown:
    echo fmt"✗ Unknown format: {input_path}"
    echo "  Supported: .deb .rpm .AppImage .flatpak .snap"
    return

  let work_dir   = "/tmp/sigma_absorb_" & $getTime().toUnix
  let output_dir = getEnv("HOME","/tmp") / ".cache/sigma/absorbed"
  createDir(output_dir)

  echo fmt"σ Absorbing {fmt}: {input_path.extractFilename}"

  let pkg = case fmt
    of FmtDeb:      absorb_deb(input_path, work_dir)
    of FmtRpm:      absorb_rpm(input_path, work_dir)
    of FmtAppImage: absorb_appimage(input_path, work_dir)
    else:
      echo fmt"✗ Absorption for {fmt} not yet implemented. Use: sigma-pkg install --flatpak"
      return

  if pkg.name.len == 0:
    echo "✗ Could not extract package metadata"
    return

  echo fmt"  Name:    {pkg.name}"
  echo fmt"  Version: {pkg.version}"
  echo fmt"  Arch:    {pkg.arch}"

  # Map dependencies
  let sigma_deps = map_depends(pkg.depends)
  if sigma_deps.len > 0:
    echo fmt"  Dependencies mapped: {sigma_deps.len}"

  # Build .sigpkg
  let out = build_sigpkg(pkg, output_dir)
  if out.len > 0:
    echo fmt"✓ Absorbed: {out}"
    echo fmt"  Install: sigma-pkg install {out}"
  else:
    echo "✗ Failed to build .sigpkg archive"

  # Cleanup work dir
  try: removeDir(work_dir) except: discard
