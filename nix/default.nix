{ pkgs ? import <nixpkgs> {} }:

pkgs.rustPlatform.buildRustPackage rec {
  pname = "sigmaos-core";
  version = "1.0.0";

  src = ../.;

  cargoLock = {
    lockFile = ../Cargo.lock;
  };

  buildInputs = with pkgs; [
    openssl
    wayland
    btrfs-progs
  ];

  meta = with pkgs.lib; {
    description = "SigmaOS sovereign desktop and microkernel framework";
    homepage = "https://github.com/AaryanSinghChauhan09/SigmaOS";
    license = licenses.gpl3Plus;
    platforms = platforms.linux;
  };
}
