# Wiki source (maintainers)

Markdown in this folder is published to the [GitHub Wiki](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki). The file **`README.md` is not copied** to the wiki (maintainer instructions only).

**Numbered backlog:** the authoritative 1–100 list is **[`docs/FEATURE_ROADMAP_100.md`](../FEATURE_ROADMAP_100.md)**. The wiki page [Feature-Backlog-100](Feature-Backlog-100.md) summarizes usage and links there; edit the repo doc when changing items.

**Repo-wide future improvements:** **[`docs/REPO_FUTURE_IMPROVEMENTS.md`](../REPO_FUTURE_IMPROVEMENTS.md)** (detailed checklist); wiki hub [Future-Improvements](Future-Improvements.md). **GitHub Pages** ideas: [`docs/SITE_FUTURE_IMPROVEMENTS.md`](../SITE_FUTURE_IMPROVEMENTS.md).

## Automatic sync (GitHub Actions)

Workflow: `.github/workflows/wiki-sync.yml` (at repository root).

Triggers on pushes to `main` / `lattice-dev` when `docs/wiki/**` changes, and on `workflow_dispatch`.

### Required secret

| Secret | Purpose |
|--------|---------|
| `WIKI_SYNC_TOKEN` | PAT with **`repo`** scope (classic) or fine-grained access to **contents** of this repository and its wiki |

Create the PAT under GitHub → Settings → Developer settings. Add it to:

**Repository → Settings → Secrets and variables → Actions → New repository secret**

Name: `WIKI_SYNC_TOKEN`

If the secret is **missing**, the workflow prints a notice and exits successfully (CI stays green).

### Manual sync (fallback)

```bash

git clone https://github.com/AaryanSinghChauhan09/SigmaOS.wiki.git sigmaos-wiki
cd sigmaos-wiki
cp /path/to/SigmaOS/docs/wiki/*.md .

# Do not copy README.md from docs/wiki unless you want a "README" wiki page

rm -f README.md
git add -A
git commit -m "Sync wiki"
git push

```

## Page names

GitHub Wiki maps `Page-Name.md` → wiki URL `.../wiki/Page-Name`. This folder uses hyphenated names (e.g. `Getting-Started.md`).

