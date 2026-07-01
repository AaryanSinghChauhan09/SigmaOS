# SPDX-License-Identifier: GPL-2.0-or-later
# ==========================================================================
# SigmaOS: Generation Manager (Nim)
# Replaces: config/core/GenerationManager.cpp
# ==========================================================================

type
  GenerationInfo* = object
    id*:        int
    timestamp*: uint64
    label*:     string
    active*:    bool

  GenerationManager* = object
    generations*: seq[GenerationInfo]
    current*:     int
    max_gens*:    int

proc newGenerationManager*(max: int = 8): GenerationManager =
  result.generations = newSeq[GenerationInfo]()
  result.current     = -1
  result.max_gens    = max

proc createGeneration*(mgr: var GenerationManager; label: string; ts: uint64): int =
  if mgr.generations.len >= mgr.max_gens:
    # Prune oldest
    mgr.generations.del(0)
  let id = mgr.generations.len
  mgr.generations.add(GenerationInfo(id: id, timestamp: ts, label: label, active: false))
  return id

proc activateGeneration*(mgr: var GenerationManager; id: int): bool =
  for i in 0 ..< mgr.generations.len:
    if mgr.generations[i].id == id:
      # Deactivate all
      for j in 0 ..< mgr.generations.len:
        mgr.generations[j].active = false
      mgr.generations[i].active = true
      mgr.current = id
      return true
  return false

proc rollback*(mgr: var GenerationManager): bool =
  ## Rolls back to the previous generation
  if mgr.generations.len < 2:
    return false
  let prev_id = mgr.generations[mgr.generations.len - 2].id
  return activateGeneration(mgr, prev_id)

proc currentGeneration*(mgr: GenerationManager): int =
  return mgr.current
