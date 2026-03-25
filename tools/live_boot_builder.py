#!/usr/bin/env python3
"""
SigmaOS Live Boot Builder
========================
Advanced live boot and portable OS builder
"""

import os
import sys
import shutil
import subprocess
import tempfile
import json
import hashlib
import time
from pathlib import Path
from typing import Dict, List, Optional, Tuple
from dataclasses import dataclass
from enum import Enum

class BootMode(Enum):
    BIOS_LEGACY = "bios_legacy"
    UEFI = "uefi"
    HYBRID = "hybrid"

class CompressionType(Enum):
    NONE = "none"
    GZIP = "gzip"
    XZ = "xz"
    ZSTD = "zstd"

class FileSystemType(Enum):
    FAT32 = "fat32"
    EXT4 = "ext4"
    ISO9660 = "iso9660"
    SQUASHFS = "squashfs"

@dataclass
class LiveBootConfig:
    name: str
    version: str
    kernel_path: str
    initrd_path: str
    rootfs_path: str
    boot_mode: BootMode
    compression: CompressionType
    filesystem: FileSystemType
    output_path: str
    size_mb: int
    persistent: bool = False
    encryption: bool = False
    uefi_secure_boot: bool = False
    include_source: bool = False
    custom_modules: List[str] = None

class LiveBootBuilder:
    """
    Advanced live boot builder with multiple boot modes and filesystems
    """
    
    def __init__(self, config: LiveBootConfig):
        self.config = config
        self.temp_dir = None
        self.work_dir = None
        self.build_log = []
        
    def log(self, message: str) -> None:
        """Log build messages"""
        timestamp = time.strftime("%Y-%m-%d %H:%M:%S")
        log_entry = f"[{timestamp}] {message}"
        self.build_log.append(log_entry)
        print(log_entry)
        
    def setup_environment(self) -> bool:
        """Setup build environment"""
        self.log("Setting up build environment")
        
        try:
            # Create temporary working directory
            self.temp_dir = tempfile.mkdtemp(prefix="sigmaos_live_")
            self.work_dir = Path(self.temp_dir)
            
            # Create directory structure
            (self.work_dir / "boot").mkdir(exist_ok=True)
            (self.work_dir / "boot" / "grub").mkdir(exist_ok=True)
            (self.work_dir / "boot" / "efi").mkdir(exist_ok=True)
            (self.work_dir / "iso").mkdir(exist_ok=True)
            (self.work_dir / "rootfs").mkdir(exist_ok=True)
            
            self.log(f"Working directory: {self.work_dir}")
            return True
            
        except Exception as e:
            self.log(f"Failed to setup environment: {e}")
            return False
    
    def prepare_kernel(self) -> bool:
        """Prepare kernel for live boot"""
        self.log("Preparing kernel")
        
        try:
            kernel_src = Path(self.config.kernel_path)
            kernel_dst = self.work_dir / "boot" / "vmlinuz"
            
            if not kernel_src.exists():
                self.log(f"Kernel not found: {kernel_src}")
                return False
            
            # Copy kernel
            shutil.copy2(kernel_src, kernel_dst)
            
            # Generate kernel hash
            kernel_hash = self._calculate_file_hash(kernel_dst)
            self.log(f"Kernel hash: {kernel_hash}")
            
            return True
            
        except Exception as e:
            self.log(f"Failed to prepare kernel: {e}")
            return False
    
    def prepare_initrd(self) -> bool:
        """Prepare initrd for live boot"""
        self.log("Preparing initrd")
        
        try:
            initrd_src = Path(self.config.initrd_path)
            initrd_dst = self.work_dir / "boot" / "initrd"
            
            if not initrd_src.exists():
                self.log(f"Initrd not found: {initrd_src}")
                return False
            
            # Copy initrd
            shutil.copy2(initrd_src, initrd_dst)
            
            # Generate initrd hash
            initrd_hash = self._calculate_file_hash(initrd_dst)
            self.log(f"Initrd hash: {initrd_hash}")
            
            return True
            
        except Exception as e:
            self.log(f"Failed to prepare initrd: {e}")
            return False
    
    def create_rootfs(self) -> bool:
        """Create root filesystem"""
        self.log("Creating root filesystem")
        
        try:
            rootfs_src = Path(self.config.rootfs_path)
            rootfs_dst = self.work_dir / "rootfs"
            
            if not rootfs_src.exists():
                self.log(f"Rootfs not found: {rootfs_src}")
                return False
            
            # Copy rootfs
            if rootfs_src.is_dir():
                shutil.copytree(rootfs_src, rootfs_dst, dirs_exist_ok=True)
            else:
                # Extract rootfs if it's an archive
                self._extract_archive(rootfs_src, rootfs_dst)
            
            # Create live boot specific directories
            (rootfs_dst / "live").mkdir(exist_ok=True)
            (rootfs_dst / "union").mkdir(exist_ok=True)
            (rootfs_dst / "overlay").mkdir(exist_ok=True)
            
            # Create live boot configuration
            self._create_live_config(rootfs_dst)
            
            return True
            
        except Exception as e:
            self.log(f"Failed to create rootfs: {e}")
            return False
    
    def create_squashfs(self) -> bool:
        """Create squashfs filesystem"""
        self.log("Creating squashfs filesystem")
        
        try:
            squashfs_path = self.work_dir / "iso" / "live" / "filesystem.squashfs"
            (squashfs_path.parent).mkdir(exist_ok=True)
            
            # Create squashfs
            cmd = [
                "mksquashfs",
                str(self.work_dir / "rootfs"),
                str(squashfs_path),
                "-comp", "xz",
                "-Xdict-size", "100%",
                "-b", "1M",
                "-e", "boot"
            ]
            
            result = subprocess.run(cmd, capture_output=True, text=True)
            if result.returncode != 0:
                self.log(f"Failed to create squashfs: {result.stderr}")
                return False
            
            self.log(f"Squashfs created: {squashfs_path}")
            return True
            
        except Exception as e:
            self.log(f"Failed to create squashfs: {e}")
            return False
    
    def create_grub_config(self) -> bool:
        """Create GRUB configuration"""
        self.log("Creating GRUB configuration")
        
        try:
            grub_cfg = self.work_dir / "boot" / "grub" / "grub.cfg"
            
            config_content = self._generate_grub_config()
            
            with open(grub_cfg, 'w') as f:
                f.write(config_content)
            
            self.log("GRUB configuration created")
            return True
            
        except Exception as e:
            self.log(f"Failed to create GRUB config: {e}")
            return False
    
    def create_bios_boot_image(self) -> bool:
        """Create BIOS boot image"""
        self.log("Creating BIOS boot image")
        
        try:
            # Create bootable image
            boot_img = self.work_dir / "boot.img"
            
            # Create FAT32 filesystem for boot
            cmd = [
                "mkfs.vfat",
                "-F32",
                "-n", "SIGMAOS_BOOT",
                str(boot_img)
            ]
            
            result = subprocess.run(cmd, capture_output=True, text=True)
            if result.returncode != 0:
                self.log(f"Failed to create boot image: {result.stderr}")
                return False
            
            # Mount and copy boot files
            mount_point = self.work_dir / "boot_mount"
            mount_point.mkdir(exist_ok=True)
            
            subprocess.run(["mount", str(boot_img), str(mount_point)], check=True)
            
            # Copy GRUB files
            shutil.copytree(self.work_dir / "boot" / "grub", 
                         mount_point / "grub", dirs_exist_ok=True)
            
            # Copy kernel and initrd
            shutil.copy2(self.work_dir / "boot" / "vmlinuz", 
                        mount_point / "vmlinuz")
            shutil.copy2(self.work_dir / "boot" / "initrd", 
                        mount_point / "initrd")
            
            # Install GRUB
            cmd = [
                "grub-install",
                "--target=i386-pc",
                "--boot-directory=str(mount_point)",
                "--removable",
                str(boot_img)
            ]
            
            result = subprocess.run(cmd, capture_output=True, text=True)
            if result.returncode != 0:
                self.log(f"Failed to install GRUB: {result.stderr}")
                return False
            
            # Unmount
            subprocess.run(["umount", str(mount_point)], check=True)
            
            self.log("BIOS boot image created")
            return True
            
        except Exception as e:
            self.log(f"Failed to create BIOS boot image: {e}")
            return False
    
    def create_uefi_boot_image(self) -> bool:
        """Create UEFI boot image"""
        self.log("Creating UEFI boot image")
        
        try:
            # Create EFI System Partition
            efi_img = self.work_dir / "efi.img"
            
            # Create FAT32 filesystem
            cmd = [
                "mkfs.vfat",
                "-F32",
                "-n", "SIGMAOS_EFI",
                str(efi_img)
            ]
            
            result = subprocess.run(cmd, capture_output=True, text=True)
            if result.returncode != 0:
                self.log(f"Failed to create EFI image: {result.stderr}")
                return False
            
            # Mount and copy EFI files
            mount_point = self.work_dir / "efi_mount"
            mount_point.mkdir(exist_ok=True)
            
            subprocess.run(["mount", str(efi_img), str(mount_point)], check=True)
            
            # Create EFI directory structure
            (mount_point / "EFI" / "SIGMAOS").mkdir(parents=True, exist_ok=True)
            
            # Copy GRUB EFI files
            cmd = [
                "grub-install",
                "--target=x86_64-efi",
                "--boot-directory=" + str(mount_point / Path("EFI") / "SIGMAOS"),
                "--efi-directory=" + str(mount_point),
                "--removable",
                "--no-nvram",
                str(efi_img)
            ]
            
            result = subprocess.run(cmd, capture_output=True, text=True)
            if result.returncode != 0:
                self.log(f"Failed to install UEFI GRUB: {result.stderr}")
                return False
            
            # Unmount
            subprocess.run(["umount", str(mount_point)], check=True)
            
            self.log("UEFI boot image created")
            return True
            
        except Exception as e:
            self.log(f"Failed to create UEFI boot image: {e}")
            return False
    
    def create_iso_image(self) -> bool:
        """Create ISO image"""
        self.log("Creating ISO image")
        
        try:
            iso_path = Path(self.config.output_path)
            
            # Prepare ISO directory
            iso_dir = self.work_dir / "iso"
            (iso_dir / "boot" / "grub").mkdir(parents=True, exist_ok=True)
            (iso_dir / "EFI" / "BOOT").mkdir(parents=True, exist_ok=True)
            
            # Copy files for ISO
            if self.config.boot_mode in [BootMode.BIOS_LEGACY, BootMode.HYBRID]:
                shutil.copytree(self.work_dir / "boot" / "grub", 
                             iso_dir / "boot" / "grub", dirs_exist_ok=True)
            
            if self.config.boot_mode in [BootMode.UEFI, BootMode.HYBRID]:
                shutil.copytree(self.work_dir / "boot" / "efi", 
                             iso_dir / "EFI", dirs_exist_ok=True)
            
            # Create ISO using xorriso
            cmd = [
                "xorriso",
                "-as", "mkisofs",
                "-iso-level", "3",
                "-full-iso9660-filenames",
                "-joliet-long",
                "-volid", f"SIGMAOS_{self.config.version}",
                "-appid", "SIGMAOS",
                "-publisher", "SigmaOS Project",
                "-preparer", "SigmaOS Live Boot Builder",
                "-eltorito-boot", "boot/grub/i386-pc/eltorito.img",
                "-no-emul-boot",
                "-boot-load-size", "4",
                "-boot-info-table",
                "-eltorito-alt-boot",
                "-e", "EFI/BOOT/BOOTX64.EFI",
                "-no-emul-boot",
                "-isohybrid-gpt-basdat",
                "-output", str(iso_path),
                str(iso_dir)
            ]
            
            if self.config.boot_mode == BootMode.UEFI:
                # UEFI-only ISO
                cmd = [
                    "xorriso",
                    "-as", "mkisofs",
                    "-iso-level", "3",
                    "-full-iso9660-filenames",
                    "-joliet-long",
                    "-volid", f"SIGMAOS_{self.config.version}",
                    "-appid", "SIGMAOS",
                    "-publisher", "SigmaOS Project",
                    "-preparer", "SigmaOS Live Boot Builder",
                    "-e", "EFI/BOOT/BOOTX64.EFI",
                    "-no-emul-boot",
                    "-output", str(iso_path),
                    str(iso_dir)
                ]
            
            result = subprocess.run(cmd, capture_output=True, text=True)
            if result.returncode != 0:
                self.log(f"Failed to create ISO: {result.stderr}")
                return False
            
            self.log(f"ISO image created: {iso_path}")
            return True
            
        except Exception as e:
            self.log(f"Failed to create ISO image: {e}")
            return False
    
    def create_usb_image(self) -> bool:
        """Create USB bootable image"""
        self.log("Creating USB bootable image")
        
        try:
            usb_path = Path(self.config.output_path).with_suffix('.img')
            
            # Create disk image
            cmd = [
                "dd",
                "if=/dev/zero",
                f"of={usb_path}",
                f"bs=1M",
                f"count={self.config.size_mb}"
            ]
            
            result = subprocess.run(cmd, capture_output=True, text=True)
            if result.returncode != 0:
                self.log(f"Failed to create disk image: {result.stderr}")
                return False
            
            # Create partitions
            cmd = [
                "parted",
                str(usb_path),
                "mklabel", "gpt",
                "mkpart", "EFI", "fat32", "1MiB", "100MiB",
                "mkpart", "SIGMAOS", "ext4", "100MiB", "100%",
                "set", "1", "boot", "on"
            ]
            
            result = subprocess.run(cmd, capture_output=True, text=True)
            if result.returncode != 0:
                self.log(f"Failed to create partitions: {result.stderr}")
                return False
            
            # Setup loop device
            cmd = ["losetup", "--find", "--show", str(usb_path)]
            result = subprocess.run(cmd, capture_output=True, text=True)
            if result.returncode != 0:
                self.log(f"Failed to setup loop device: {result.stderr}")
                return False
            
            loop_device = result.stdout.strip()
            
            try:
                # Create filesystems
                subprocess.run(["mkfs.vfat", "-F32", f"{loop_device}p1"], check=True)
                subprocess.run(["mkfs.ext4", "-F", f"{loop_device}p2"], check=True)
                
                # Mount and copy files
                efi_mount = self.work_dir / "efi_mount"
                root_mount = self.work_dir / "root_mount"
                
                efi_mount.mkdir(exist_ok=True)
                root_mount.mkdir(exist_ok=True)
                
                subprocess.run(["mount", f"{loop_device}p1", str(efi_mount)], check=True)
                subprocess.run(["mount", f"{loop_device}p2", str(root_mount)], check=True)
                
                # Copy EFI files
                shutil.copytree(self.work_dir / "boot" / "efi", 
                             efi_mount, dirs_exist_ok=True)
                
                # Copy system files
                shutil.copytree(self.work_dir / "iso", 
                             root_mount, dirs_exist_ok=True)
                
                # Unmount
                subprocess.run(["umount", str(efi_mount)], check=True)
                subprocess.run(["umount", str(root_mount)], check=True)
                
            finally:
                # Cleanup loop device
                subprocess.run(["losetup", "-d", loop_device], check=False)
            
            self.log(f"USB image created: {usb_path}")
            return True
            
        except Exception as e:
            self.log(f"Failed to create USB image: {e}")
            return False
    
    def create_persistent_storage(self) -> bool:
        """Create persistent storage configuration"""
        if not self.config.persistent:
            return True
        
        self.log("Creating persistent storage configuration")
        
        try:
            # Create persistent overlay
            overlay_dir = self.work_dir / "rootfs" / "overlay"
            overlay_dir.mkdir(exist_ok=True)
            
            # Create upper and work directories
            (overlay_dir / "upper").mkdir(exist_ok=True)
            (overlay_dir / "work").mkdir(exist_ok=True)
            
            # Create persistent configuration
            persistent_config = {
                "type": "overlay",
                "upper_dir": "/overlay/upper",
                "work_dir": "/overlay/work",
                "lower_dir": "/live/filesystem.squashfs"
            }
            
            config_path = overlay_dir / "persistent.json"
            with open(config_path, 'w') as f:
                json.dump(persistent_config, f, indent=2)
            
            self.log("Persistent storage configuration created")
            return True
            
        except Exception as e:
            self.log(f"Failed to create persistent storage: {e}")
            return False
    
    def _calculate_file_hash(self, file_path: Path) -> str:
        """Calculate SHA256 hash of file"""
        hash_sha256 = hashlib.sha256()
        with open(file_path, "rb") as f:
            for chunk in iter(lambda: f.read(4096), b""):
                hash_sha256.update(chunk)
        return hash_sha256.hexdigest()
    
    def _extract_archive(self, archive_path: Path, extract_path: Path) -> bool:
        """Extract archive"""
        try:
            if archive_path.suffix in ['.tar', '.tgz', '.tar.gz']:
                subprocess.run(['tar', 'xf', str(archive_path), '-C', str(extract_path)], check=True)
            elif archive_path.suffix == '.zip':
                subprocess.run(['unzip', str(archive_path), '-d', str(extract_path)], check=True)
            elif archive_path.suffix == '.7z':
                subprocess.run(['7z', 'x', str(archive_path), f"-o{extract_path}"], check=True)
            else:
                self.log(f"Unsupported archive format: {archive_path.suffix}")
                return False
            return True
        except Exception as e:
            self.log(f"Failed to extract archive: {e}")
            return False
    
    def _create_live_config(self, rootfs_path: Path) -> None:
        """Create live boot configuration"""
        live_config = {
            "version": self.config.version,
            "boot_mode": self.config.boot_mode.value,
            "compression": self.config.compression.value,
            "filesystem": self.config.filesystem.value,
            "persistent": self.config.persistent,
            "encryption": self.config.encryption,
            "uefi_secure_boot": self.config.uefi_secure_boot,
            "custom_modules": self.config.custom_modules or []
        }
        
        config_path = rootfs_path / "etc" / "live" / "config.json"
        config_path.parent.mkdir(parents=True, exist_ok=True)
        
        with open(config_path, 'w') as f:
            json.dump(live_config, f, indent=2)
    
    def _generate_grub_config(self) -> str:
        """Generate GRUB configuration"""
        config = f"""# SigmaOS Live Boot Configuration
set default=0
set timeout=5

if [ "${grub_platform}" == "pc" ]; then
    # Legacy BIOS
    menuentry "SigmaOS Live (BIOS)" {{
        linux /boot/vmlinuz boot=live quiet splash
        initrd /boot/initrd
    }}
fi

if [ "${grub_platform}" == "efi" ]; then
    # UEFI
    menuentry "SigmaOS Live (UEFI)" {{
        linux /boot/vmlinuz boot=live quiet splash
        initrd /boot/initrd
    }}
fi

menuentry "SigmaOS Live (Verbose)" {{
    linux /boot/vmlinuz boot=live verbose
    initrd /boot/initrd
}}

menuentry "SigmaOS Live (Debug)" {{
    linux /boot/vmlinuz boot=live debug
    initrd /boot/initrd
}}

menuentry "Memory Test" {{
    linux16 /boot/memtest86+.bin
}}

menuentry "Reboot System" {{
    reboot
}}

menuentry "Shutdown System" {{
    halt
}}
"""
        
        return config
    
    def build(self) -> bool:
        """Build live boot image"""
        self.log(f"Starting SigmaOS Live Boot build: {self.config.name} v{self.config.version}")
        
        # Setup environment
        if not self.setup_environment():
            return False
        
        # Prepare components
        if not self.prepare_kernel():
            return False
        
        if not self.prepare_initrd():
            return False
        
        if not self.create_rootfs():
            return False
        
        if not self.create_squashfs():
            return False
        
        if not self.create_grub_config():
            return False
        
        # Create boot images based on mode
        if self.config.boot_mode in [BootMode.BIOS_LEGACY, BootMode.HYBRID]:
            if not self.create_bios_boot_image():
                return False
        
        if self.config.boot_mode in [BootMode.UEFI, BootMode.HYBRID]:
            if not self.create_uefi_boot_image():
                return False
        
        # Create persistent storage
        if not self.create_persistent_storage():
            return False
        
        # Create final image
        if self.config.output_path.endswith('.iso'):
            return self.create_iso_image()
        elif self.config.output_path.endswith('.img'):
            return self.create_usb_image()
        else:
            self.log("Unknown output format, defaulting to ISO")
            self.config.output_path += '.iso'
            return self.create_iso_image()
    
    def cleanup(self) -> None:
        """Cleanup build environment"""
        if self.temp_dir and os.path.exists(self.temp_dir):
            shutil.rmtree(self.temp_dir)
            self.log("Build environment cleaned up")
    
    def save_build_log(self, output_path: str) -> None:
        """Save build log"""
        try:
            with open(output_path, 'w') as f:
                f.write('\n'.join(self.build_log))
            self.log(f"Build log saved to: {output_path}")
        except Exception as e:
            self.log(f"Failed to save build log: {e}")

def main():
    """Main function"""
    if len(sys.argv) < 2:
        print("Usage: python live_boot_builder.py <config.json>")
        sys.exit(1)
    
    config_file = sys.argv[1]
    
    try:
        with open(config_file, 'r') as f:
            config_data = json.load(f)
        
        config = LiveBootConfig(**config_data)
        builder = LiveBootBuilder(config)
        
        # Build live boot image
        success = builder.build()
        
        # Save build log
        log_path = config.output_path + '.log'
        builder.save_build_log(log_path)
        
        # Cleanup
        builder.cleanup()
        
        if success:
            print(f"Live boot image created successfully: {config.output_path}")
            sys.exit(0)
        else:
            print("Failed to create live boot image")
            sys.exit(1)
            
    except Exception as e:
        print(f"Error: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
