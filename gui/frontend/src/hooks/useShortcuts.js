/**
 * gui/frontend/src/hooks/useShortcuts.js
 * Keyboard shortcut engine for SigmaOS Zenith Dashboard.
 * Listens for Ctrl+Key combinations and triggers corresponding actions.
 */

export function useShortcuts(shortcuts, actions) {
    function handleKeydown(e) {
        // Only trigger if Ctrl is pressed
        if (!e.ctrlKey) return;

        const key = e.key.toUpperCase();
        
        // Find matching action in the shortcuts map
        // Shortcuts are in format "Ctrl+K"
        for (const [actionName, combo] of Object.entries(shortcuts)) {
            const parts = combo.split('+');
            if (parts.length === 2 && parts[0] === 'Ctrl' && parts[1].toUpperCase() === key) {
                if (actions[actionName]) {
                    e.preventDefault();
                    console.log(`Σ://ZENITH_SHORTCUT> Triggering ${actionName} (${combo})`);
                    actions[actionName]();
                }
            }
        }
    }

    window.addEventListener('keydown', handleKeydown);
    
    // Return a cleanup function
    return () => {
        window.removeEventListener('keydown', handleKeydown);
    };
}
