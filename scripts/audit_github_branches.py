#!/usr/bin/env python3
"""Read-only audit of branch and open-PR state across SigmaOS GitHub repos.

The audit never fetches or modifies git checkouts and never deletes a branch.
Branches other than the canonical `main` are listed for separate ancestry and
ownership review before any cleanup.
"""

from __future__ import annotations

import subprocess
import sys


QUERY = """query($login:String!, $endCursor:String) {
  user(login:$login) {
    repositories(first:100, after:$endCursor, affiliations:OWNER, orderBy:{field:NAME,direction:ASC}) {
      nodes {
        name
        defaultBranchRef { name }
        refs(refPrefix:"refs/heads/", first:100) {
          nodes { name }
          pageInfo { hasNextPage }
        }
        pullRequests(states:OPEN, first:1) { totalCount }
      }
      pageInfo { hasNextPage endCursor }
    }
  }
}"""


def gh(*args: str) -> str:
    result = subprocess.run(("gh", *args), text=True, capture_output=True, check=False)
    if result.returncode:
        detail = result.stderr.strip() or result.stdout.strip()
        raise RuntimeError(detail)
    return result.stdout


def main() -> int:
    try:
        owner = gh("api", "user", "--jq", ".login").strip()
        rows = gh(
            "api",
            "--paginate",
            "graphql",
            "-f",
            f"query={QUERY}",
            "-f",
            f"login={owner}",
            "--jq",
            "[.data.user.repositories.nodes[] | select(.name == \"SigmaOS\" or (.name | startswith(\"SigmaOS-\"))) | [.name, (.defaultBranchRef.name // \"\"), ([.refs.nodes[].name] | join(\",\")), (.refs.pageInfo.hasNextPage|tostring), (.pullRequests.totalCount|tostring)] | @tsv] | .[]",
        ).splitlines()
    except (FileNotFoundError, RuntimeError) as error:
        print(f"GitHub branch audit failed: {error}", file=sys.stderr)
        return 1

    repositories: list[tuple[str, str, list[str], bool, int]] = []
    for row in rows:
        fields = row.split("\t")
        if len(fields) != 5:
            print(f"Unexpected GitHub response row: {row!r}", file=sys.stderr)
            return 1
        name, default_branch, raw_branches, truncated, open_prs = fields
        branches = [branch for branch in raw_branches.split(",") if branch]
        repositories.append((name, default_branch, branches, truncated == "true", int(open_prs)))

    needs_review = [
        item
        for item in repositories
        if item[1] != "main" or item[2] != ["main"] or item[3] or item[4]
    ]
    print(f"SigmaOS repositories audited: {len(repositories)}")
    print(f"Repositories with exactly one main branch and no open PRs: {len(repositories) - len(needs_review)}")
    print(f"Repositories needing review: {len(needs_review)}")
    for name, default_branch, branches, truncated, open_prs in needs_review:
        print(
            f"{name}: default={default_branch or '(none)'}; branches={','.join(branches) or '(none)'}; "
            f"branch_list_truncated={truncated}; open_prs={open_prs}"
        )
    print("No branches were changed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
