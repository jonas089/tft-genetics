# TFT Genetic Algorithm Team Builder

A Rust-based genetic algorithm that automatically generates optimal Teamfight Tactics (TFT) team compositions by evolving populations of champion combinations to maximize trait synergies and strategic effectiveness.

## Overview

This project uses evolutionary computation to solve the complex optimization problem of building the best TFT team. The genetic algorithm evolves teams over multiple generations, using a sophisticated fitness function that rewards:

- **Trait Threshold Completion**: Progressive rewards for reaching trait breakpoints
- **Origin Synergies**: Higher rewards for origin traits vs class traits
- **Strategic Composition**: Balanced team structures with meaningful synergies
- **Origin Focus**: Penalizes excessive origin diversity to encourage focused compositions

## Features

### 🧬 Genetic Algorithm Engine
- **Population Evolution**: 200 teams evolve over 1500 generations
- **Tournament Selection**: Competitive selection for breeding
- **Crossover & Mutation**: Genetic operators for team diversity
- **Elitism**: Preserves best performing teams across generations

### 🎯 Advanced Fitness Scoring
- **Progressive Threshold Rewards**: Higher stages get exponentially more rewards
- **Origin Priority**: Origins rewarded 1.25x more than classes (7.5 vs 6.0 weight)
- **Synergy Optimization**: Focuses on completing trait synergies
- **Origin Diversity Control**: Penalizes teams with more than 2 unique origins

### 🏆 TFT Trait System
- **14 Origins**: Anima Squad, BoomBots, Cyberboss, Cypher, Divinicorp, Exotech, God of the Net, Golden Ox, Nitro, Overlord, Soul Killer, Street Demon, Syndicate, Virus
- **13 Classes**: AMP, Bastion, Bruiser, Dynamo, Executioner, Marksman, Rapidfire, Slayer, Strategist, Techie, Vanguard, Overlord, Soul Killer
- **Dynamic Thresholds**: Each trait has configurable breakpoints for bonuses

## How It Works

### 1. Team Representation
Each team consists of up to 10 champions, where each champion has:
- **Name**: Champion identifier
- **Classes**: Vector of class traits (e.g., [AMP, BRUISER])
- **Origin**: Single origin trait (e.g., ANIMASQUAD)

### 2. Fitness Evaluation
The algorithm evaluates teams using multiple criteria:

```rust
// Progressive threshold rewards
Stage 0 (1st threshold): 1.0 × weight
Stage 1 (2nd threshold): 1.5 × weight  
Stage 2 (3rd threshold): 2.0 × weight
Stage 3 (4th threshold): 2.5 × weight

// Origin vs Class weights
Class Threshold Weight: 6.0
Origin Threshold Weight: 7.5 (1.25x multiplier)

// Synergy rewards
Class Synergy Weight: 2.0
Origin Synergy Weight: 8.0

// Diversity control
Origin Diversity Penalty: 6.5 (for >2 unique origins)
```

### 3. Evolution Process
1. **Initialization**: Random teams generated
2. **Evaluation**: Fitness scores calculated for all teams
3. **Selection**: Tournament selection picks parents
4. **Crossover**: Teams combine champion pools
5. **Mutation**: Random champion swaps
6. **Elitism**: Best teams preserved
7. **Repeat**: Process continues for 1500 generations

## Configuration

Key parameters in `src/constants.rs`:

```rust
// Algorithm Parameters
MAX_CHAMPIONS: usize = 10
POPULATION_SIZE: usize = 200
GENERATIONS: usize = 1500
MUTATION_RATE: f64 = 0.2

// Fitness Weights
CLASS_THRESHOLD_WEIGHT: f64 = 6.0
ORIGIN_THRESHOLD_WEIGHT: f64 = 7.5
CLASS_SYNERGY_WEIGHT: f64 = 2.0
ORIGIN_SYNERGY_WEIGHT: f64 = 8.0
UNIQUE_CLASS_WEIGHT: f64 = 1.0
UNIQUE_ORIGIN_WEIGHT: f64 = 0.0
ACTIVE_TRAIT_WEIGHT: f64 = 0.0
ORIGIN_DIVERSITY_PENALTY: f64 = 6.5

// Selection Parameters
ELITE_PERCENTAGE: f64 = 0.33
RANDOM_PERCENTAGE: f64 = 0.02
TOURNAMENT_SIZE: usize = 3
```

