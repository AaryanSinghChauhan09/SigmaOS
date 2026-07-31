#!/usr/bin/env python3
import subprocess

branches_to_delete = [
    'bolt-crypto-ipc-opt-1946821541735961980',
    'bolt-package-opt-6708816887824727981',
    'feature/absorb-projects-6094893531408206919',
    'feature/sigmaos-strategic-roadmap-14829580979621853229',
    'jules-12799775792171137399-270bcaa3',
    'jules-14881431534840389204-c5224a09',
    'jules-17622072834113773464-03d7127e',
    'jules-1918597706965504152-80c6172c',
    'jules-1995206873680793168-00602c07',
    'jules-6767104542390946089-fb02bf86',
    'jules-8362645389262009630-ccefedb8',
    'jules-8725025787677827882-82aa0a51',
    'main-17021762207314737714',
    'sovereign-agent-repos-absorption-plans-8694159628422824584',
    'sovereign-universal-sufficiency-ultimate-plan-16625691866364960862',
    'universal-packaging-adapters-7113179245413805560'
]

print("1. Pushing main branch to origin...")
subprocess.run(['git', 'push', 'origin', 'main'])

print("2. Deleting merged remote branches on origin...")
for b in branches_to_delete:
    print(f"Deleting origin/{b}...")
    subprocess.run(['git', 'push', 'origin', '--delete', b])

print("3. Pushing updated Wiki repo...")
subprocess.run(['git', 'push', 'origin', 'master'], cwd='/tmp/SigmaOS_wiki')

print("Sync and Remote Cleanup Complete!")
