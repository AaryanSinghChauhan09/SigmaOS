# Hacking SigmaOS Zenith

SigmaOS is an HTML/JS-based sovereign simulation.

## Shard Orchestration

New windows can be added to index.html by defining a <div class=\"window\"> and adding the corresponding logic to zenith_desktop.js.

## System Simulation

Kernel telemetry is simulated via ddLog() and updateClock() calls. For real-time bus simulation, see the PersonalizationEngine.
