import sys
import os
sys.path.append(os.path.join(os.getcwd(), 'userland', 'system-api'))
sys.path.append(os.path.join(os.getcwd(), 'sigma_core'))
sys.path.append(os.getcwd())

from sigma_games_engine import SigmaGamesEngine
from sigma_core.kernel import SigmaKernel

k = SigmaKernel()
ge = SigmaGamesEngine(k)
print(ge.health_check())
print("Installing Ludo Apex...")
res = ge.install_game("G02")
print(res["message"])
print(ge.play_game("G02"))