## Usage

### Prerequisites
- Rust 1.70+ installed
- Cargo package manager

### Running the Algorithm
```bash
# Clone the repository
git clone <repository-url>
cd tft

# Build and run
cargo run --release
```

### Output Example
```
Total champions available: 58
Generation 0: Best fitness = 45.67
Generation 100: Best fitness = 78.23
Generation 200: Best fitness = 89.45
...

=== BEST TEAM FOUND ===
Fitness Score: 156.78
Champions:
  Vayne (SLAYER, ANIMASQUAD)
  Leona (VANGUARD, ANIMASQUAD)
  Seraphine (TECHIE, ANIMASQUAD)
  ...

Trait Breakdown:
Classes:
  AMP: 3
  BASTION: 2
  MARKSMAN: 2
  ...
Origins:
  ANIMASQUAD: 5
  BOOMBOTS: 3
  ...
```

## Project Structure

```
src/
├── main.rs          # Genetic algorithm implementation
├── types.rs         # Champion, Class, Origin definitions
├── champions.rs     # Champion database (58 champions)
└── constants.rs     # Algorithm parameters
```

## Trait Thresholds

### Example Origins
- **Anima Squad**: [3, 5, 7, 10] - Weapon scaling bonuses
- **BoomBots**: [2, 4, 7] - Missile damage bonuses
- **Cyberboss**: [2, 3, 4] - Health and AP bonuses
- **Divinicorp**: [1, 2, 3, 4, 5, 6, 7] - Stat granting bonuses

### Example Classes
- **AMP**: [2, 3, 4, 5] - Ability upgrades and Health
- **Bastion**: [2, 4, 6] - Armor and Magic Resist
- **Executioner**: [2, 3, 4, 5] - Critical strike bonuses
- **Techie**: [2, 4, 6, 8] - Ability Power and damage reduction

## Champion Database

The algorithm includes 58 champions across all origins and classes:

- **Anima Squad**: Aurora, Illaoi, Leona, Sylas, Seraphine, Vayne, Yuumi
- **BoomBots**: Chogath, Fiddlesticks, Kog'Maw, Skarner, Urgot
- **Cyberboss**: Kobuko, Poppy, Veigar, Ziggs
- **Cypher**: Draven, Galio, Leblanc, Vi, Zed
- **Divinicorp**: Gragas, Morgana, Rhaast, Senna, Vex, Renekton
- **Exotech**: Jax, Jhin, Mordekaiser, Naafiri, Sejuani, Varus, Zeri
- **God of the Net**: Garen
- **Golden Ox**: Alistar, Annie, Aphelios, Graves, Jarvan, Viego
- **Nitro**: Elise, Kindred, Nidalee, Shyvana
- **Overlord**: Overlord trait champions
- **Soul Killer**: Soul Killer trait champions
- **Street Demon**: Brand, Dr. Mundo, Ekko, Jinx, Neeko, Rengar, Samira, Zyra
- **Syndicate**: Braum, Darius, MissFortune, Shaco, Twisted Fate
- **Virus**: Zac

## Algorithm Advantages

### 🎯 Strategic Focus
- Prioritizes completing trait synergies over random combinations
- Rewards higher trait stages exponentially
- Balances origin and class synergies effectively
- Encourages focused origin compositions (1-2 main origins)

### 🚀 Performance
- Efficient Rust implementation
- Configurable parameters for different strategies
- Tournament selection for competitive evolution

### 🔧 Flexibility
- Easy to modify trait definitions
- Adjustable fitness weights
- Extensible for new TFT sets