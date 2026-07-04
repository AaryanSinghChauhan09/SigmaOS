# Package
version     = "15.1.0"
author      = "SigmaOS Project"
description = "sigma-agent: AI CLI Agent for SigmaOS — 35 modules"
license     = "MIT"
srcDir      = "."
bin         = @["sigma_agent_main", "sigma_agent_session"]

# Dependencies — no third-party: pure Nim stdlib only
requires "nim >= 2.0.0"

# ── Build tasks ───────────────────────────────────────────────────────────────
task build, "Build all sigma-agent binaries":
  exec "nim c -d:release --opt:speed --verbosity:0 -o:sigma-agent sigma_agent_main.nim"
  exec "nim c -d:release --opt:speed --verbosity:0 -o:sigma-agent-session sigma_agent_session.nim"
  echo "✓ Built: sigma-agent, sigma-agent-session"

task install, "Install sigma-agent to /usr/local/bin":
  exec "nim c -d:release --opt:speed -o:sigma-agent sigma_agent_main.nim"
  exec "cp sigma-agent /usr/local/bin/sigma-agent"
  exec "sigma-agent install --shell-integration 2>/dev/null || true"
  echo "✓ Installed sigma-agent to /usr/local/bin"
  echo "  Restart your shell or: source ~/.sigma_agent_rc"

task doctor, "Run sigma-agent doctor":
  exec "./sigma-agent doctor 2>/dev/null || true"

task test, "Run full benchmark + smoke tests":
  exec "./sigma-agent benchmark quick 2>/dev/null | tail -5"
  exec "./sigma-agent doctor 2>/dev/null | tail -3"
  echo "✓ Tests complete"

task seed, "Seed training dataset":
  exec "./sigma-agent train seed 2>/dev/null"
  exec "./sigma-agent train stats 2>/dev/null"

task workflow_templates, "Install all built-in workflow templates":
  exec "./sigma-agent workflow install --all 2>/dev/null"
  echo "✓ All workflow templates installed"

task clean, "Remove built binaries":
  exec "rm -f sigma-agent sigma-agent-session"
  echo "✓ Cleaned"
