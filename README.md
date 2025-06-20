# TFT Team Optimizer

A genetic algorithm-based team composition optimizer for Teamfight Tactics (TFT). This tool automatically generates optimal team compositions by balancing champion synergies, origins, and classes.

## Features

- **Genetic Algorithm Optimization**: Uses evolutionary computation to find optimal team compositions
- **Origin-Focused Strategy**: Prioritizes stacking champions from 1-2 main origins for maximum synergy
- **Class Synergy Balancing**: Efficiently collects class synergies while maintaining origin focus
- **Configurable Weights**: Fine-tune the algorithm's behavior through easily adjustable parameters
- **Comprehensive Output**: Detailed breakdown of team composition, synergies, and fitness scores

## How It Works

The optimizer uses a genetic algorithm with the following components:

### Fitness Function
Teams are scored based on:
- **Origin Synergies**: Rewards for having multiple champions of the same origin
- **Class Synergies**: Rewards for having multiple champions of the same class
- **Origin Diversity Penalty**: Penalizes teams with too many different origins (encourages focus on 1-2 main origins)
- **Class Diversity**: Rewards for having champions from different classes

### Genetic Algorithm Parameters
- **Population Size**: Number of teams in each generation
- **Generations**: Number of evolutionary cycles
- **Mutation Rate**: Probability of random champion swaps
- **Elitism**: Percentage of best teams that survive unchanged
- **Tournament Selection**: Method for selecting parent teams

## Installation

### Prerequisites
- Rust (latest stable version)

### Build and Run
```bash
# Clone the repository
git clone <your-repo-url>
cd tft

# Build the project
cargo build --release

# Run the optimizer
cargo run --release
```

## Configuration

All algorithm parameters are configurable in `src/constants.rs`:

### Genetic Algorithm Parameters
```rust
pub const MAX_CHAMPIONS: usize = 10;        // Team size
pub const POPULATION_SIZE: usize = 200;     // Population size
pub const GENERATIONS: usize = 1500;        // Number of generations
pub const MUTATION_RATE: f64 = 0.2;        // Mutation probability
```

### Fitness Weights
```rust
pub const CLASS_SYNERGY_WEIGHT: f64 = 2.0;           // Weight for class synergies
pub const ORIGIN_SYNERGY_WEIGHT: f64 = 8.0;          // Weight for origin synergies
pub const UNIQUE_CLASS_WEIGHT: f64 = 1.0;            // Weight for class diversity
pub const UNIQUE_ORIGIN_WEIGHT: f64 = 0.0;           // Weight for origin diversity
pub const ACTIVE_TRAIT_WEIGHT: f64 = 0.0;            // Weight for trait diversity
pub const ORIGIN_DIVERSITY_PENALTY: f64 = 7.0;       // Penalty for too many origins
```

### Selection Parameters
```rust
pub const ELITE_PERCENTAGE: f64 = 0.33;    // Percentage of elite teams
pub const RANDOM_PERCENTAGE: f64 = 0.02;   // Percentage of random teams
pub const TOURNAMENT_SIZE: usize = 3;      // Tournament selection size
```

## Strategy Configuration

### Origin-Focused Strategy (Current Default)
The current configuration is optimized for:
- **Stacking 1-2 main origins** with 4-6 champions each
- **Efficient class synergies** on the side
- **Minimal origin diversity** (penalty for 3+ origins)

### Alternative Strategies

#### Balanced Strategy
```rust
pub const ORIGIN_SYNERGY_WEIGHT: f64 = 4.0;
pub const UNIQUE_ORIGIN_WEIGHT: f64 = 1.0;
pub const ORIGIN_DIVERSITY_PENALTY: f64 = 0.0;
```

#### Class-Focused Strategy
```rust
pub const CLASS_SYNERGY_WEIGHT: f64 = 8.0;
pub const ORIGIN_SYNERGY_WEIGHT: f64 = 2.0;
pub const UNIQUE_CLASS_WEIGHT: f64 = 2.0;
```

## Output Format

The optimizer provides detailed output including:

```
=== BEST TEAM FOUND ===
Fitness Score: 45.67
Champions:
  Champion1 (Classes, Origin)
  Champion2 (Classes, Origin)
  ...

Trait Breakdown:
Classes:
  AMP: 2
  BASTION: 3
  BRUISER: 1
  ...

Unique Classes: 8
Unique Origins: 2

Origins:
  ANIMASQUAD: 5
  BOOMBOTS: 3
  CYBERBOSS: 1
  ...
```

## Understanding the Parameters

### Tournament Size
- **Smaller (1-2)**: More random selection, maintains diversity
- **Larger (5+)**: More selective, may converge too quickly
- **Current (3)**: Balanced approach

### Elite Percentage
- **Higher (50%+)**: Preserves more good solutions, less exploration
- **Lower (10-20%)**: More exploration, might lose good solutions
- **Current (33%)**: Balanced preservation vs exploration

### Origin Diversity Penalty
- **Higher values**: Stronger focus on 1-2 main origins
- **Lower values**: More tolerance for origin diversity
- **Current (7.0)**: Strong penalty for 3+ origins
