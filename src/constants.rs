// Genetic Algorithm Parameters
pub const MAX_CHAMPIONS: usize = 10;
pub const POPULATION_SIZE: usize = 200;
pub const GENERATIONS: usize = 1500;
pub const MUTATION_RATE: f64 = 0.2;

// Fitness Weights
pub const CLASS_SYNERGY_WEIGHT: f64 = 2.0;
pub const ORIGIN_SYNERGY_WEIGHT: f64 = 8.0; // Much higher to prioritize origin stacking
pub const UNIQUE_CLASS_WEIGHT: f64 = 1.0;
pub const UNIQUE_ORIGIN_WEIGHT: f64 = 0.0; // Zero to completely discourage origin diversity
pub const ACTIVE_TRAIT_WEIGHT: f64 = 0.0; // Zero to completely eliminate diversity rewards
pub const ORIGIN_DIVERSITY_PENALTY: f64 = 7.0; // Penalty for having too many unique origins

// Selection Parameters
pub const ELITE_PERCENTAGE: f64 = 0.33; // 33% for elitism
pub const RANDOM_PERCENTAGE: f64 = 0.02; // 2% for random individuals (reduced to minimize origin diversity)
pub const TOURNAMENT_SIZE: usize = 3;
