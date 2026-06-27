# SPDX-License-Identifier: GPL-2.0-or-later
# sigma-pkg/templates/sigma-healthd/template.py
#
# Chimera Linux cports-style package template for sigma-healthd.
# Every field is typed Python — no shell magic, no string concatenation.
# Build runner: sigma-pkg/cbuild.py

pkgname  = "sigma-healthd"
pkgver   = "1.2.0"
pkgrel   = 0
pkgdesc  = "SigmaOS structured health monitoring daemon"
license  = "GPL-2.0-or-later"
url      = "https://sigma-os.dev"
sha256   = ""  # filled by cbuild after download

# Build system
build_style  = "cmake"
cmake_args   = [
    "-DCMAKE_BUILD_TYPE=Release",
    "-DSIGMA_PROFILE=standalone",
    "-DSIGMA_USE_HEALTHD=ON",
]

# Dependencies
makedepends = [
    "cmake>=3.20",
    "ninja",
    "sigma-klib-devel",
    "golang>=1.21",
]
depends = [
    "sigma-ds>=1.0.0",
    "sigma-core-libs>=0.3.0",
]

# Chimera-style typed hardening flags (no magic shell string concatenation)
hardening = [
    "vis",    # -fvisibility=hidden      — minimise exported symbol surface
    "cfi",    # -fsanitize=cfi           — control-flow integrity
    "ssp",    # -fstack-protector-strong — stack canaries
    "pie",    # -fPIE -pie               — position-independent executable
    "relro",  # -Wl,-z,relro,-z,now      — full RELRO (read-only GOT)
]

# Typed compiler/linker flags — not concatenated strings
tool_flags = {
    "CXXFLAGS": ["-fno-rtti", "-fno-exceptions"],
    "LDFLAGS":  ["-z", "max-page-size=0x1000"],
    "GOFLAGS":  ["-trimpath", "-buildvcs=false"],
}


def pre_build(self):
    """Run protoc before build to regenerate gRPC stubs."""
    self.do("protoc",
            "--go_out=.", "--go-grpc_out=.",
            "api/sigma.proto")


def post_install(self):
    """Install service file, MAC policy, and man page."""
    self.install_service("sigma-healthd.d",
                         "sigma/etc/services/")
    self.install_file("sigma-healthd.pol",
                      "sigma/mac/policy/")
    self.install_man("docs/man/sigma-healthd.8")


@subpackage("sigma-healthd-devel")
def _(self):
    self.pkgdesc = "Development headers for sigma-healthd"
    return [
        "sigma/include/sigma-healthd/**",
        "sigma/lib/pkgconfig/sigma-healthd.pc",
    ]
