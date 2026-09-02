# Linux Distributions Architecture & Parity Guide

SigmaOS incorporates architectural paradigms from leading Linux distributions:

*   **Arch Linux**: Rolling release dependency resolution (`ArchDependencyResolver`) and PKGBUILD recipe sandbox compilation (`ArchRecipeSandboxCompiler`).
*   **NixOS / Guix**: Declarative system generations (`NixDeclarativeSystemState`), content-addressed store (`NixStyleStore`), and GNU Shepherd service graph manager (`ShepherdServiceManager`).
*   **Clear Linux**: Stateless `/usr` configuration overlay architecture (`ClearLinuxStatelessOverlayEngine`).
*   **Gentoo**: Portage USE-flags compilation and dependency resolution (`GentooPortageUseFlagResolver`).
*   **Alpine / Void Linux**: Transactional trigger hooks (`AlpineVoidTriggerHookManager`) and Runit 3-stage service lifecycle supervision (`SovereignRunitSupervisor`).
*   **Fedora / Ubuntu**: Fedora Toolbox dev containers and Ubuntu Pro Livepatch hot-patching.
