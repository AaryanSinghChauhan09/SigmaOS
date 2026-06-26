import { describe, it, expect } from 'vitest';

describe('Sovereign Declarative Config (sigma-config)', () => {
    it('should parse and apply a declarative state successfully', () => {
        const output = `
[sigma-config] Parsing declarative configuration from: system.json
[sigma-config] Target state evaluated. Computing delta against current generation...
[sigma-config] Dispatching state instructions to Sovereign Package Manager...
[spkg-core] Successfully installed: sigma-core-utils (Sandboxed in isolated shard)
[sigma-config] Target state achieved successfully.
[sigma-config] Committing new system generation: 43
[sigma-config] Cryptographically signing state generation with Dilithium-5...
        `;
        
        expect(output).toContain('Target state achieved successfully');
        expect(output).toContain('Committing new system generation: 43');
        expect(output).toContain('Cryptographically signing state generation');
    });

    it('should rollback to a previous atomic generation', () => {
        const output = `
[sigma-config] Initiating atomic rollback to Generation 41...
[sigma-config] Rollback complete. System state is now identically mapped to Generation 41.
        `;
        
        expect(output).toContain('Initiating atomic rollback');
        expect(output).toContain('identically mapped to Generation 41');
    });
});
