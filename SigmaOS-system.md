    # SigmaOS-system

    Init, service manager & system orchestration

    Part of the **SigmaOS** sovereign operating-system ecosystem — a multi-repository
    distribution modelled on Linux Mint and Omarchy Linux. See the parent
    [`SigmaOS`](https://github.com/AaryanSinghChauhan09/SigmaOS) repository for the
    full component index.

    ## Upstream inspirations
    - native

    ## Design principles applied
    - **Separation of Mechanism & Policy**
- **Fail-Fast**
- **Pure Plan / Thin Execute**
- **KISS**
- **Deterministic Testing**

    ## Usage
    Verbs: `boot-plan, services, target, recover`

    ```
    $ sigma-init boot-plan --target graphical
$ sigma-init services units.json --target multi-user
$ sigma-init target units.json graphical
$ sigma-init recover fs-error bad-config

    ```

    ## Build & test
    ```
    make build
    make test
    ```

    ## Licence
    GNU General Public License v3.0 (FOSS).
