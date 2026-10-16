# Service Manager Agent

## Role
`SigmaServiceMgr` provides the `sigmactrl` CLI and service management daemon for SigmaOS — enabling start/stop/restart/status operations on system services, analogous to `systemctl`.

## Status
🟡 In Progress — Basic in `src/system/sigmaservice.rs`; `sigmactrl` CLI and IPC channel to SigmaInit needed.

## Source File
`src/system/sigmaservice.rs`

## Linux/BSD Inspiration
| Concept | Linux (systemctl) | FreeBSD (service) | SigmaOS |
|---|---|---|---|
| Start/stop | `systemctl start X` | `service X start` | `sigmactrl start X` |
| Status | `systemctl status X` | `service X status` | `sigmactrl status X` |
| Enable/disable | `systemctl enable X` | `sysrc X_enable` | `sigmactrl enable X` |
| List units | `systemctl list-units` | `service -e` | `sigmactrl list` |
| Journal | `journalctl -u X` | `tail /var/log/` | `sigmactrl logs X` |
| Daemon reload | `systemctl daemon-reload` | — | `sigmactrl reload` |
| User units | `systemctl --user` | — | `sigmactrl --user` |

## Responsibilities
- IPC with `SigmaInit` over Unix domain socket
- `sigmactrl start|stop|restart|reload|status|enable|disable <unit>`
- Service dependency check before start
- Real-time log streaming (`sigmactrl logs -f X`)
- Enable/disable (symlink to `/etc/sigma/wants/`)
- List all units with state, PID, memory usage
- Snapshot and restore service state (checkpoint)
- D-Bus compatible interface for GUI service managers

## Implementation Notes
- `#![no_std]` — IPC via Unix socket with JSON/CBOR protocol
- `SigmaCtrlCmd { op: Op, unit: [u8;64] }` → send to `/run/sigmaos/init.sock`
- Response: `SigmaCtrlReply { status: Status, message: &str }`
- Enable = create symlink in `/etc/sigma/multi-user.target.wants/`

## Testing
```bash
rustc --test src/system/sigmaservice.rs --edition 2021 -o /tmp/test_svc && /tmp/test_svc --test-threads=1
```

## Future Development
1. **Predictive start** — Start services before explicit request based on usage
2. **Service mesh integration** — Register services with SigmaServiceMesh
3. **Resource usage display** — Show CPU/RAM per service in `sigmactrl status`
4. **Rollback support** — Revert to previous service configuration on failure
5. **Remote service management** — `sigmactrl --host remote.host start X`
6. **Signed unit files** — Verify service unit file signature before loading
