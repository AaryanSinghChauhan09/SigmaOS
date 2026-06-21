# SigmaOS Migration Guide

Welcome to the **SovereignMigrationAssistant** guide. Switching operating systems is traditionally a painful process of losing bookmarks, shell configurations, and IDE setups. SigmaOS completely eliminates this barrier.

## 🚀 How it Works

The Migration Assistant is a kernel-adjacent utility that performs four critical phases:
1. **Detection Layer:** Safely identifies foreign partitions (Windows NTFS or Ubuntu ext4) and mounts them strictly as Read-Only to prevent host corruption.
2. **Extraction Layer:** Pulls configurations, dotfiles, and personal documents from the foreign host.
3. **Translation Layer:** Automatically translates foreign paths (e.g., converting Windows `AppData` paths into Linux `~/.config` equivalents).
4. **Integration Layer:** This is the magic of SigmaOS. Instead of blindly pasting old configurations into your new home directory, the assistant wraps imported applications (like browsers and IDEs) into dedicated **SovereignSandbox** containers.

## 🛡️ The Sandbox Guarantee
Imported dotfiles from Ubuntu are notoriously messy and can break pristine environments. By sandboxing imported applications, SigmaOS guarantees that your new operating system remains fundamentally incorruptible, even if your old `.bashrc` or VS Code plugins were bloated or broken.

## 🛠️ Usage

From the SigmaOS terminal (post-installation), invoke the CLI:

### Migrate Everything
```bash
sigma_migration_cli /dev/nvme0n1p3 --all
```

### Selective Migration
If you only want your documents and browser bookmarks, you can explicitly select them:
```bash
sigma_migration_cli /dev/nvme0n1p3 --browsers --files
```

## Troubleshooting Sandboxed Apps
If an imported IDE fails to launch, it is likely because it lacks permissions in its new `SovereignSandbox`. You can grant it explicit access via the Sigma Control Center's capability matrix.
