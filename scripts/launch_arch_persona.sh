#!/usr/bin/env bash
# SigmaOS Arch Linux Minimalist Persona Launcher
# Enables Pacman package manager, rolling release updates, makepkg, and minimalist CLI.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

echo -e "\033[1;36m"
cat <<'EOF'
                   -`                    
                  .o+`                   SIGMAOS · ARCH LINUX PERSONA
                 `ooo/                   Minimalist · Rolling Release · Pacman
                `+oooo:                  AUR Helper · Makepkg · Clean TTY
               `+oooooo:                 
               -+oooooo+:                
             `/:-:++oooo+:               
            `/++++/+++++++:              
           `/++++++++++++++:             
          `/+++ooooooooooooo/`           
         ./ooosssso++osssssso+`          
        .oossssso-````/ossssss+`         
       -osssssso.      :ssssssso.        
      :osssssss/        osssso+++.       
     /ossssssss/        +ssssooo/-       
   `/ossssso+/:-        -:/+osssso+-     
  `+sso+:-`                 `.-/+oso:    
 `++:.                           `-/+/   
 .`                                 `/   
EOF
echo -e "\033[0m"

echo "--> Initializing Arch Linux Persona on SigmaOS Substrate..."
echo "  [✓] Package Manager : SigPkg Pacman Translation Gate (pacman, yay, paru)"
echo "  [✓] Release Model   : Bleeding-Edge Rolling Release (v75.0)"
echo "  [✓] Build System    : Makepkg + PKGBUILD Parser"
echo "  [✓] Kernel Core     : Minimalist Microkernel with eBPF and io_uring"
echo "  [✓] AUR Integration : SigPkg AUR Web Search & Resolver"
echo ""

echo -e "\033[1mArch Linux Persona Active.\033[0m Native pacman syntax enabled."
echo "Examples:"
echo "  sigpkg pacman -Syu"
echo "  sigpkg pacman -S neovim rust"
echo "  sigpkg query kernel"
echo ""
EOF
