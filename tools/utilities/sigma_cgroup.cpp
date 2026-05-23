/*
 * Σ SigmaOS Zenith — cgroup Resource Manager Utility
 * Absorbs: Linux cgroups v2 / Kubernetes ResourceQuota
 * Zero-Dependency: No libc.
 */

extern "C" void cgroup_cli_run(int argc, char** argv);

extern "C" int sigma_cgroup_main(int argc, char** argv) {
    cgroup_cli_run(argc, argv);
    return 0;
}
