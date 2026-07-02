# Package
version     = "15.0.0"
author      = "SigmaOS Project"
description = "sigma-agent: AI CLI Agent for SigmaOS"
license     = "MIT"
srcDir      = "."
bin         = @["sigma_agent_main", "sigma_agent_session"]

# Dependencies — no third-party: pure Nim stdlib only
requires "nim >= 2.0.0"

# Build tasks
task build, "Build all sigma-agent binaries":
  exec "nim c -d:release --opt:speed -o:sigma-agent sigma_agent_main.nim"
  exec "nim c -d:release --opt:speed -o:sigma-agent-session sigma_agent_session.nim"
  echo "✓ Built: sigma-agent, sigma-agent-session"

task install, "Install to /usr/local/bin":
  exec "nim c -d:release -o:sigma-agent sigma_agent_main.nim"
  exec "cp sigma-agent /usr/local/bin/sigma-agent"
  echo "✓ Installed sigma-agent to /usr/local/bin"

task test, "Run agent self-tests":
  exec "./sigma-agent \"system info\" 2>/dev/null"
  exec "./sigma-agent \"list .\" 2>/dev/null"
  echo "✓ Basic tests passed"
