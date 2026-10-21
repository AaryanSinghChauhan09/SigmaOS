# sigmaos-keyring - SigmaOS package signing keyring
# Modeled on Omarchy 4.0.6 omarchy-keyring package
pkgname=sigmaos-keyring
pkgver=2026.10
pkgrel=1
pkgdesc="SigmaOS package signing keyring"
arch=('any')
url="https://github.com/AaryanSinghChauhan09/SigmaOS"
license=('MIT OR GPL-2.0')
install=sigmaos-keyring.install

package() {
  mkdir -p "${pkgdir}/usr/share/pacman/keyrings"
  
  # Create SigmaOS keyring (placeholder - real keyring generated at build time)
  cat > "${pkgdir}/usr/share/pacman/keyrings/sigmaos.gpg" <<'EOF'
# SigmaOS package signing keyring
# Modeled on Omarchy 4.0.6 omarchy-keyring
# This is a placeholder. Real keyring is generated during package build.
EOF
  
  mkdir -p "${pkgdir}/usr/share/pacman/keyrings/trusted"
  cat > "${pkgdir}/usr/share/pacman/keyrings/sigmaos-trusted" <<'EOF'
# SigmaOS trusted keys
# Modeled on Omarchy 4.0.6 omarchy-keyring
EOF
  
  echo "[sigmaos-keyring] Keyring package installed"
}