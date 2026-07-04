/**
 * SigmaOS Biometric Authentication Shard
 * Fingerprint, face recognition, and iris scan integration
 */

class BiometricAuth {
    constructor() {
        this.shardId = "S52_BiometricAuth";
        this.supportedMethods = ["fingerprint", "face", "iris"];
        this.enrolledTemplates = [];
        this.authHistory = [];

        console.log(`Σ://INIT> ${this.shardId} Initializing biometric authentication system...`);
        this.init();
    }

    init() {
        window.addEventListener('sigma.core.boot', () => {
            console.log(`Σ://BIOMETRIC> ${this.shardId} Online. Biometric authentication ready.`);
        });
    }

    enroll(method, templateData, userId) {
        if (!this.supportedMethods.includes(method)) {
            console.error(`Σ://BIOMETRIC> ${this.shardId} Unsupported method: ${method}`);
            return false;
        }

        this.enrolledTemplates.push({
            method,
            template: templateData,
            userId,
            timestamp: Date.now()
        });

        console.log(`Σ://BIOMETRIC> ${this.shardId} Enrolled ${method} for user ${userId}`);
        return true;
    }

    authenticate(method, inputData) {
        if (!this.supportedMethods.includes(method)) {
            console.error(`Σ://BIOMETRIC> ${this.shardId} Unsupported method: ${method}`);
            return { success: false, userId: null };
        }

        const template = this.enrolledTemplates.find(t => t.method === method);
        if (!template) {
            console.log(`Σ://BIOMETRIC> ${this.shardId} No ${method} template found`);
            return { success: false, userId: null };
        }

        const success = Math.random() > 0.1; // Simulate 90% success rate
        this.authHistory.push({
            method,
            success,
            timestamp: Date.now()
        });

        if (success) {
            console.log(`Σ://BIOMETRIC> ${this.shardId} ${method} authentication successful for user ${template.userId}`);
            window.dispatchEvent(new CustomEvent('sigma.auth.success', { 
                detail: { method, userId: template.userId } 
            }));
        } else {
            console.log(`Σ://BIOMETRIC> ${this.shardId} ${method} authentication failed`);
            window.dispatchEvent(new CustomEvent('sigma.auth.failure', { 
                detail: { method } 
            }));
        }

        return { success, userId: success ? template.userId : null };
    }

    getAuthHistory() {
        return this.authHistory;
    }
}

window.SigmaBiometricAuth = new BiometricAuth();
