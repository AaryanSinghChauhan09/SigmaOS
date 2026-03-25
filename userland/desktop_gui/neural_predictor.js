/**
 * Σ SIGMA OS PREDICTIVE NEURAL ENGINE v1.0
 * Zero-Dependency Machine Learning Time-Series Forecaster
 * Unsupervised Anomaly Detection for Kernel Process Monitoring
 */

export const NeuralPredictor = {
    initialized: false,
    history: [],
    maxHistory: 100,
    learningRate: 0.05,
    weights: [0.5, 0.5], // W1: CPU context, W2: RAM context
    bias: 0.1,
    threshold: 2.5, // Standard deviations for anomaly

    init() {
        if (this.initialized) return;
        this.initialized = true;
        console.log("[ML CORE] Time-Series Predictive Engine Online.");
    },

    // A simple Recurrent Neural Network (RNN) approximation
    feedForward(cpu, ram) {
        // Normalize inputs relative to expected bounds
        const nCpu = cpu / 100;
        const nRam = ram / 100;

        // Linear activation function
        let prediction = (this.weights[0] * nCpu) + (this.weights[1] * nRam) + this.bias;
        
        // Relu
        if (prediction < 0) prediction = 0;
        
        return prediction;
    },

    train(cpu, ram) {
        const actual = this.feedForward(cpu, ram);
        this.history.push({ cpu, ram, prediction: actual });

        if (this.history.length > this.maxHistory) {
            this.history.shift();
        }

        // Backpropagation (Stochastic Gradient Descent estimation)
        // If system is healthy (assumed normal data), adjust weights to make this the "expected" baseline
        const expected = 0.3; // Baseline stable entropy 
        const error = actual - expected;

        // Weight update rule
        this.weights[0] -= this.learningRate * error * (cpu / 100);
        this.weights[1] -= this.learningRate * error * (ram / 100);
        this.bias -= this.learningRate * error;
    },

    detectAnomaly(processes) {
        if (this.history.length < 10) return false; // Needs warm-up data

        // Calculate current system entropy
        let totalCpu = 0;
        let totalRam = 0;
        processes.forEach(p => {
            totalCpu += parseFloat(p.cpu);
            totalRam += parseFloat(p.mem);
        });

        const currentPrediction = this.feedForward(totalCpu, totalRam);
        
        // Calculate historical mean and variance (Gaussian Distribution approximation)
        const mean = this.history.reduce((sum, val) => sum + val.prediction, 0) / this.history.length;
        const variance = this.history.reduce((sum, val) => sum + Math.pow(val.prediction - mean, 2), 0) / this.history.length;
        const standardDeviation = Math.sqrt(variance);

        // Z-Score calculation
        const zScore = Math.abs((currentPrediction - mean) / (standardDeviation || 0.001));

        // Train the model on the new data
        this.train(totalCpu, totalRam);

        // Flag Anomaly 
        if (zScore > this.threshold) {
             console.warn(`[ML CORE] HEURISTIC ANOMALY DETECTED. Z-Score: ${zScore.toFixed(2)} | CPU: ${totalCpu.toFixed(1)}% | RAM: ${totalRam.toFixed(1)}%`);
             return { anomaly: true, zScore: zScore, severity: zScore > (this.threshold * 1.5) ? 'CRITICAL' : 'WARNING' };
        }

        return { anomaly: false };
    }
};
