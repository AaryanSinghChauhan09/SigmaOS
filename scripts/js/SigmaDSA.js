"use strict";

import { SigmaShard } from './SigmaShard.js';

/**
 * Σ DSA VISUALIZER (OOPS-Refactored)
 */
export class SigmaDSA extends SigmaShard {
    constructor(system) {
        super('dsashard', 'DSA Viz', system);
        this.area = document.getElementById('dsa-viz-area');
        this.algoInput = document.getElementById('dsa-algo');
    }

    /**
     * Polymorphic implementation of execute() for Quicksort.
     */
    async execute() {
        if (!this.area || !this.algoInput) return;
        const algo = this.algoInput.value;
        const arr = Array(20).fill(0).map(() => Math.floor(Math.random() * 100));
        
        const render = (a) => {
            if (!this.area) return;
            this.area.innerHTML = '';
            a.forEach(v => {
                const bar = document.createElement('div');
                bar.className = 'status-chip';
                bar.style.height = v + 'px';
                bar.style.width = '10px';
                bar.style.margin = '1px';
                this.area.appendChild(bar);
            });
        };

        if (algo === 'QUICKSORT') {
            const sort = async (a, low, high) => {
                if (low < high) {
                    let pivot = a[high];
                    let i = low - 1;
                    for (let j = low; j < high; j++) {
                        if (a[j] < pivot) {
                            i++;
                            [a[i], a[j]] = [a[j], a[i]];
                            render(a);
                            await new Promise(r => setTimeout(r, 50));
                        }
                    }
                    [a[i + 1], a[high]] = [a[high], a[i + 1]];
                    render(a);
                    let pi = i + 1;
                    await sort(a, low, pi - 1);
                    await sort(a, pi + 1, high);
                }
            };
            await sort(arr, 0, arr.length - 1);
            this.log('Quicksort complete.');
        }
    }
}
