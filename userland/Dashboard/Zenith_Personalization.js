/**
 * Zenith Dashboard - Automated Personalization Subsystem
 * Integrates SOVEREIGN_CONFIG.json into the sentient UI.
 */

async function applySovereignPersonalization() {
    try {
        const response = await fetch('SOVEREIGN_CONFIG.json');
        const config = await response.json();
        
        // Apply Identity
        document.getElementById('user-rank').textContent = config.identity.rank;
        document.getElementById('user-name').textContent = config.identity.sovereign_name;
        
        // Apply Sentient Theme logic
        if (config.zenith_ui.theme === "dark-chroma-sentient") {
            document.documentElement.style.setProperty('--sentient-primary', '#57c7ff'); // Sovereign Blue
            document.body.classList.add('chroma-mode');
        }
        
        console.log("Σ [ZENITH]: Personalization Shards seated.");
    } catch (e) {
        console.warn("Σ [ZENITH]: Personalized config not found. Reverting to base Sovereignty.");
    }
}

// Initialize on boot
document.addEventListener('DOMContentLoaded', applySovereignPersonalization);
