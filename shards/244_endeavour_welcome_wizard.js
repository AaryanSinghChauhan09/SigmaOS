/**
 * SigmaOS Endeavour Welcome Wizard Shard
 * Logic: EndeavourOS inspired user-friendly first-boot wizard for setup.
 */

class EndeavourWelcomeWizard {
    constructor() {
        this.shardId = "S" + "244_endeavour_welcome_wizard.js".split('_')[0] + "_EndeavourWelcomeWizard";
        this.active = false;
        
        console.log(`Σ://ULTIMATE> ${this.shardId} Initializing: Endeavour Welcome Wizard...`);
        this.init();
    }

    init() {
        window.addEventListener('sigma.core.boot', () => {
            this.active = true;
            console.log(`Σ://SINGULARITY> ${this.shardId} Online. EndeavourOS inspired user-friendly first-boot wizard for setup.`);
            this.registerCLI();
        });
    }

    registerCLI() {
        if(!window.SigmaCLI) window.SigmaCLI = {};
        window.SigmaCLI['welcome-os'] = (args) => {
            return `[Endeavour Welcome Wizard] Singularity Command: ${args.join(' ') || 'EXECUTE'}`;
        };
    }
}

window.SigmaEndeavourWelcomeWizard = new EndeavourWelcomeWizard();
