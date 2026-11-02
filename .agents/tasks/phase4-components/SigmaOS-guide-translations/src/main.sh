#!/usr/bin/env bash
# SigmaOS Guide — Translation Contributor Guide CLI scaffold
# Upstream: GNOME Translation Project (https://l10n.gnome.org/),
#           Weblate (https://weblate.org/)
set -euo pipefail

GUIDE_VERSION="0.5"
GUIDE_NAME="SigmaOS Translation Guide"

usage() {
    cat >&2 <<EOF
Usage: $(basename "$0") <subcommand>

$GUIDE_NAME v$GUIDE_VERSION

Subcommands:
  overview     How SigmaOS translations work
  setup        Set up a translation environment
  workflow     Contribution workflow (translate → review → merge)
  status       Show translation completion by language
  locales      List supported locales
  po-guide     PO/POT file format and tools
  weblate      Using Weblate for online translation
  review       Guidelines for reviewing translations
  submit       How to submit completed translations
  version      Print guide version

EOF
}

print_overview() { cat <<'EOF'
SigmaOS Translation Overview
==============================
SigmaOS uses GNU gettext (PO/POT files) for UI translations.
Translations live in: .agents/tasks/phase4-components/SigmaOS-guide-translations/po/

Language teams contribute via:
  1. Weblate (preferred, online): translate.sigmaos.org
  2. Direct PO file editing + GitHub PR

Every translatable string is extracted from source with:
  xgettext -o po/sigmaos.pot src/**/*.py src/**/*.sh

Merged locale catalogs are built into the release ISO.
EOF
}

print_setup() { cat <<'EOF'
Translation Environment Setup
================================
1. Fork the SigmaOS repo and clone:
   git clone https://github.com/<you>/SigmaOS.git

2. Install gettext tools:
   sigmaos-pkg install gettext poedit

3. Copy the POT template to your locale:
   msginit -i po/sigmaos.pot -o po/fr.po -l fr_FR

4. Edit po/fr.po with Poedit, Gtranslator, or any text editor.

5. Verify the PO file:
   msgfmt --check po/fr.po

6. Submit a PR: see 'submit' subcommand.
EOF
}

print_workflow() { cat <<'EOF'
Contribution Workflow
======================
1. Claim your language on the Weblate team page or create a PO file.
2. Translate all msgid strings in the .po file.
3. Keep translation consistent: use the glossary in docs/translations/GLOSSARY.md.
4. Run msgfmt --check to validate.
5. Ask a native-speaker reviewer to review your PO file.
6. After approval, submit a PR (see 'submit').
7. Maintainer merges and the strings appear in the next release.
EOF
}

print_status() { cat <<'EOF'
Translation Completion Status (scaffold data)
==============================================
Locale  Language                Translated  Fuzzy  Untranslated
de      German                  95%         2%     3%
fr      French                  88%         4%     8%
es      Spanish                 91%         3%     6%
pt_BR   Brazilian Portuguese    82%         5%     13%
ja      Japanese                70%         8%     22%
zh_CN   Simplified Chinese      75%         5%     20%
ar      Arabic                  45%         10%    45%
(Updated at release build time from Weblate statistics.)
EOF
}

print_locales() { cat <<'EOF'
Supported Locales
==================
aa ar bg ca cs da de el en_US es et fi fr gl he
hi hr hu id it ja ko lt lv ms nb nl pa pl pt_BR
ro ru sk sl sq sr sv th tr uk vi zh_CN zh_TW
EOF
}

print_po_guide() { cat <<'EOF'
PO/POT File Format
===================
A .pot (template) contains:
  msgid  "Original English string"
  msgstr ""    (empty — filled by translator)

A translated .po adds msgstr:
  msgid  "Save"
  msgstr "Enregistrer"    (French)

Plural forms:
  msgid        "1 file"
  msgid_plural "%d files"
  msgstr[0]    "1 fichier"
  msgstr[1]    "%d fichiers"

Useful tools:
  msginit       Create a new PO from POT
  msgmerge      Merge updated POT into existing PO
  msgfmt        Compile PO to MO (binary)
  msgfmt --check  Validate PO syntax
  poedit        GUI editor for PO files
EOF
}

print_weblate() { cat <<'EOF'
Using Weblate (Online Translation)
====================================
1. Register at translate.sigmaos.org
2. Join your language team
3. Browse components and translate strings in the browser
4. Suggestions are reviewed by team leads
5. Approved strings sync to the git repo automatically

Weblate features:
  • Translation memory (reuse past translations)
  • Machine translation suggestions (optional)
  • Glossary enforcement
  • Review workflow with language team leads
EOF
}

print_review() { cat <<'EOF'
Review Guidelines
==================
Reviewer checklist:
  [ ] Terminology matches the SigmaOS Glossary (docs/translations/GLOSSARY.md)
  [ ] Plurals handled correctly for the target language
  [ ] Punctuation and capitalization follow the target language norms
  [ ] No untranslated strings left in msgstr
  [ ] No machine-translated strings without human review
  [ ] Context (msgctxt) respected where present
  [ ] msgfmt --check passes
EOF
}

print_submit() { cat <<'EOF'
Submitting Translations
========================
Via Weblate: automatic sync on approval — no extra steps needed.

Via GitHub PR:
  1. git checkout -b translations/fr
  2. git add po/fr.po
  3. git commit -m "translations(fr): update French locale to 91%"
  4. git push origin translations/fr
  5. Open a PR targeting main; request review from the i18n maintainer.

PR title convention: "translations(<locale>): <description>"
EOF
}

case "${1:-}" in
    overview)  print_overview ;;
    setup)     print_setup ;;
    workflow)  print_workflow ;;
    status)    print_status ;;
    locales)   print_locales ;;
    po-guide)  print_po_guide ;;
    weblate)   print_weblate ;;
    review)    print_review ;;
    submit)    print_submit ;;
    version)   echo "$GUIDE_NAME v$GUIDE_VERSION" ;;
    -h|--help|"") usage ;;
    *) echo "Unknown subcommand: $1" >&2; usage; exit 1 ;;
esac
