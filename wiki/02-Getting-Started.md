# Getting Started

This guide is for building and exploring the SigmaOS source. It does not describe a supported desktop installation: the full boot-to-user-session path has not yet been validated.

## Prerequisites

- A Rust toolchain matching [`rust-toolchain.toml`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/rust-toolchain.toml).
- Git and a working C toolchain for repository scripts that compile native test fixtures.
- Optional: Python 3 for Python-based test utilities.

## Check out and build

```sh
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS
cargo check --lib
```

Expected result: Cargo reports a successful library check. This checks the hosted Rust library; it does not build or boot the bare-metal operating system.

## Run tests

Start with the affected module:

```sh
cargo test --lib <module_or_test_filter>
```

For a broader check:

```sh
cargo test --lib
./run_sigma_tests.sh
```

Record the exact commands and outcomes. A unit test for a model does not prove that the feature is connected to the kernel or works on hardware. See [Testing](Testing.md) for the validation ladder.

## Find the owning component

Start at [Home](Home.md) and open the component page before making changes. The component page records source paths, design references, current evidence, limitations, and the future roadmap.

## Troubleshooting

- **`cargo check` fails:** keep the complete compiler output and report the first actionable error with the Rust toolchain version.
- **A test fails:** rerun its exact filter and check whether the failure is deterministic before changing unrelated code.
- **You need a bootable image:** the current ISO and QEMU path is not a supported installation workflow; see [Installation status](01-Installation.md).
