import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import TelemetrySystem from '../modules/telemetry.js';

describe('TelemetrySystem', () => {
    let telemetry;

    beforeEach(() => {
        telemetry = new TelemetrySystem({ updateInterval: 100 });
    });

    afterEach(() => {
        telemetry.stop();
    });

    it('should initialize with default values', () => {
        expect(telemetry.smoothCpu).toBe(12);
        expect(telemetry.smoothMem).toBe(4.2);
    });

    it('should start and stop animation frame', () => {
        const spy = vi.spyOn(global, 'requestAnimationFrame');
        
        telemetry.start();
        expect(spy).toHaveBeenCalled();
        
        telemetry.stop();
        expect(telemetry.animationFrameId).toBeNull();
    });

    it('should update metrics within interval', async () => {
        telemetry.start();
        await new Promise(resolve => setTimeout(resolve, 150));
        
        expect(telemetry.metrics.length).toBeGreaterThan(0);
    });

    it('should calculate average CPU correctly', () => {
        telemetry.metrics = [
            { cpu: 10 },
            { cpu: 20 },
            { cpu: 30 }
        ];
        
        expect(telemetry.getAverageCPU()).toBe('20.00');
    });

    it('should bound metrics array', () => {
        telemetry.maxMetrics = 5;
        
        for (let i = 0; i < 10; i++) {
            telemetry.recordMetrics();
        }
        
        expect(telemetry.metrics.length).toBeLessThanOrEqual(5);
    });
});
