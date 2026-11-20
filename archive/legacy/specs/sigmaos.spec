Name:           sigmaos-core
Version:        1.0.0
Release:        1%{?dist}
Summary:        SigmaOS Sovereign Operating System Core Architecture
License:        GPL-3.0-or-later
URL:            https://github.com/AaryanSinghChauhan09/SigmaOS
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  rust >= 1.75
BuildRequires:  cargo
BuildRequires:  wayland-devel
BuildRequires:  openssl-devel
Requires:       systemd
Requires:       btrfs-progs

%description
SigmaOS is a modular, sovereign operating system designed for high
reliability, container isolation, and dynamic desktop compositing.

%prep
%autosetup

%build
cargo build --release

%install
install -Dm755 target/release/omarchy-shell %{buildroot}%{_bindir}/omarchy-shell

%files
%{_bindir}/omarchy-shell

%changelog
* Sun Nov 15 2026 SigmaOS Team <team@sigmaos.org> - 1.0.0-1
- Initial Fedora RPM release.
