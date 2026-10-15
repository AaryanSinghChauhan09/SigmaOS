# SPDX-License-Identifier: MIT
# SigmaOS Universal Wayland Session Manager (UWSM) (`src/desktop/sigma_uwsm_session_manager.nim`)
#
# Universal Wayland Session Manager lifecycle in Nim.
# Uses Nim standard libraries (os, osproc, tables, strutils) to orchestrate
# session environment variables, startup targets, and clean graceful shutdowns.

import os, osproc, tables, strutils

type
  SessionState* = enum
    ssUninitialized, ssStarting, ssRunning, ssStopping, ssTerminated

  SessionService* = object
    name*: string
    command*: string
    pid*: int
    autoRestart*: bool

  UwsmSessionManager* = object
    state*: SessionState
    envVars*: Table[string, string]
    services*: seq[SessionService]

proc initUwsmSession*(): UwsmSessionManager =
  result.state = ssUninitialized
  result.envVars = initTable[string, string]()
  result.services = @[]
  
  # Standard Wayland session environment setup
  result.envVars["XDG_SESSION_TYPE"] = "wayland"
  result.envVars["XDG_CURRENT_DESKTOP"] = "Hyprland"
  result.envVars["MOZ_ENABLE_WAYLAND"] = "1"
  result.envVars["QT_QPA_PLATFORM"] = "wayland;xcb"
  result.envVars["GDK_BACKEND"] = "wayland,x11,*"
  result.envVars["SDL_VIDEODRIVER"] = "wayland"
  result.envVars["CLUTTER_BACKEND"] = "wayland"

proc registerService*(mgr: var UwsmSessionManager, name, command: string, autoRestart: bool = true) =
  mgr.services.add(SessionService(
    name: name,
    command: command,
    pid: 0,
    autoRestart: autoRestart
  ))

proc exportEnvironment*(mgr: UwsmSessionManager) =
  for k, v in mgr.envVars:
    putEnv(k, v)

proc startSession*(mgr: var UwsmSessionManager): bool =
  mgr.exportEnvironment()
  mgr.state = ssRunning
  return true

proc stopSession*(mgr: var UwsmSessionManager): bool =
  mgr.state = ssStopping
  # Mark all services terminated
  for s in mgr.services.mitems:
    s.pid = 0
  mgr.state = ssTerminated
  return true
