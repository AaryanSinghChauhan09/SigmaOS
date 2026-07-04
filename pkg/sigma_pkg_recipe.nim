# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
#
# pkg/sigma_pkg_recipe.nim — Declarative package recipe system
# Inspired by NixOS derivations + Arch PKGBUILD + Flatpak manifests
# Enables: reproducible builds, community packages, AUR-style contributions
#
# Recipe format (sigma_pkg_registry/recipes/<name>.toml):
#   [package]
#   name, version, description, license, homepage, arch
#   [source]
#   url, sha256, sig_dilithium5 (optional)
#   [build]
#   steps = ["cargo build --release", "strip target/release/myapp"]
#   [install]
#   bins = ["target/release/myapp → /usr/bin/myapp"]
#   [dependencies]
#   runtime = ["sigma-libc", "sigma-tls"]
#   build   = ["sigma-sdk", "rust-toolchain"]
#   [security]
#   pledge  = ["stdio", "rpath", "inet"]
#   unveil  = ["/usr/share/myapp:r", "/home:rw"]
#
# Language: Nim (stdlib only)

import std/[os, osproc, strutils, strformat, json, times, tables, hashes]

# ── Recipe types ──────────────────────────────────────────────────────────────
type
  RecipeSource = object
    url:            string
    sha256:         string
    sig_dilithium5: string  # post-quantum signature
    git_url:        string
    git_ref:        string

  RecipeBuild = object
    steps:    seq[string]
    env:      Table[string, string]
    workdir:  string

  RecipeInstall = object
    bins:     seq[string]   # "src → /usr/bin/name"
    libs:     seq[string]
    shares:   seq[string]
    services: seq[string]   # systemd-style service files

  RecipeSecurity = object
    pledge:  seq[string]
    unveil:  seq[string]

  PackageRecipe = object
    name:        string
    version:     string
    description: string
    license:     string
    homepage:    string
    arch:        seq[string]
    source:      RecipeSource
    build:       RecipeBuild
    install:     RecipeInstall
    deps_runtime: seq[string]
    deps_build:   seq[string]
    security:    RecipeSecurity
    maintainer:  string
    created:     string

# ── TOML-lite parser ──────────────────────────────────────────────────────────
proc parse_recipe(path: string): PackageRecipe =
  result.created = $now()
  result.arch = @["x86_64"]
  result.build.env = initTable[string, string]()
  var section = ""
  for raw_line in lines(path):
    let line = raw_line.strip()
    if line.len == 0 or line.startsWith("#"): continue
    if line.startsWith("[") and line.endsWith("]"):
      section = line[1..^2].toLowerAscii; continue
    if "=" notin line: continue
    let parts = line.split("=", 1)
    let k = parts[0].strip()
    let v = parts[1].strip().strip(chars={'"', '\''})
    let v_list = v.strip(chars={'[',']'}).split(",").mapIt(it.strip().strip(chars={'"','\''}))
    case section
    of "package":
      case k
      of "name":        result.name        = v
      of "version":     result.version     = v
      of "description": result.description = v
      of "license":     result.license     = v
      of "homepage":    result.homepage    = v
      of "maintainer":  result.maintainer  = v
      of "arch":        result.arch        = v_list
    of "source":
      case k
      of "url":            result.source.url    = v
      of "sha256":         result.source.sha256 = v
      of "sig_dilithium5": result.source.sig_dilithium5 = v
      of "git_url":        result.source.git_url = v
      of "git_ref":        result.source.git_ref = v
    of "build":
      case k
      of "steps":   result.build.steps  = v_list
      of "workdir": result.build.workdir = v
      else:         result.build.env[k] = v
    of "install":
      case k
      of "bins":     result.install.bins     = v_list
      of "libs":     result.install.libs     = v_list
      of "shares":   result.install.shares   = v_list
      of "services": result.install.services = v_list
    of "dependencies":
      case k
      of "runtime": result.deps_runtime = v_list
      of "build":   result.deps_build   = v_list
    of "security":
      case k
      of "pledge": result.security.pledge = v_list
      of "unveil": result.security.unveil = v_list

