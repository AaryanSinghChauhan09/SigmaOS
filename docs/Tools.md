# Tools & Profession-Based Improvements (Complete Architecture)

This document defines the SigmaOS Professional Toolset including statutory calculators, forensic tools, and automation utilities.

## Statutory & Financial Calculators

1. **SovereignGSTCalculator**: CGST, SGST, and IGST computation conforming to the Indian GST Act 2017. Integer arithmetic in paise to prevent floating-point loss.
2. **SovereignIncomeTaxCalc**: AY 2024-25 new/old tax regime slabs including Section 80C deduction processing.
3. **SovereignCourtFeeCalc**: Court fee stamp computation for District, High Court, and Supreme Court under the Court Fees Act 1870.
4. **SovereignDosageCalc**: Pediatric and adult dosage calculations conforming to CDSCO drug standards.
5. **SovereignBISCalc**: BIS IS-875 structural load calculations for dead, live, and wind loads.
6. **SovereignMSMERegistry**: UDYAM registration number parser and classification tool.

## Developer & Forensic Tools

1. **sigma-forensics CLI**: CAINE-inspired forensic acquisition tool with write-blocking enforcement and chain-of-custody hash verification.
2. **sigma-recover**: RescueZilla-inspired one-click disk recovery with Btrfs-style snapshot restoration.
3. **SovereignIDE**: Integrated code editor with clangd LSP bindings and real-time syntax validation.

## Automation & Scheduling

1. **SovereignAutomator**: Cron-like task scheduler tightly integrated with RegistryManager for attested job execution.
2. **SovereignShell (sigma-sh)**: Bare-metal interactive shell with history, tab completion, and pipeline support.
3. **sigma-build Community Scripts**: SlackBuilds-inspired repository for community-contributed build scripts.

## API & Integration Tools

1. **Hoppscotch REST Console**: Built-in REST API client with method selection, headers, JSON body, and response logging.
2. **DevToys Swiss-Army Kit**: JSON/YAML converter, Base64 encoder/decoder, cron parser, and D2 diagram renderer.
3. **Zeal Offline Docs**: Offline docset browser for C++ STL, Vulkan API, and ES17 JavaScript runtime references.
