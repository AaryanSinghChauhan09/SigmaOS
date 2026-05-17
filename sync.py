import os
os.system('git add zenith.html tools/sigma_vr_studio.cpp')
os.system('git commit -m "fix: Final HTML class merge and VR studio typo"')
os.system('py tools/sync_all_branches.py')
print("Synced!")
