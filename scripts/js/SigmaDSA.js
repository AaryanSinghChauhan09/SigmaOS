"use strict";

/**
 * Σ DSA VISUALIZER
 * Algorithm performance auditing and visualization.
 */
export class SigmaDSA {
    static async runViz(area, algo, spawnToast) {
        if (!area) return;
        
        const arr = Array(20).fill(0).map(() => Math.floor(Math.random() * 100));
        const render = (a) => {
            area.innerHTML = '';
            a.forEach(v => {
                const bar = document.createElement('div');
                bar.className = 'status-chip';
                bar.style.height = v + 'px';
                bar.style.width = '10px';
                bar.style.margin = '1px';
                area.appendChild(bar);
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
            spawnToast('DSA: Quicksort complete.');
        }
    }
}
