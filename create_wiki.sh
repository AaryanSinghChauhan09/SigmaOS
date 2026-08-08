#!/bin/bash
cd /home/aaryansinghchauhan/SigmaOS
mkdir -p wiki

cat << 'WIKI' > wiki/Home.md
# Welcome to the SigmaOS Wiki

SigmaOS is a sovereign operating system written in Rust.
Explore the sidebar to learn more about architecture, security, and driver development.
WIKI

cat << 'WIKI' > wiki/Installation.md
# Installation
1. Install Rust nightly.
2. Install QEMU.
3. Clone repo and run `cargo run`.
WIKI

cat << 'WIKI' > wiki/Architecture.md
# Architecture
Modular monolithic with strict capability-based isolation.
WIKI

cat << 'WIKI' > wiki/Security.md
# Security
Capability tokens restrict syscalls.
Pledge/unveil isolate processes.
WIKI

cat << 'WIKI' > wiki/Driver-Development.md
# Driver Development
Write drivers by implementing Rust traits like `StorageDriver` or `NetworkDriver`.
WIKI

cat << 'WIKI' > wiki/Package-Management.md
# Package Management
SigmaPkg uses SAT-solvers and declarative state for atomic installations.
WIKI

cat << 'WIKI' > wiki/Kernel-Development.md
# Kernel Development
Kernel subsystems are located in `src/`. Avoid unsafe code.
WIKI

cat << 'WIKI' > wiki/Compatibility.md
# Compatibility
SigmaOS supports POSIX compatibility via FHS virtualization.
WIKI

cat << 'WIKI' > wiki/Roadmap.md
# Roadmap
- V1.0: Full POSIX support
- V2.0: GUI stack
WIKI

cat << 'WIKI' > wiki/FAQ.md
# FAQ
Q: Why Rust?
A: Memory safety.
WIKI

cat << 'WIKI' > wiki/Contributing.md
# Contributing
See `CONTRIBUTING.md` in the main repo.
WIKI

cat << 'WIKI' > wiki/Code-Scanning-Fixes.md
# Code Scanning Fixes
Recent updates removed unsafe transmutes, unused variables, and potential security risks from the codebase.
WIKI

chmod +x create_wiki.sh
./create_wiki.sh
