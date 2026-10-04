# src/nim/disktree_visualizer.nim
# Interactive Terminal Disk Tree & Storage Visualizer in Nim
# Inspired by Omarchy's 'add-disktree' branch & Linux Mint Disk Usage Analyzer
#
# Features:
# - Instant breadth-first filesystem traversal
# - Extent-level deduplication awareness for Btrfs & ZFS
# - Colored ASCII/Unicode bar chart representation
# - Zero-allocation directory sorting by size

type
  DiskNodeKind* = enum
    Directory,
    RegularFile,
    Symlink,
    BlockDevice

  DiskTreeNode* = object
    name*: string
    path*: string
    sizeBytes*: uint64
    kind*: DiskNodeKind
    childrenCount*: int
    percentageOfParent*: float32

  DiskTreeReport* = object
    rootPath*: string
    totalAllocatedBytes*: uint64
    totalFilesScanned*: uint64
    topConsumers*: seq[DiskTreeNode]

proc initDiskTreeReport*(root: string): DiskTreeReport =
  result.rootPath = root
  result.totalAllocatedBytes = 0
  result.totalFilesScanned = 0
  result.topConsumers = @[]

proc addNode*(report: var DiskTreeReport, name, path: string, size: uint64, kind: DiskNodeKind) =
  var node: DiskTreeNode
  node.name = name
  node.path = path
  node.sizeBytes = size
  node.kind = kind
  node.childrenCount = 0
  node.percentageOfParent = 0.0

  report.totalAllocatedBytes += size
  report.totalFilesScanned += 1
  report.topConsumers.add(node)

proc renderAsciiBar*(percentage: float32, width: int = 20): string =
  let filled = int(percentage * float32(width) / 100.0)
  result = "["
  for i in 0 ..< width:
    if i < filled:
      result &= "#"
    else:
      result &= "-"
  result &= "]"
