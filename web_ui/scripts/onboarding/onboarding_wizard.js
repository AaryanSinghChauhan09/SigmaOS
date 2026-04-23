/**
 * SigmaOS: Sovereign Onboarding Wizard
 * Inspired by Zorin OS and Elementary OS.
 * USP: Guide the user through lattice configuration and personalization.
 */

const OnboardingWizard = {
    steps: [
        { title: "Welcome to SigmaOS", content: "You are now entering the Sovereign Lattice. Let's configure your environment." },
        { title: "Select Your Theme", content: "Choose a visual profile that matches your workflow.", action: () => SigmaThemingEngine.applyTheme('sovereign_dark') },
        { title: "Lattice Hardening", content: "Enable Capability-Based Security and Domain Isolation?", action: () => console.log("Σ://BOOT_HARDEN> Enabled.") },
        { title: "All Set!", content: "Your Sovereign Singularity is ready for deployment." }
    ],
    currentStep: 0,

    start() {
        this.renderStep();
    },

    renderStep() {
        const step = this.steps[this.currentStep];
        const win = SovereignUI.createWindow(step.title, 
            SovereignUI.createComponent('div', { className: 'onboarding-content' }, [
                SovereignUI.createComponent('p', {}, [step.content]),
                SovereignUI.createComponent('button', { 
                    className: 'next-btn', 
                    onClick: () => this.nextStep(win) 
                }, [this.currentStep === this.steps.length - 1 ? 'Finish' : 'Next'])
            ])
        );
        if (step.action) step.action();
    },

    nextStep(win) {
        win.remove();
        this.currentStep++;
        if (this.currentStep < this.steps.length) {
            this.renderStep();
        } else {
            UIUtils.appendLog('audit-log', 'SYSTEM: Onboarding complete. Welcome home.', 'success');
        }
    }
};

if (typeof window !== 'undefined') {
    window.SigmaOnboarding = OnboardingWizard;
}
