    # SigmaOS-security

    Privilege separation, LSM hooks, secure defaults

    Part of the **SigmaOS** sovereign operating-system ecosystem — a multi-repository
    distribution modelled on Linux Mint and Omarchy Linux. See the parent
    [`SigmaOS`](https://github.com/AaryanSinghChauhan09/SigmaOS) repository for the
    full component index.

    ## Upstream inspirations
    - https://github.com/omacom/omarchy/security

    ## Design principles applied
    - **Secure Defaults**
- **Least Privilege**
- **Fail-Fast**
- **Pure Plan / Thin Execute**
- **KISS**

    ## Usage
    Verbs: `audit, harden, caps, policy`

    ```
    $ sigma-sec audit sysctls.json
$ sigma-sec harden --profile strict
$ sigma-sec caps /usr/bin/ping CAP_NET_RAW
$ sigma-sec policy rules.json

    ```

    ## Build & test
    ```
    make build
    make test
    ```

    ## Licence
    GNU General Public License v3.0 (FOSS).
