# fetch_fs_progs.ps1
# Pulls btrfs-progs, e2fsprogs, and systemd source trees

Write-Host "Fetching btrfs-progs..."
if (-not (Test-Path "lib\btrfs-progs")) {
    git clone https://github.com/kdave/btrfs-progs.git lib\btrfs-progs
}

Write-Host "Fetching e2fsprogs..."
if (-not (Test-Path "lib\e2fsprogs")) {
    git clone https://git.kernel.org/pub/scm/fs/ext2/e2fsprogs.git lib\e2fsprogs
}

Write-Host "Fetching OpenRC for init system replacement..."
if (-not (Test-Path "lib\openrc")) {
    git clone https://github.com/OpenRC/openrc.git lib\openrc
}

Write-Host "Successfully fetched Filesystems and Init programs!"
