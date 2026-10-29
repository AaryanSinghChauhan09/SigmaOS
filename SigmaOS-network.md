    # SigmaOS-network

    Networking stack: TCP/IP, TLS, mDNS discovery

    Part of the **SigmaOS** sovereign operating-system ecosystem — a multi-repository
    distribution modelled on Linux Mint and Omarchy Linux. See the parent
    [`SigmaOS`](https://github.com/AaryanSinghChauhan09/SigmaOS) repository for the
    full component index.

    ## Upstream inspirations
    - native

    ## Design principles applied
    - **Fail-Fast**
- **Pure Plan / Thin Execute**
- **Secure Defaults**
- **Least Astonishment**
- **Deterministic Testing**

    ## Usage
    Verbs: `interfaces, tls, mdns, firewall-plan`

    ```
    $ sigma-net interfaces ifaces.json
$ sigma-net tls --min 1.3
$ sigma-net mdns services.json
$ sigma-net firewall-plan policy.json --default deny

    ```

    ## Build & test
    ```
    make build
    make test
    ```

    ## Licence
    GNU General Public License v3.0 (FOSS).
