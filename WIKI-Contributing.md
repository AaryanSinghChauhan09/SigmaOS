# Contributing to the SigmaOS Wiki

This guide explains how to contribute to the SigmaOS GitHub Wiki.

The wiki is the primary knowledge base for SigmaOS — covering architecture, features, roadmaps, and user documentation.

---

## Wiki Repository

The wiki is managed as a separate Git repository:

```text
https://github.com/AaryanSinghChauhan09/SigmaOS.wiki.git
```

It is mirrored locally at `wiki_repo/` inside the main SigmaOS repository.

---

## Quick Start

### 1. Clone the wiki

```bash
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.wiki.git
cd SigmaOS.wiki
```

### 2. Edit or add a page

Wiki pages are Markdown files at the root of the wiki repository. The filename becomes the URL slug:

- `Architecture-Overview.md` → `https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Architecture-Overview`


### 3. Commit and push

```bash
git add MyPage.md
git commit -m "wiki: add explanation of MyPage"
git push origin master
```

---

## Wiki Page Types

| Type | Naming | Example |
| ---- | ------ | ------- |
| Architecture | `Title-Case-Hyphenated.md` | `Kernel-Architecture.md` |
| Roadmaps | `Title-Roadmap.md` | `Development-Roadmap.md` |
| Absorption docs | `OSS_Absorption_Project.md` | `OSS_Absorption_Docker.md` |
| How-to guides | `Action-Verb-Topic.md` | `Building-from-Source.md` |

---

## Markdown Style

- Use a single `# H1` heading as the page title
- Headings must be surrounded by blank lines (MD022)
- Fenced code blocks must include the language identifier (MD040)
- Use `<!-- slide -->` comments only in carousel blocks
- No HTML-only markup; use Markdown equivalents


---

## Syncing from `wiki_repo/`

Pages in `wiki_repo/` inside the main repository are kept in sync with the live wiki:

```powershell

# Push wiki_repo changes to GitHub Wiki

git -C wiki_repo add .
git -C wiki_repo commit -m "wiki: update pages"
git -C wiki_repo push origin master
```

---

## What to Contribute

- Fix errors, outdated information, or broken links
- Add new architecture explanations or implementation notes
- Write how-to guides for common tasks
- Add distro absorption analysis pages for new projects
- Improve roadmap pages with concrete timelines


---

## Related

- [README.md](README.md) — Wiki home
- [CONTRIBUTING.md](../../CONTRIBUTING.md) — Main repo contributing guide
- [docs/CONTRIBUTING.md](../../docs/CONTRIBUTING.md) — Engineering docs guide