# ── Recipe builder ─────────────────────────────────────────────────────────────
proc build_from_recipe*(recipe: PackageRecipe, build_dir, output_dir: string): bool =
  createDir(build_dir); createDir(output_dir)
  echo fmt"Σ Building: {recipe.name}-{recipe.version}"

  # 1. Fetch source
  if recipe.source.url.len > 0:
    echo "  Fetching source..."
    let archive = build_dir / fmt"{recipe.name}-{recipe.version}.tar.gz"
    let (_, code) = execCmdEx(
      fmt"curl -fL --progress-bar -o {archive.quoteShell} {recipe.source.url.quoteShell}")
    if code != 0: echo "✗ Fetch failed"; return false

    # Verify sha256
    if recipe.source.sha256.len > 0:
      let (hash, _) = execCmdEx(fmt"sha256sum {archive.quoteShell} | awk '{{print $1}}'")
      if hash.strip() != recipe.source.sha256:
        echo "✗ Checksum mismatch — aborting"; return false
      echo "  ✓ Checksum verified"

    discard execCmdEx(fmt"tar -xf {archive.quoteShell} -C {build_dir.quoteShell}")

  elif recipe.source.git_url.len > 0:
    echo "  Cloning source..."
    let (_, code) = execCmdEx(
      fmt"git clone --depth 1 --branch {recipe.source.git_ref} {recipe.source.git_url.quoteShell} {build_dir.quoteShell}/src")
    if code != 0: echo "✗ Clone failed"; return false

  # 2. Install build dependencies
  for dep in recipe.deps_build:
    discard execCmdEx(fmt"sigma-pkg install {dep.quoteShell} 2>/dev/null")

  # 3. Run build steps
  let work_dir = if recipe.build.workdir.len > 0: build_dir / recipe.build.workdir
                 else: build_dir
  for step in recipe.build.steps:
    echo fmt"  + {step}"
    # Set build env vars
    for k, v in recipe.build.env: putEnv(k, v)
    let (out, code) = execCmdEx(fmt"cd {work_dir.quoteShell} && {step}")
    if code != 0:
      echo fmt"✗ Build step failed: {step}\n{out[0..<min(200,out.len)]}"
      return false

  # 4. Install into staging directory
  let stage_dir = build_dir / "stage"
  createDir(stage_dir)
  for entry in recipe.install.bins:
    let parts = entry.split("→")
    if parts.len == 2:
      let src = work_dir / parts[0].strip()
      let dst_rel = parts[1].strip()
      let dst = stage_dir & dst_rel
      createDir(dst.parentDir())
      if fileExists(src): copyFile(src, dst)
  for entry in recipe.install.libs:
    let parts = entry.split("→")
    if parts.len == 2:
      let src = work_dir / parts[0].strip()
      let dst = stage_dir & parts[1].strip()
      createDir(dst.parentDir())
      if fileExists(src): copyFile(src, dst)

  # 5. Write SIGPKG_MANIFEST
  let manifest = %*{
    "name":    recipe.name, "version": recipe.version,
    "description": recipe.description, "license": recipe.license,
    "arch":    recipe.arch, "deps":    recipe.deps_runtime,
    "pledge":  recipe.security.pledge, "unveil":  recipe.security.unveil,
    "built_at": $now(), "format": "sigpkg-v1"}
  writeFile(stage_dir / "SIGPKG_MANIFEST.json", manifest.pretty())

  # 6. Create .sigpkg archive
  let out_path = output_dir / fmt"{recipe.name}-{recipe.version}.sigpkg"
  let (_, pkg_code) = execCmdEx(
    fmt"tar -czf {out_path.quoteShell} -C {stage_dir.quoteShell} . 2>/dev/null")
  if pkg_code != 0:
    echo "✗ Failed to create .sigpkg"; return false

  echo fmt"✓ Built: {out_path}"
  true

