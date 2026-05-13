# WIKI FIX

1

The SigmaOS Wiki currently suffers from broken navigation and missing pages. Follow these instructions to restore the Sovereign Documentation.

1

The GitHub Wiki uses a specific slug format. All relative links must point to the Wiki page name, not the file path.

**Incorrect**: `[API Reference](wiki/API_Reference)`

**Correct**: `[API Reference](API-Reference)`

1

1. Navigate to the `Home` page of the Wiki.
2. Update the Sidebar and Table of Contents to use the Wiki slug format (no extension, no directory prefix).

3. Ensure the following pages are populated with content from the `docs/` directory in the main repo.

1

To automate Wiki synchronization, use the following GitHub Action pattern in your `.github/workflows/wiki-sync.yml`:

1

1

  uses: Andrew-Chen-Wang/github-wiki-action@v4
  with:
    path: docs/
    token: ${{ secrets.GITHUB_TOKEN }}

1

