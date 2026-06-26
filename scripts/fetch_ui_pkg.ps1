# fetch_ui_pkg.ps1
# Pulls package manager and GUI toolkits

Write-Host "Fetching pacman for sovereign package manager core..."
if (-not (Test-Path "lib\pacman")) {
    git clone https://gitlab.archlinux.org/pacman/pacman.git lib\pacman
}

Write-Host "Fetching GTK source tree..."
if (-not (Test-Path "lib\gtk")) {
    git clone https://gitlab.gnome.org/GNOME/gtk.git lib\gtk
}

Write-Host "Successfully fetched UI and Pkg tools!"
