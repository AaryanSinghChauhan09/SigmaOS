    # SigmaOS-packages

    Package manager core (resolve, install, verify)

    Part of the **SigmaOS** sovereign operating-system ecosystem — a multi-repository
    distribution modelled on Linux Mint and Omarchy Linux. See the parent
    [`SigmaOS`](https://github.com/AaryanSinghChauhan09/SigmaOS) repository for the
    full component index.

    ## Upstream inspirations
    - https://github.com/linuxmint/mintsources
- https://github.com/linuxmint/repolib

    ## Design principles applied
    - **Fail-Fast**
- **Pure Plan / Thin Execute**
- **KISS**
- **DRY**
- **Deterministic Testing**

    ## Usage
    Verbs: `resolve, install-plan, verify, repo`

    ```
    $ sigma-pkg resolve repo.json base desktop
$ sigma-pkg install-plan repo.json base
$ sigma-pkg verify installed.json repo.json base
$ sigma-pkg repo sigma-main https://repo.sigmaos.example/stable --trusted

    ```

    ## Build & test
    ```
    make build
    make test
    ```

    ## Licence
    GNU General Public License v3.0 (FOSS).
