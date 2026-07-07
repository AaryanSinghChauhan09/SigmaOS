# Observability Roadmap

## Lightweight Telemetry
SigmaOS avoids heavy userland daemons by embedding tracepoints directly into the core primitives (scheduler, buddy allocator, VFS). 

## Prometheus / Grafana Integration
A native `sigma-prom-agent` serves metrics over HTTP, allowing seamless integration into standard Grafana dashboards.

## Anomaly Detection & Auto-Rollback
- By continuously monitoring kernel telemetry post-update, the system can detect crash loops or excessive page faults.
- Upon detecting critical failure, the OS triggers the `sigpkg` Auto-Rollback protocol to restore the previous atomic snapshot automatically.
