# SPDX-License-Identifier: GPL-3.0-or-later
# SigmaOS Sovereign Fast Installer TUI
# (`src/installer/sigma_installer_tui.nim`)
# Nim module providing high-speed graphical and console installer workflow,
# hardware profile selection, and the "I'm migrating from Mint/Omarchy" fast path.

import strutils, tables

type
  InstallFlowMode* = enum
    ifmCleanZenith, ifmMigrateMint, ifmMigrateOmarchy, ifmDualBoot

  DriverPackageTier* = enum
    tierNvidiaProprietary, tierAmdRadvMesa, tierIntelIrisMesa, tierGenericVesa

  InstallerSessionState* = object
    targetDisk*: string
    flowMode*: InstallFlowMode
    driverTier*: DriverPackageTier
    progressPercent*: int
    statusMessage*: string

proc initInstallerSession*(disk: string, mode: InstallFlowMode): InstallerSessionState =
  result.targetDisk = disk
  result.flowMode = mode
  result.driverTier = tierAmdRadvMesa
  result.progressPercent = 0
  result.statusMessage = "Ready to install SigmaOS on " & disk

proc selectMigrationMode*(session: var InstallerSessionState, mode: InstallFlowMode) =
  session.flowMode = mode
  case mode
  of ifmMigrateMint:
    session.statusMessage = "Configured for zero-friction Linux Mint migration."
  of ifmMigrateOmarchy:
    session.statusMessage = "Configured for zero-friction Omarchy Hyprland migration."
  of ifmCleanZenith:
    session.statusMessage = "Configured for pure fresh SigmaOS Zenith installation."
  of ifmDualBoot:
    session.statusMessage = "Configured for dual-boot installation alongside existing OS."

proc updateProgress*(session: var InstallerSessionState, pct: int, msg: string) =
  session.progressPercent = pct
  session.statusMessage = msg

proc renderInstallerHeader*(session: InstallerSessionState): string =
  result = "========================================================\n"
  result.add("  SigmaOS Sovereign Fast Installer (Migration-First)   \n")
  result.add("========================================================\n")
  result.add("Target: " & session.targetDisk & " | Mode: " & $session.flowMode & "\n")
  result.add("Status: [" & $session.progressPercent & "%] " & session.statusMessage & "\n")
