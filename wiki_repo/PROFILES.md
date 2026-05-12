# PROFILES

1

SigmaOS adopts a highly specialized, intent-driven OS model through its **Profession Profiles**. Instead of a generic desktop environment, the OS dynamically adapts its UI, underlying kernel policies, toolchain, and shortcuts based on the active profession profile.

1

The repository's `/profiles/` directory structure dictates the available configurations:

1

<<<<<<< HEAD

1

1

=======
>>>>>>> c682b9ae193869d405d851dfbeb13314cb964f9f

Each directory contains:

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1
<<<<<<< HEAD

1

1

1

1

1

1

1

=======
>>>>>>> c682b9ae193869d405d851dfbeb13314cb964f9f

1. **Create a new folder** under `/profiles/` (e.g., `/profiles/architect/`).
2. **Add `config.json`**: Define the profession name and active status.

3. **Link Tools**: Add symlinks to binaries in `/profiles/architect/shortcuts/`.
4. **Define Tools**: Write `/profiles/architect/tools.md` explaining the workflow.

5. **Restart Context Manager**: Run `agent.task run` and the OS will automatically adapt layout and quotas.

