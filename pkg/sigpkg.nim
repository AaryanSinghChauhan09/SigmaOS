# SPDX-License-Identifier: GPL-2.0-or-later
# ==========================================================================
# SigmaOS: Sovereign Package Manager (Nim)
# Replaces: include/core/SovereignPackageManager.h
# ==========================================================================

type
  PackageState* = enum
    pkgAvailable, pkgInstalled, pkgBroken

  Package* = object
    name*:    string
    version*: string
    state*:   PackageState

  SigPkg* = object
    packages*: seq[Package]
    repoURL*:  string

proc newSigPkg*(url: string): SigPkg =
  result.packages = newSeq[Package]()
  result.repoURL  = url

proc addPackage*(pkg: var SigPkg; name, version: string) =
  pkg.packages.add(Package(name: name, version: version, state: pkgAvailable))

proc installPackage*(pkg: var SigPkg; name: string): bool =
  for i in 0 ..< pkg.packages.len:
    if pkg.packages[i].name == name:
      pkg.packages[i].state = pkgInstalled
      return true
  return false

proc removePackage*(pkg: var SigPkg; name: string): bool =
  for i in 0 ..< pkg.packages.len:
    if pkg.packages[i].name == name:
      pkg.packages.del(i)
      return true
  return false

proc listInstalled*(pkg: SigPkg): seq[string] =
  result = newSeq[string]()
  for p in pkg.packages:
    if p.state == pkgInstalled:
      result.add(p.name)
