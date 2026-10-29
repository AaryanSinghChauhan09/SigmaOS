    # SigmaOS-kernel

    SigmaOS microkernel core (scheduler, VFS, syscall gate, memory)

    Part of the **SigmaOS** sovereign operating-system ecosystem — a multi-repository
    distribution modelled on Linux Mint and Omarchy Linux. See the parent
    [`SigmaOS`](https://github.com/AaryanSinghChauhan09/SigmaOS) repository for the
    full component index.

    ## Upstream inspirations
    - https://github.com/omacom/linux-aurora

    ## Design principles applied
    - **Separation of Mechanism & Policy**
- **Pure Plan / Thin Execute**
- **Fail-Fast**
- **KISS**
- **Deterministic Testing**

    ## Usage
    Verbs: `configure, build-plan, modules, clean`

    ```
    $ sigma-kernel configure base.config --set CONFIG_KVM=y --unset CONFIG_APPARMOR
$ sigma-kernel build-plan base.config --arch arm64 --jobs 8
$ sigma-kernel modules base.config --state m
$ sigma-kernel clean --target mrproper

    ```

    ## Build & test
    ```
    make build
    make test
    ```

    ## Licence
    GNU General Public License v3.0 (FOSS).
