import subprocess

branches = [
    "release/app",
    "release/browser",
    "release/cloud",
    "release/distributed",
    "release/dual-boot",
    "release/microkernel",
    "release/mobile",
    "release/rtos",
    "release/standalone"
]

def run_command(cmd):
    result = subprocess.run(cmd, shell=True, capture_output=True, text=True)
    if result.returncode != 0:
        print(f"Error running {cmd}: {result.stderr}")
    return result.stdout

print("Starting cross-branch synchronization...")

for branch in branches:
    print(f"Syncing {branch}...")
    run_command(f"git checkout {branch}")
    run_command("git merge main --no-edit")
    run_command(f"git push origin {branch}")

print("Switching back to main...")
run_command("git checkout main")

print("Cross-branch synchronization COMPLETE.")
