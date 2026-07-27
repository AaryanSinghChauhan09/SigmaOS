#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};

/// Kernel parameter to be tuned
#[derive(Debug, Clone, Copy)]
pub struct KernelParam {
    pub id: u32,
    pub name: &'static str,
    pub min: u64,
    pub max: u64,
    pub current: u64,
}

/// Fitness score for a parameter set
#[derive(Debug, Clone, Copy)]
pub struct FitnessScore {
    pub throughput: f64,
    pub latency: f64,
    pub power: f64,
    pub thermal: f64,
}

impl FitnessScore {
    pub fn composite(&self) -> f64 {
        let norm_throughput = self.throughput.clamp(0.0, 1.0);
        let norm_latency = 1.0 - self.latency.clamp(0.0, 1.0);
        let norm_power = 1.0 - self.power.clamp(0.0, 1.0);
        let norm_thermal = 1.0 - self.thermal.clamp(0.0, 1.0);
        0.4 * norm_throughput + 0.3 * norm_latency + 0.2 * norm_power + 0.1 * norm_thermal
    }
}

/// Individual in the genetic algorithm population
#[derive(Debug, Clone)]
pub struct AutotunerIndividual {
    pub params: Vec<u64>,
    pub fitness: Option<FitnessScore>,
    pub generation: u32,
}

impl AutotunerIndividual {
    pub fn new(params: Vec<u64>) -> Self {
        AutotunerIndividual {
            params,
            fitness: None,
            generation: 0,
        }
    }

    pub fn fitness(&self) -> Option<f64> {
        self.fitness.map(|f| f.composite())
    }
}

/// Genetic Kernel Autotuner
///
/// Uses a genetic algorithm to optimize kernel parameters
/// (scheduler quantum, VM writeback intervals, swappiness, etc.)
pub struct KernelGeneticAutotuner {
    pub population: Vec<AutotunerIndividual>,
    pub params: Vec<KernelParam>,
    pub generation: AtomicU32,
    pub best_fitness: f64,
    pub mutation_rate: f64,
    pub crossover_rate: f64,
    pub population_size: usize,
}

impl KernelGeneticAutotuner {
    pub fn new(params: Vec<KernelParam>, population_size: usize) -> Self {
        let mut rng = 0u64;
        let population = (0..population_size)
            .map(|_| {
                let genes = params
                    .iter()
                    .map(|p| {
                        rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
                        p.min + (rng % (p.max - p.min))
                    })
                    .collect();
                AutotunerIndividual::new(genes)
            })
            .collect();

        KernelGeneticAutotuner {
            population,
            params,
            generation: AtomicU32::new(0),
            best_fitness: 0.0,
            mutation_rate: 0.1,
            crossover_rate: 0.7,
            population_size,
        }
    }

    pub fn evaluate(&mut self, fitness_fn: impl Fn(&[u64]) -> FitnessScore) {
        for individual in &mut self.population {
            let score = fitness_fn(&individual.params);
            individual.fitness = Some(score);
            let composite = score.composite();
            if composite > self.best_fitness {
                self.best_fitness = composite;
            }
        }
        self.population.sort_by(|a, b| {
            let fa = a.fitness.map(|f| f.composite()).unwrap_or(0.0);
            let fb = b.fitness.map(|f| f.composite()).unwrap_or(0.0);
            fb.partial_cmp(&fa).unwrap_or(core::cmp::Ordering::Equal)
        });
    }

    pub fn evolve(&mut self) {
        let mut rng = 0u64;
        let elite_count = (self.population_size as f64 * 0.1) as usize;
        let mut new_population = self.population[..elite_count].to_vec();

        while new_population.len() < self.population_size {
            let parent1 = self.select();
            let parent2 = self.select();
            let mut child = self.crossover(parent1, parent2, &mut rng);
            self.mutate(&mut child, &mut rng);
            new_population.push(child);
        }

        self.population = new_population;
        self.generation.fetch_add(1, Ordering::SeqCst);
    }

    fn select(&self) -> &AutotunerIndividual {
        let mut rng = 0u64;
        let tournament_size = 3;
        let mut best_idx = 0;
        let mut best_fit = 0.0;
        for _ in 0..tournament_size {
            rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
            let idx = (rng % self.population.len() as u64) as usize;
            let fit = self.population[idx]
                .fitness
                .map(|f| f.composite())
                .unwrap_or(0.0);
            if fit > best_fit {
                best_fit = fit;
                best_idx = idx;
            }
        }
        &self.population[best_idx]
    }

    fn crossover(
        &self,
        a: &AutotunerIndividual,
        b: &AutotunerIndividual,
        rng: &mut u64,
    ) -> AutotunerIndividual {
        if *rng as f64 / u64::MAX as f64 > self.crossover_rate {
            return a.clone();
        }
        let mut child = Vec::new();
        for i in 0..a.params.len() {
            *rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
            if (*rng as f64 / u64::MAX as f64) < 0.5 {
                child.push(a.params[i]);
            } else {
                child.push(b.params[i]);
            }
        }
        AutotunerIndividual::new(child)
    }

    fn mutate(&self, individual: &mut AutotunerIndividual, rng: &mut u64) {
        for (i, param) in individual.params.iter_mut().enumerate() {
            if (*rng as f64 / u64::MAX as f64) < self.mutation_rate {
                let range = self.params[i].max - self.params[i].min;
                let delta = ((*rng % range) as i64) - (range as i64) / 2;
                let new_val = (*param as i64 + delta).clamp(self.params[i].min as i64, self.params[i].max as i64) as u64;
                *param = new_val;
            }
            *rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
        }
    }

    pub fn best(&self) -> Option<&AutotunerIndividual> {
        self.population.first()
    }

    pub fn generation(&self) -> u32 {
        self.generation.load(Ordering::SeqCst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_autotuner_creation() {
        let params = vec![
            KernelParam { id: 1, name: "quantum", min: 1, max: 1000, current: 100 },
            KernelParam { id: 2, name: "swappiness", min: 0, max: 100, current: 60 },
        ];
        let tuner = KernelGeneticAutotuner::new(params, 10);
        assert_eq!(tuner.population.len(), 10);
    }

    #[test]
    fn test_autotuner_evolution() {
        let params = vec![
            KernelParam { id: 1, name: "quantum", min: 1, max: 1000, current: 100 },
        ];
        let mut tuner = KernelGeneticAutotuner::new(params, 8);
        tuner.evaluate(|genes| FitnessScore {
            throughput: genes[0] as f64 / 1000.0,
            latency: 1.0 - (genes[0] as f64 / 1000.0),
            power: 0.5,
            thermal: 0.5,
        });
        tuner.evolve();
        assert_eq!(tuner.generation(), 1);
    }
}
