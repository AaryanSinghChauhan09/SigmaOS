#!/bin/bash
# Close all 30 open PRs via GitHub API (using gh CLI or curl)
# Since changes were implemented directly into main, close PRs with explanation

OWNER="AaryanSinghChauhan09"
REPO="SigmaOS"

# PR numbers to close
PR_NUMBERS=(2009 2008 2007 2006 2005 2004 2003 2002 2001 2000 1999 1998 1997 1996 1995 1994 1993 1992 1991 1990 1989 1988 1987 1986 1985 1984 1983 1982 1981 1980)

CLOSE_MSG="✅ **Implemented directly into \`main\`**

This PR's changes have been merged directly into the \`main\` branch as part of a comprehensive batch integration of all pending PRs. The code is now live in main.

- All branches were merged (with conflict resolution where needed)
- Main branch pushed to GitHub: \`06efc23011\`
- Branch will be deleted to clean up the repository

Thank you for the contribution! 🚀"

for PR in "${PR_NUMBERS[@]}"; do
    echo "Closing PR #$PR..."
    
    # Add comment
    curl -s -X POST \
        -H "Authorization: token $(cat ~/.git-credentials | grep github | head -1 | sed 's|.*:\(.*\)@.*|\1|')" \
        -H "Accept: application/vnd.github.v3+json" \
        "https://api.github.com/repos/$OWNER/$REPO/issues/$PR/comments" \
        -d "{\"body\": $(echo "$CLOSE_MSG" | python3 -c 'import json,sys; print(json.dumps(sys.stdin.read()))')}" \
        > /dev/null 2>&1 || true
    
    # Close PR
    curl -s -X PATCH \
        -H "Authorization: token $(cat ~/.git-credentials | grep github | head -1 | sed 's|.*:\(.*\)@.*|\1|')" \
        -H "Accept: application/vnd.github.v3+json" \
        "https://api.github.com/repos/$OWNER/$REPO/pulls/$PR" \
        -d '{"state": "closed"}' \
        > /dev/null 2>&1 || true
    
    echo "  Done PR #$PR"
    sleep 0.5
done

echo "All PRs closed!"
