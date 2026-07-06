# Linux-Inspired Subsystems in SigmaOS

SigmaOS features a comprehensive suite of custom, `no_std`, zero-allocation subsystems designed to match the functional interface of standard Linux core systems:

## 1. udev (Sovereign Device Manager)
Implemented in [sigma_udev.rs](file:///C:/Users/Aaryan/.gemini/antigravity-ide/scratch/SigmaOS/kernel/core/sigma_udev.rs). It manages hotplug hardware events, matches rules, populates `/dev` nodes dynamically, and exposes device classes (e.g. block, char, net, input).

## 2. sysfs
Implemented in [sigma_sysfs.rs](file:///C:/Users/Aaryan/.gemini/antigravity-ide/scratch/SigmaOS/kernel/core/sigma_sysfs.rs). It exports active kernel object attributes to a virtual filesystem directory hierarchy under `/sys/`.

## 3. procfs
Implemented in [sigma_procfs.rs](file:///C:/Users/Aaryan/.gemini/antigravity-ide/scratch/SigmaOS/kernel/core/sigma_procfs.rs). It maps process resource statistics, maps, open file descriptors, and system load metrics to files under `/proc/`.

## 4. tmpfs
Implemented in [sigma_tmpfs.rs](file:///C:/Users/Aaryan/.gemini/antigravity-ide/scratch/SigmaOS/kernel/fs/sigma_tmpfs.rs). It implements a high-speed RAM-backed filesystem using the kernel page pool.

## 5. inotify
Implemented in [sigma_inotify.rs](file:///C:/Users/Aaryan/.gemini/antigravity-ide/scratch/SigmaOS/kernel/core/sigma_inotify.rs). It facilitates filesystem event monitoring for files and directories (create, modify, move, delete).

## 6. dmesg
Implemented in [sigma_dmesg.rs](file:///C:/Users/Aaryan/.gemini/antigravity-ide/scratch/SigmaOS/kernel/core/sigma_dmesg.rs). It manages the circular kernel ring buffer containing timestamped boot messages.

## 7. mount
Implemented in [sigma_mount.rs](file:///C:/Users/Aaryan/.gemini/antigravity-ide/scratch/SigmaOS/kernel/core/sigma_mount.rs). It maintains the global mount namespace and processes fstab records.

## 8. sysctl
Implemented in [sigma_sysctl.rs](file:///C:/Users/Aaryan/.gemini/antigravity-ide/scratch/SigmaOS/kernel/core/sigma_sysctl.rs). It handles live tuning of kernel parameters under the `kernel`, `vm`, `net`, and `fs` hierarchies.

## 9. logrotate
Implemented in [sigma_logrotate.rs](file:///C:/Users/Aaryan/.gemini/antigravity-ide/scratch/SigmaOS/kernel/core/sigma_logrotate.rs). It manages log prunings and rotation intervals.
