/*
 * Σ SigmaOS Zenith — overlayfs Union File System Governor Utility
 * Absorbs: Linux OverlayFS / unionfs / Alpine Live USB
 * Zero-Dependency: No libc.
 */

extern "C" void overlay_cli_run(int argc, char** argv);

extern "C" int sigma_overlayfs_main(int argc, char** argv) {
    overlay_cli_run(argc, argv);
    return 0;
}
