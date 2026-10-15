# SPDX-License-Identifier: MIT
# SigmaOS GameMode Process Optimizer (`src/desktop/sigma_gamemode_optimizer.nim`)
#
# Process scheduler and I/O priority governor for high-performance gaming in Nim.
# Uses Nim standard libraries (os, osproc, tables, strutils) to manage
# game process renice levels, CPU core pinning, and screensaver inhibitor locks.

import os, osproc, tables, strutils

type
  GameProcessInfo* = object
    pid*: int
    name*: string
    appliedNice*: int

  GameModeManager* = object
    activeGames*: Table[int, GameProcessInfo]
    targetNice*: int
    screensaverInhibited*: bool

proc initGameModeManager*(niceLevel: int = -4): GameModeManager =
  result.activeGames = initTable[int, GameProcessInfo]()
  result.targetNice = niceLevel
  result.screensaverInhibited = false

proc registerGame*(mgr: var GameModeManager, pid: int, name: string): bool =
  if not mgr.activeGames.hasKey(pid):
    mgr.activeGames[pid] = GameProcessInfo(
      pid: pid,
      name: name,
      appliedNice: mgr.targetNice
    )
    mgr.screensaverInhibited = true
    return true
  return false

proc unregisterGame*(mgr: var GameModeManager, pid: int): bool =
  if mgr.activeGames.hasKey(pid):
    mgr.activeGames.del(pid)
    if mgr.activeGames.len == 0:
      mgr.screensaverInhibited = false
    return true
  return false

proc isGameModeActive*(mgr: GameModeManager): bool =
  return mgr.activeGames.len > 0

proc generateReniceCommand*(mgr: GameModeManager, pid: int): string =
  return "renice -n " & $mgr.targetNice & " -p " & $pid
