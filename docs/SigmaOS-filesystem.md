    # SigmaOS-filesystem

    Virtual filesystem layer, journaling & zero-copy paths

    Part of the **SigmaOS** sovereign operating-system ecosystem — a multi-repository
    distribution modelled on Linux Mint and Omarchy Linux. See the parent
    [`SigmaOS`](https://github.com/AaryanSinghChauhan09/SigmaOS) repository for the
    full component index.

    ## Upstream inspirations
    - native

    ## Design principles applied
    - **Pure Plan / Thin Execute**
- **Fail-Fast**
- **Defense in Depth**
- **KISS**
- **Deterministic Testing**

    ## Usage
    Verbs: `mount-plan, journal, paths, verify`

    ```
    $ sigma-vfs mount-plan spec.json
$ sigma-vfs journal ext4 --level ordered --commit 5
$ sigma-vfs paths spec.json
$ sigma-vfs verify spec.json

    ```

    ## Build & test
    ```
    make build
    make test
    ```

    ## Licence
    GNU General Public License v3.0 (FOSS).
