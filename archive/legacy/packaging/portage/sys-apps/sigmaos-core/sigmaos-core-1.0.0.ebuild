# Copyright 2026 SigmaOS Authors
# Distributed under the terms of the GNU General Public License v3

EAPI=8
CRATES=" "
inherit cargo

DESCRIPTION="SigmaOS Sovereign Desktop and Kernel Architecture Suite"
HOMEPAGE="https://github.com/AaryanSinghChauhan09/SigmaOS"
SRC_URI="https://github.com/AaryanSinghChauhan09/SigmaOS/archive/refs/tags/v${PV}.tar.gz -> ${P}.tar.gz"

LICENSE="GPL-3"
SLOT="0"
KEYWORDS="~amd64 ~arm64"
IUSE="wayland pipewire btrfs"

RDEPEND="
	dev-libs/wayland
	media-video/pipewire
	sys-fs/btrfs-progs
"
DEPEND="${RDEPEND}"
BDEPEND=">=virtual/rust-1.75.0"

src_install() {
	cargo_src_install
	dodoc README.md
}
