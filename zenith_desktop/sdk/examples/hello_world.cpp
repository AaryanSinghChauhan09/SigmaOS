/**
 * =========================================================================
 * Σ ZENITH SDK EXAMPLE: HELLO WORLD
 * =========================================================================
 * Demonstrates how to build a basic, containerized Zenith application.
 * Note that this app automatically runs inside an isolated Orchestrator 
 * Shard simply by initializing the Zenith::Application object.
 * =========================================================================
 */

#include "../include/zenith.h"

using namespace Zenith;
using namespace Zenith::UI;

int main() {
    // 1. Initialize the app. This automatically negotiates with the 
    // Sovereign Orchestrator to spawn a dedicated Container Shard.
    Application app("Sigma Hello World");

    // 2. Request a 800x600 window. The memory for this window is 
    // allocated inside the isolated container pool.
    app.createWindow(800, 600);

    // 3. Declarative UI setup
    Label title({ 300, 50, 200, 40 }, "Welcome to Sovereign Desktop");
    Button btn_click({ 325, 120, 150, 50 }, "Click Me!");

    app.addWidget(&title);
    app.addWidget(&btn_click);

    // 4. Enter the secure event loop
    app.run();

    return SIGMA_SUCCESS;
}
