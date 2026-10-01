import os
import glob
import re

base_dir = "/home/aaryansinghchauhan/SigmaOS-work"
wiki_dir = os.path.join(base_dir, "wiki")
docs_dir = os.path.join(base_dir, "docs")

def append_to_file(target, source):
    with open(os.path.join(base_dir, source), 'r') as s, open(os.path.join(wiki_dir, target), 'a') as t:
        t.write('\n\n' + s.read())

def read_file(source):
    with open(os.path.join(base_dir, source), 'r') as s:
        return s.read()

def write_file(target, content):
    with open(os.path.join(wiki_dir, target), 'w') as t:
        t.write(content)

# 1. 11-Roadmap.md
append_to_file("11-Roadmap.md", "docs/PROJECT_STATUS.md")
append_to_file("11-Roadmap.md", "COMPLETION_STATUS.md")

# 2. 09-Packaging.md
append_to_file("09-Packaging.md", "docs/SIGMAOS_UNIVERSAL_PACKAGE_SYSTEM_LINUX_BSD_PARITY_PR_PROPOSAL.md")

# 3. 15-Architecture-Decisions.md
write_file("15-Architecture-Decisions.md", read_file("docs/ARCHITECTURE_DECISIONS.md"))

# 4. 16-Self-Sufficiency-Encyclopedia.md
v_files = sorted(glob.glob(os.path.join(docs_dir, "SOVEREIGN_OS_ABSOLUTE_OMNIPRESENT_SELF_SUFFICIENCY_ULTRA_ENCYCLOPEDIA_V*.md")))
v_content = "\n\n".join(open(f, 'r').read() for f in v_files)
write_file("16-Self-Sufficiency-Encyclopedia.md", "# Sovereign OS Self-Sufficiency Encyclopedia\n\n" + v_content)

# Remove transferred files
os.remove(os.path.join(base_dir, "COMPLETION_STATUS.md"))
os.remove(os.path.join(docs_dir, "PROJECT_STATUS.md"))
os.remove(os.path.join(docs_dir, "SIGMAOS_UNIVERSAL_PACKAGE_SYSTEM_LINUX_BSD_PARITY_PR_PROPOSAL.md"))
os.remove(os.path.join(docs_dir, "ARCHITECTURE_DECISIONS.md"))
for f in v_files:
    os.remove(f)

# Add AI Agent Maintenance Instructions
ai_instructions = """

## AI Agent Maintenance Instructions

- **Bolt ⚡**: Ensure documentation of any new zero-allocation optimizations or performance improvements are added concisely without marketing fluff.
- **Palette 🎨**: Maintain Arch Linux wiki style: clear, factual, one page per topic, using appropriate markdown formatting and tables where necessary.
- **Sentinel 🛡️**: Verify that no hardcoded credentials or unvetted cryptographic algorithms are documented as production-ready. Ensure security limitations are accurately stated.
- **General**: Keep pages up-to-date with current repository capabilities. Remove redundant files when consolidating information.
"""

for target in ["11-Roadmap.md", "09-Packaging.md", "15-Architecture-Decisions.md", "16-Self-Sufficiency-Encyclopedia.md"]:
    with open(os.path.join(wiki_dir, target), 'a') as t:
        t.write(ai_instructions)

# 5. Update 00-Home.md
home_content = read_file("wiki/00-Home.md")
# Remove old V38 and V40 lines
home_content = re.sub(r'- \[Sovereign OS Absolute Omnipresent Self-Sufficiency Ultra Encyclopedia V40\].*\n', '', home_content)
home_content = re.sub(r'- \[Sovereign OS Self-Sufficiency Encyclopedia V38\].*\n', '', home_content)
# Add new links
home_content = re.sub(r'- \[Roadmap\]\(11-Roadmap.md\)\n', 
    '- [Roadmap](11-Roadmap.md)\n- [Architecture Decisions](15-Architecture-Decisions.md)\n- [Self-Sufficiency Encyclopedia](16-Self-Sufficiency-Encyclopedia.md)\n', home_content)

with open(os.path.join(wiki_dir, "00-Home.md"), 'w') as t:
    t.write(home_content)

