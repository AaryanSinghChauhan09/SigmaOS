/**
 * SigmaOS Performance Optimization Shard
 * Optimizes performance, speed, and capabilities
 */

class PerformanceMonitor {
    constructor() {
        this.shardId = 'PerformanceMonitor';
        this.metrics = {
            fps: 0,
            lastFrameTime: performance.now(),
            frameCount: 0,
            memoryUsage: 0,
            cpuLoad: 0
        };
    }

    startFPSCounter() {
        setInterval(() => {
            const now = performance.now();
            this.metrics.fps = Math.round(this.metrics.frameCount * 1000 / (now - this.metrics.lastFrameTime));
            this.metrics.lastFrameTime = now;
            this.metrics.frameCount = 0;
            console.log(`Σ://PERF> Current FPS: ${this.metrics.fps}`);
        }, 1000);
    }

    updateFrame() {
        this.metrics.frameCount++;
    }

    getMemoryUsage() {
        if (performance.memory) {
            this.metrics.memoryUsage = performance.memory.usedJSHeapSize / 1024 / 1024;
            console.log(`Σ://PERF> Memory usage: ${this.metrics.memoryUsage.toFixed(2)} MB`);
            return this.metrics.memoryUsage;
        }
        return null;
    }
}

class ObjectPool {
    constructor(createFn, resetFn, initialSize = 10) {
        this.shardId = 'ObjectPool';
        this.createFn = createFn;
        this.resetFn = resetFn;
        this.pool = [];
        this.inUse = [];
        for (let i = 0; i < initialSize; i++) {
            this.pool.push(this.createFn());
        }
        console.log(`Σ://PERF> ObjectPool initialized with ${initialSize} objects.`);
    }

    acquire() {
        let obj;
        if (this.pool.length > 0) {
            obj = this.pool.pop();
        } else {
            obj = this.createFn();
        }
        this.inUse.push(obj);
        return obj;
    }

    release(obj) {
        const index = this.inUse.indexOf(obj);
        if (index > -1) {
            this.inUse.splice(index, 1);
            if (this.resetFn) {
                this.resetFn(obj);
            }
            this.pool.push(obj);
        }
    }

    getStats() {
        return {
            poolSize: this.pool.length,
            inUseSize: this.inUse.length
        };
    }
}

class Debouncer {
    constructor(func, wait) {
        this.func = func;
        this.wait = wait;
        this.timeout = null;
    }

    debounce(...args) {
        clearTimeout(this.timeout);
        this.timeout = setTimeout(() => {
            this.func.apply(this, args);
        }, this.wait);
    }
}

class Throttler {
    constructor(func, limit) {
        this.func = func;
        this.limit = limit;
        this.inThrottle = false;
    }

    throttle(...args) {
        if (!this.inThrottle) {
            this.func.apply(this, args);
            this.inThrottle = true;
            setTimeout(() => {
                this.inThrottle = false;
            }, this.limit);
        }
    }
}

class SigmaPerformanceFramework {
    constructor() {
        this.shardId = 'S58_PerformanceOptimization';
        this.monitor = new PerformanceMonitor();
        console.log(`Σ://INIT> ${this.shardId} Initializing performance optimization framework...`);
        this.init();
    }

    init() {
        window.addEventListener('sigma.core.boot', () => {
            console.log(`Σ://PERF> ${this.shardId} Online. Performance framework active.`);
        });
    }

    createObjectPool(createFn, resetFn, initialSize) {
        return new ObjectPool(createFn, resetFn, initialSize);
    }

    createDebouncer(func, wait) {
        return new Debouncer(func, wait);
    }

    createThrottler(func, limit) {
        return new Throttler(func, limit);
    }

    startMonitoring() {
        this.monitor.startFPSCounter();
        setInterval(() => {
            this.monitor.getMemoryUsage();
        }, 5000);
    }
}

window.SigmaPerformance = new SigmaPerformanceFramework();
