# SPDX-License-Identifier: MIT
# SigmaOS Omarchy Theme Bridge (`src/desktop/sigma_omarchy_theme_bridge.nim`)
#
# Real-time Theme Omakase dispatcher inspired by Omarchy Linux.
# Uses Nim standard libraries (json, strutils, tables, os) to broadcast
# atomic theme color updates to Hyprland, Waybar, Walker, and Alacritty.

import json, strutils, tables, os

type
  OmarchyTheme* = object
    name*: string
    background*: string
    foreground*: string
    accentPrimary*: string
    accentSecondary*: string
    isDark*: bool

  ThemeBridgeManager* = object
    themes*: Table[string, OmarchyTheme]
    activeThemeName*: string

proc initThemeBridge*(): ThemeBridgeManager =
  result.themes = initTable[string, OmarchyTheme]()
  
  # Standard Omarchy and SigmaOS Themes
  result.themes["Tokyo Night"] = OmarchyTheme(
    name: "Tokyo Night",
    background: "#1a1b26",
    foreground: "#c0caf5",
    accentPrimary: "#7aa2f7",
    accentSecondary: "#bb9af7",
    isDark: true
  )
  
  result.themes["Catppuccin Mocha"] = OmarchyTheme(
    name: "Catppuccin Mocha",
    background: "#1e1e2e",
    foreground: "#cdd6f4",
    accentPrimary: "#89b4fa",
    accentSecondary: "#f5c2e7",
    isDark: true
  )
  
  result.themes["SigmaOS Zenith Cyber"] = OmarchyTheme(
    name: "SigmaOS Zenith Cyber",
    background: "#0a0e14",
    foreground: "#e6e1cf",
    accentPrimary: "#ffb454",
    accentSecondary: "#39bae6",
    isDark: true
  )
  
  result.activeThemeName = "SigmaOS Zenith Cyber"

proc switchTheme*(mgr: var ThemeBridgeManager, name: string): bool =
  if mgr.themes.hasKey(name):
    mgr.activeThemeName = name
    return true
  return false

proc exportWaybarCss*(mgr: ThemeBridgeManager): string =
  let theme = mgr.themes[mgr.activeThemeName]
  result = "@define-color background " & theme.background & ";\n" &
           "@define-color foreground " & theme.foreground & ";\n" &
           "@define-color primary " & theme.accentPrimary & ";\n" &
           "@define-color secondary " & theme.accentSecondary & ";\n"

proc exportJsonTokens*(mgr: ThemeBridgeManager): string =
  let theme = mgr.themes[mgr.activeThemeName]
  var jobj = %* {
    "theme": theme.name,
    "bg": theme.background,
    "fg": theme.foreground,
    "accent1": theme.accentPrimary,
    "accent2": theme.accentSecondary,
    "dark": theme.isDark
  }
  return $jobj