# ── Community recipe scaffold ─────────────────────────────────────────────────
const RECIPE_TEMPLATE = """
[package]
name        = "{NAME}"
version     = "0.1.0"
description = "A SigmaOS package"
license     = "MIT"
homepage    = "https://github.com/your/project"
maintainer  = "Your Name <you@example.com>"
arch        = ["x86_64", "aarch64"]

[source]
url    = "https://github.com/your/project/archive/refs/tags/v0.1.0.tar.gz"
sha256 = ""   # fill in: sha256sum <tarball>

[build]
steps  = [
  "cargo build --release",
  "strip target/release/{NAME}",
]

[install]
bins   = ["target/release/{NAME} → /usr/bin/{NAME}"]

[dependencies]
runtime = ["sigma-libc"]
build   = ["rust-toolchain"]

[security]
pledge  = ["stdio", "rpath"]
unveil  = ["/home:r", "/tmp:rw"]
"""

proc scaffold_recipe*(name, output_dir: string) =
  createDir(output_dir)
  let path = output_dir / fmt"{name}.toml"
  writeFile(path, RECIPE_TEMPLATE.replace("{NAME}", name))
  echo fmt"✓ Recipe scaffolded: {path}"
  echo fmt"  Edit the recipe, then: sigma-pkg build {path}"

# ── CLI ────────────────────────────────────────────────────────────────────────
proc recipe_cmd*(args: seq[string]) =
  if args.len == 0 or args[0] == "help":
    echo """sigma-pkg recipe — Declarative package recipe system

Usage:
  sigma-pkg recipe new <name>          Scaffold a new recipe
  sigma-pkg recipe build <file.toml>   Build a package from recipe
  sigma-pkg recipe list                List available recipes
  sigma-pkg recipe validate <file>     Validate a recipe file
  sigma-pkg recipe info <name>         Show recipe details

Inspired by: NixOS derivations + Arch PKGBUILD + Flatpak manifests

Examples:
  sigma-pkg recipe new my-tool
  sigma-pkg recipe build sigma_pkg_registry/recipes/sigma-edit.toml
  sigma-pkg recipe list
"""
    return

  let recipe_dir = "sigma_pkg_registry/recipes"
  case args[0].toLowerAscii
  of "new","scaffold","create":
    if args.len < 2: echo "Usage: sigma-pkg recipe new <name>"; return
    scaffold_recipe(args[1], recipe_dir)

  of "build":
    if args.len < 2: echo "Usage: sigma-pkg recipe build <file.toml>"; return
    let recipe = parse_recipe(args[1])
    if recipe.name.len == 0: echo "✗ Invalid recipe"; return
    let build_dir = "/tmp/sigma_build_" & $hash(recipe.name & recipe.version).abs
    let out_dir   = getEnv("HOME","/tmp") / ".cache/sigma/built"
    let ok = build_from_recipe(recipe, build_dir, out_dir)
    if ok: echo fmt"Install: sigma-pkg install {out_dir}/{recipe.name}-{recipe.version}.sigpkg"

  of "list":
    if dirExists(recipe_dir):
      for _, path in walkDir(recipe_dir):
        if path.endsWith(".toml"):
          let r = parse_recipe(path)
          if r.name.len > 0:
            echo fmt"  {r.name:<25} {r.version:<10}  {r.description[0..<min(40,r.description.len)]}"
    else: echo "(no recipes directory found)"

  of "validate":
    if args.len < 2: echo "Usage: sigma-pkg recipe validate <file>"; return
    let r = parse_recipe(args[1])
    if r.name.len == 0:   echo "✗ Missing: name"
    elif r.version.len == 0: echo "✗ Missing: version"
    elif r.source.url.len == 0 and r.source.git_url.len == 0: echo "⚠ No source URL"
    else: echo fmt"✓ Valid recipe: {r.name}-{r.version}"

  of "info":
    if args.len < 2: echo "Usage: sigma-pkg recipe info <name>"; return
    let path = recipe_dir / args[1] & ".toml"
    if not fileExists(path): echo fmt"Recipe not found: {args[1]}"; return
    let r = parse_recipe(path)
    echo fmt"Name:        {r.name}"
    echo fmt"Version:     {r.version}"
    echo fmt"Description: {r.description}"
    echo fmt"License:     {r.license}"
    echo fmt"Homepage:    {r.homepage}"
    echo fmt"Arch:        {r.arch.join(\", \")}"
    echo fmt"Runtime deps:{r.deps_runtime.join(\", \")}"
    echo fmt"Pledge:      {r.security.pledge.join(\", \")}"
  else:
    echo fmt"Unknown recipe command: {args[0]}"
