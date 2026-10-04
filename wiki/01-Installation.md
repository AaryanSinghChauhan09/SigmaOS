# Installation Status

**Status: not ready for general installation.** SigmaOS does not currently publish a verified install image and complete installation/recovery procedure. The repository contains installer and ISO-building code, but source presence is not evidence that an image boots, installs safely, or can recover after interruption.

## Do not install to a physical disk

Do not use the current development ISO builder as installation media or write its output to a disk. The documented boot-to-user-session path and installer recovery behavior have not been validated end to end.

## What must be validated before an install guide is published

1. Build the bare-metal kernel and image reproducibly from a clean checkout.
2. Boot the image in QEMU and on each claimed hardware target; capture serial logs and identify a runtime-ready marker.
3. Validate partition selection, writes, interruption behavior, reboot, and recovery using disposable virtual disks.
4. Verify that the installed system reaches a usable shell or desktop and that documented input/network/storage devices work.
5. Publish image provenance/checksums, minimum requirements, supported hardware, known limitations, and a tested rollback/recovery procedure.

## Developer setup

To build and test the source, use [Getting Started](02-Getting-Started.md). To contribute installer work, read the [Installer component notes](Installer.md), add failure-path tests, and update this page only when the end-to-end checks above pass.
