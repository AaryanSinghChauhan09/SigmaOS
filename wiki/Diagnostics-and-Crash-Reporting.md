# System Information and Diagnostics

**Capability state: Partial.** SigmaOS has a hosted, read-only system information collector inspired by Linux Mint's System Information and System Reports tools. It can format a report from available host facts. It is not a SigmaOS desktop app, kernel telemetry service, crash collector, or support bundle, and it does not imply SigmaOS is bootable.

## Current capability

`src/tools/mint_system_report.rs` provides `MintSystemReport` (also exported as `SystemInformationReport`) and the `tools::mint_system_report` module. It collects the host architecture, Linux distribution release fields, kernel release, CPU model and logical processor count, and total/available memory when the corresponding Linux sources are readable. Missing sources appear as collection warnings; the collector does not fill gaps with sample values.

Report formats are plain text, JSON, and Markdown. JSON strings and Markdown table cells are escaped. Collection is local and read-only. By default, it omits hostname, usernames, network addresses, serial numbers, logs, and file contents. The report labels its scope as hosted Linux inspection, not SigmaOS kernel telemetry.

This is narrower than Linux Mint's integrated System Information/System Reports experience: there is no graphical viewer, device/driver inventory, crash report capture, or bug-report upload flow. No telemetry is sent.

## Design references

- **Linux Mint:** present system facts and troubleshooting information in a discoverable, understandable report.
- **SigmaOS:** gather only observed values, show missing sources, keep reports local, and make sensitive-data collection explicit.

References are design guidance, not evidence of feature parity.

## Validation

Run the module tests:

```sh
rustc --edition=2021 --test src/tools/mint_system_report.rs -o /tmp/sigmaos-system-report-tests
/tmp/sigmaos-system-report-tests
```

The tests cover report formatting, parsers, search, duplicate updates, and escaping. They do not establish a graphical session, SigmaOS kernel integration, or physical-device support.

## Roadmap

1. Add an accessible graphical System Information view backed by a documented provider interface.
2. Add device/driver details only from enumerated runtime sources, with unavailable and untested states visible.
3. Add local crash-report discovery only after SigmaOS has a real crash store; require preview and explicit export before sharing.
4. Exercise collectors on supported boot targets and test absent, malformed, and permission-denied sources.

**Completion evidence:** a user can open the report in a supported SigmaOS session, inspect source and collection status, export a privacy-reviewed report locally, and reproduce the facts on named tested hardware.

## Related pages

- [Desktop and UX](08-Desktop.md)
- [Security](07-Security.md)
- [Testing](Testing.md)
