# SPDX-License-Identifier: GPL-2.0-or-later
# ==========================================================================
# SigmaOS: Sovereign Aether Orchestrator & Absorption (Nim)
# Replaces: ecosystem/SovereignAetherAbsorption.cpp, SovereignAetherOrchestrator.cpp
# ==========================================================================

type
  AetherNode* = object
    nodeId*:     int
    connected*:  bool
    throughput*: float

  AetherOrchestrator* = object
    nodes*:     seq[AetherNode]
    active*:    bool

proc newAetherOrchestrator*(): AetherOrchestrator =
  result.nodes = newSeq[AetherNode]()
  result.active = true

proc registerNode*(orch: var AetherOrchestrator; id: int): bool =
  if not orch.active:
    return false
  orch.nodes.add(AetherNode(nodeId: id, connected: true, throughput: 0.0))
  return true

proc performAbsorption*(orch: var AetherOrchestrator; nodeId: int; dataLen: int): bool =
  for i in 0 ..< orch.nodes.len:
    if orch.nodes[i].nodeId == nodeId:
      orch.nodes[i].throughput += float(dataLen) / 1024.0
      return true
  return false
