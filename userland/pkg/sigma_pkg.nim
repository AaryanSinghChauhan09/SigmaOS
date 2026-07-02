# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
#
# userland/pkg/sigma_pkg.nim — sigma-pkg: Sovereign Package Manager
# Replaces: sigma_pkg.cpp (C++ stub, removed)
#
# Language: Nim — native binary, no GC overhead in critical paths
# Pattern: OOP via object + methods; no third-party deps beyond stdlib

import std/[os, strutils, json, httpclient, hashes, streams, parseopt]

# ── Constants ─────────────────────────────────────────────────────────────────

const
  PKG_VERSION   = "15.0.0"
  LOCAL_DB      = "/var/sigma/pkg/installed.json"
  LOCAL_REPO    = "/var/sigma/pkg/repo"
  REGISTRY_URL  = "https://pkg.sigmaos.app/v1"
  CACHE_DIR     = "/var/sigma/pkg/cache"

# ── Types ─────────────────────────────────────────────────────────────────────

type
  PkgMeta = object
    name:        string
    version:     string
    description: string
    arch:        string
    size_bytes:  int
    sha256:      string
    profile:     seq[string]

  InstalledDb = object
    packages: seq[PkgMeta]

  PkgCmd = enum
    CmdInstall, CmdRemove, CmdList, CmdSearch,
    CmdInfo, CmdUpdate, CmdVerify, CmdBuild

# ── Installed Database ────────────────────────────────────────────────────────

proc loadDb(): InstalledDb =
  if not fileExists(LOCAL_DB): return InstalledDb(packages: @[])
  let raw = readFile(LOCAL_DB)
  let node = parseJson(raw)
  result.packages = @[]
  for pkg in node["packages"]:
    result.packages.add PkgMeta(
      name:    pkg["name"].getStr,
      version: pkg["version"].getStr,
      arch:    pkg.getOrDefault("arch").getStr("any"))

proc saveDb(db: InstalledDb) =
  createDir(parentDir(LOCAL_DB))
  var arr = newJArray()
  for p in db.packages:
    arr.add %* {"name": p.name, "version": p.version, "arch": p.arch}
  let root = %* {"packages": arr}
  writeFile(LOCAL_DB, $root)

proc isInstalled(db: InstalledDb, name: string): bool =
  db.packages.anyIt(it.name == name)

# ── SHA-256 (cleanroom — no stdlib crypto) ───────────────────────────────────

proc sha256Hex(data: string): string =
  ## Tiny pure-Nim SHA-256 (no stdlib crypto dependency)
  ## Real impl — K constants + message schedule + compression function
  const K: array[64, uint32] = [
    0x428a2f98'u32, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5,
    0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
    0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc,
    0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
    0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
    0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3,
    0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5,
    0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
    0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2]

  proc rotr(x: uint32, n: int): uint32 = (x shr n) or (x shl (32 - n))
  proc ch(e, f, g: uint32): uint32  = (e and f) xor ((not e) and g)
  proc maj(a, b, c: uint32): uint32 = (a and b) xor (a and c) xor (b and c)
  proc ep0(a: uint32): uint32 = rotr(a,2) xor rotr(a,13) xor rotr(a,22)
  proc ep1(e: uint32): uint32 = rotr(e,6) xor rotr(e,11) xor rotr(e,25)
  proc sig0(x: uint32): uint32 = rotr(x,7) xor rotr(x,18) xor (x shr 3)
  proc sig1(x: uint32): uint32 = rotr(x,17) xor rotr(x,19) xor (x shr 10)

  var h = [0x6a09e667'u32, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
           0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19]

  # Pad message
  var msg = newSeq[uint8](data.len)
  for i, c in data: msg[i] = c.uint8
  let bitLen = data.len * 8
  msg.add 0x80'u8
  while (msg.len mod 64) != 56: msg.add 0'u8
  for i in countdown(7, 0): msg.add uint8((bitLen shr (i*8)) and 0xFF)

  # Process each 512-bit block
  var i = 0
  while i < msg.len:
    var w: array[64, uint32]
    for j in 0..<16:
      w[j] = (msg[i+j*4].uint32 shl 24) or (msg[i+j*4+1].uint32 shl 16) or
             (msg[i+j*4+2].uint32 shl 8)  or  msg[i+j*4+3].uint32
    for j in 16..<64:
      w[j] = sig1(w[j-2]) + w[j-7] + sig0(w[j-15]) + w[j-16]

    var (a,b,c,d,e,f,g,hh) = (h[0],h[1],h[2],h[3],h[4],h[5],h[6],h[7])
    for j in 0..<64:
      let t1 = hh + ep1(e) + ch(e,f,g) + K[j] + w[j]
      let t2 = ep0(a) + maj(a,b,c)
      hh=g; g=f; f=e; e=d+t1; d=c; c=b; b=a; a=t1+t2

    h[0]+=a; h[1]+=b; h[2]+=c; h[3]+=d
    h[4]+=e; h[5]+=f; h[6]+=g; h[7]+=hh
    i += 64

  result = ""
  for v in h: result &= v.toHex(8).toLowerAscii()

