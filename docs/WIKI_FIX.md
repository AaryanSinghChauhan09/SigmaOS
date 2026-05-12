# Σ SIGMAOS WIKI RESTORATION GUIDE

The SigmaOS Wiki currently suffers from broken navigation and missing pages. Follow these instructions to restore the Sovereign Documentation.

## 🔗 Fixing Broken Links

The GitHub Wiki uses a specific slug format. All relative links must point to the Wiki page name, not the file path.

**Incorrect**: `[API Reference](wiki/API_Reference.md)`
**Correct**: `[API Reference](API-Reference)`

### Action Plan:




1. Navigate to the `Home` page of the Wiki.
2. Update the Sidebar and Table of Contents to use the Wiki slug format (no extension, no directory prefix).



3. Ensure the following pages are populated with content from the `docs/` directory in the main repo.

## 📄 Page Population Matrix

| Wiki Page Name       | Source File (Main Repo)   |
| :------------------- | :------------------------ |
| **Home**             | `README.md` (Modularized) |
| **API Reference**    | `docs/API_REFERENCE.md`   |
| **CI Pipeline**      | `docs/CI_PIPELINE.md`     |
| **Sovereign Shards** | `os_guide.md`             |

## 🚀 Automation Shard

To automate Wiki synchronization, use the following GitHub Action pattern in your `.github/workflows/wiki-sync.yml`:

```yaml




* name: Push to Wiki
  uses: Andrew-Chen-Wang/github-wiki-action@v4
  with:
    path: docs/
    token: ${{ secrets.GITHUB_TOKEN }}

```

