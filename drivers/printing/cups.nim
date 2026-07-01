# SPDX-License-Identifier: GPL-2.0-or-later
# ==========================================================================
# SigmaOS: Sovereign Printing Backend (Nim)
# Replaces: drivers/printing/SovereignCUPS.cpp
# ==========================================================================

type
  PrintJob* = object
    jobId*:      int
    document*:   string
    pages*:      int
    completed*:  bool

  SovereignCUPS* = object
    jobs*: seq[PrintJob]
    active*: bool

proc newSovereignCUPS*(): SovereignCUPS =
  result.jobs = newSeq[PrintJob]()
  result.active = true

proc submitJob*(cups: var SovereignCUPS; document: string; pages: int): int =
  if not cups.active:
    return -1
  let id = cups.jobs.len + 1
  cups.jobs.add(PrintJob(jobId: id, document: document, pages: pages, completed: false))
  return id

proc cancelJob*(cups: var SovereignCUPS; jobId: int): bool =
  for i in 0 ..< cups.jobs.len:
    if cups.jobs[i].jobId == jobId:
      cups.jobs.del(i)
      return true
  return false

proc processJobs*(cups: var SovereignCUPS) =
  for i in 0 ..< cups.jobs.len:
    cups.jobs[i].completed = true
