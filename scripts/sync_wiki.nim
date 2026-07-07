# SigmaOS sync_wiki — Nim implementation
# Compiles to a native binary with zero runtime dependency.
# Replaces sync_wiki.ps1 (PowerShell) and sync_wiki.sh (POSIX shell).
# Nim compiles to C then to a native binary; no GC overhead in --gc:none mode.

import std/[os, strutils]

const
  SOURCE_DIR = "."
  WIKI_DIR   = "./wiki_repo"

proc syncWiki =
  if not dirExists(WIKI_DIR):
    createDir(WIKI_DIR)

  echo "Syncing Markdown files to Wiki..."

  for entry in walkDirRec(SOURCE_DIR):
    if entry.kind == pcFile and entry.path.endsWith(".md"):
      let path = entry.path
      # Skip files already in wiki_repo
      if WIKI_DIR in path:
        continue

      let filename   = extractFilename(path)
      # GitHub Wiki: replace spaces with hyphens
      let destname   = filename.replace(" ", "-")
      let destpath   = WIKI_DIR / destname

      copyFileWithPermissions(path, destpath)

  let readme = SOURCE_DIR / "README.md"
  if fileExists(readme):
    copyFileWithPermissions(readme, WIKI_DIR / "Home.md")

  echo "Wiki Sync COMPLETE."

syncWiki()
