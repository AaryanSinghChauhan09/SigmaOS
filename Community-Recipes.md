# Community `.spkg` Recipes (SlackBuilds-style)

1. Copy `example.hello.sigma.recipe` to `your-package.sigma.recipe`.

2. Set `curation=community` and a real `hash=sha256:...`.

3. Open a PR; CI runs `./scripts/ci_branch_check.sh`.

4. After merge, wiki sync publishes package notes via `sigma_automation.sh wiki-sync`.

Official recipes use `curation=official` and require maintainer signature.
