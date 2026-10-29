    # SigmaOS-desktop

    Wayland compositor shell & desktop primitives

    Part of the **SigmaOS** sovereign operating-system ecosystem — a multi-repository
    distribution modelled on Linux Mint and Omarchy Linux. See the parent
    [`SigmaOS`](https://github.com/AaryanSinghChauhan09/SigmaOS) repository for the
    full component index.

    ## Upstream inspirations
    - native

    ## Design principles applied
    - **Fail-Fast**
- **Least Astonishment**
- **Pure Plan / Thin Execute**
- **KISS**
- **Deterministic Testing**

    ## Usage
    Verbs: `outputs, inputs, keybinds, session`

    ```
    $ sigma-desktop outputs outputs.json
$ sigma-desktop inputs inputs.json
$ sigma-desktop keybinds keybinds.json
$ sigma-desktop session session.json

    ```

    ## Build & test
    ```
    make build
    make test
    ```

    ## Licence
    GNU General Public License v3.0 (FOSS).
