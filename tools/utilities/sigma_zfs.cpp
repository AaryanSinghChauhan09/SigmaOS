/*
 * Σ SigmaOS Zenith — zfs Storage Pool Governor Utility
 * Absorbs: OpenZFS / FreeBSD zpool / ZFS filesystems
 * Zero-Dependency: No libc.
 */

extern "C" void zfs_cli_run(int argc, char** argv);

extern "C" int sigma_zfs_main(int argc, char** argv) {
    zfs_cli_run(argc, argv);
    return 0;
}
