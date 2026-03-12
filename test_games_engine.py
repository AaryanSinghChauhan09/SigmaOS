import sys
import os

# Add the project root to sys.path
root = r"c:\Users\Aaryan\Downloads\SigmaOS"
sys.path.append(os.path.join(root, "userland", "system_api"))

try:
    from sigma_games_engine import SigmaGamesEngine
    
    # Mock kernel
    class MockKernel:
        def __init__(self):
            pass
            
    engine = SigmaGamesEngine(MockKernel())
    print(f"Engine status: {engine.health_check()}")
    print(f"Total games: {len(engine.list_games())}")
    
    # Check if a few games can be instantiated and hydrated
    for g_id in ["G01", "G50", "G100"]:
        cls = engine.catalog.get(g_id)
        if cls:
            game = cls()
            print(f"Instantiated {game.GAME_NAME} ({g_id})")
            print(f"Hydrating {g_id}: {game.hydrate()}")
            print(f"Health: {game.health_check()}")
        else:
            print(f"Error: Game {g_id} not found in catalog!")

except Exception as e:
    print(f"FAILURE: {e}")
    import traceback
    traceback.print_exc()
    sys.exit(1)