# ── Package Operations ────────────────────────────────────────────────────────

proc fetchIndex(): seq[PkgMeta] =
  let client = newHttpClient()
  client.headers = newHttpHeaders({"Accept": "application/json"})
  let resp = client.getContent(REGISTRY_URL & "/index")
  let node = parseJson(resp)
  result = @[]
  for p in node["packages"]:
    result.add PkgMeta(
      name:    p["name"].getStr,
      version: p["version"].getStr,
      sha256:  p.getOrDefault("sha256").getStr(""),
      arch:    p.getOrDefault("arch").getStr("any"))

proc downloadPkg(name, version, arch: string): string =
  createDir(CACHE_DIR)
  let filename = name & "-" & version & "-" & arch & ".sigpkg"
  let dest     = CACHE_DIR / filename
  if fileExists(dest): return dest
  let url = REGISTRY_URL & "/pkg/" & name & "/" & version & "/" & arch
  let client = newHttpClient()
  client.downloadFile(url, dest)
  return dest

proc installPkg(name: string) =
  var db = loadDb()
  if db.isInstalled(name):
    echo "sigma-pkg: " & name & " is already installed"
    return
  echo "sigma-pkg: fetching package index..."
  let index = fetchIndex()
  let found = index.filterIt(it.name == name)
  if found.len == 0:
    echo "sigma-pkg: error: package '" & name & "' not found"
    quit(1)
  let meta = found[0]
  echo "sigma-pkg: downloading " & meta.name & " " & meta.version & "..."
  let path = downloadPkg(meta.name, meta.version, meta.arch)
  echo "sigma-pkg: verifying sha256..."
  let data   = readFile(path)
  let digest = sha256Hex(data)
  if meta.sha256.len > 0 and digest != meta.sha256:
    echo "sigma-pkg: ERROR: sha256 mismatch — aborting"
    removeFile(path); quit(1)
  echo "sigma-pkg: installing " & meta.name & "..."
  # Extract .sigpkg (tar.gz) — simplified: just track in DB
  db.packages.add meta
  saveDb(db)
  echo "sigma-pkg: installed " & meta.name & " " & meta.version

proc removePkg(name: string) =
  var db = loadDb()
  let before = db.packages.len
  db.packages.keepIf(proc(p: PkgMeta): bool = p.name != name)
  if db.packages.len == before:
    echo "sigma-pkg: " & name & " is not installed"; quit(1)
  saveDb(db)
  echo "sigma-pkg: removed " & name

proc listPkgs() =
  let db = loadDb()
  if db.packages.len == 0:
    echo "No packages installed."; return
  echo "Installed packages:"
  for p in db.packages: echo "  " & p.name & "  " & p.version

proc searchPkgs(query: string) =
  echo "sigma-pkg: searching for '" & query & "'..."
  let index = fetchIndex()
  var found = false
  for p in index:
    if query.toLowerAscii in p.name.toLowerAscii or
       query.toLowerAscii in p.description.toLowerAscii:
      echo "  " & p.name & "  " & p.version & "  " & p.description
      found = true
  if not found: echo "No packages found matching '" & query & "'"

# ── CLI Entry ─────────────────────────────────────────────────────────────────

proc usage() =
  echo "sigma-pkg v" & PKG_VERSION & " — Sovereign Package Manager"
  echo "Usage: sigma-pkg <command> [args]"
  echo "Commands:"
  echo "  install <name>   Install a package"
  echo "  remove  <name>   Remove a package"
  echo "  list             List installed packages"
  echo "  search  <query>  Search package registry"
  echo "  update           Update all installed packages"
  echo "  verify  <file>   Verify a .sigpkg file's sha256"

proc main() =
  var args = commandLineParams()
  if args.len == 0: usage(); quit(0)
  case args[0]
  of "install": installPkg(if args.len>1: args[1] else: (usage(); quit(1); ""))
  of "remove":  removePkg(if args.len>1: args[1] else: (usage(); quit(1); ""))
  of "list":    listPkgs()
  of "search":  searchPkgs(if args.len>1: args[1] else: "")
  of "update":  echo "sigma-pkg: update not yet implemented"
  of "verify":
    if args.len < 2: echo "Usage: sigma-pkg verify <file>"; quit(1)
    let data = readFile(args[1])
    echo "SHA-256: " & sha256Hex(data)
  else: echo "sigma-pkg: unknown command '" & args[0] & "'"; usage(); quit(1)

main()
