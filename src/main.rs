use crate::constants::*;
use crate::types::{
    Champion, Class, Origin, amp_details, animasquad_details, bastion_details, boombots_details,
    bruiser_details, cyberboss_details, cypher_details, divinicorp_details, dynamo_details,
    executioner_details, exotech_details, godofthenet_details, goldenox_details, marksman_details,
    nitro_details, overlord_details, rapidfire_details, slayer_details, soulkiller_details,
    strategist_details, streetdemon_details, syndicate_details, techie_details, vanguard_details,
    virus_details,
};
use std::collections::HashSet;

mod champions;
mod constants;
mod types;

pub struct Team {
    pub champions: Vec<Champion>,
}

impl Team {
    pub fn new() -> Self {
        Self { champions: vec![] }
    }

    pub fn add_champion(&mut self, champion: Champion) {
        self.champions.push(champion);
    }

    pub fn amp(&self) -> usize {
        self.champions
            .iter()
            .filter(|c| c.classes.contains(&Class::AMP))
            .count()
    }

    pub fn bastion(&self) -> usize {
        self.champions
            .iter()
            .filter(|c| c.classes.contains(&Class::BASTION))
            .count()
    }

    pub fn bruiser(&self) -> usize {
        self.champions
            .iter()
            .filter(|c| c.classes.contains(&Class::BRUISER))
            .count()
    }

    pub fn dynamo(&self) -> usize {
        self.champions
            .iter()
            .filter(|c| c.classes.contains(&Class::DYNAMO))
            .count()
    }

    pub fn executioner(&self) -> usize {
        self.champions
            .iter()
            .filter(|c| c.classes.contains(&Class::EXECUTIONER))
            .count()
    }

    pub fn marksman(&self) -> usize {
        self.champions
            .iter()
            .filter(|c| c.classes.contains(&Class::MARKSMAN))
            .count()
    }

    pub fn rapidfire(&self) -> usize {
        self.champions
            .iter()
            .filter(|c| c.classes.contains(&Class::RAPIDFIRE))
            .count()
    }

    pub fn slayer(&self) -> usize {
        self.champions
            .iter()
            .filter(|c| c.classes.contains(&Class::SLAYER))
            .count()
    }

    pub fn strategist(&self) -> usize {
        self.champions
            .iter()
            .filter(|c| c.classes.contains(&Class::STRATEGIST))
            .count()
    }

    pub fn techie(&self) -> usize {
        self.champions
            .iter()
            .filter(|c| c.classes.contains(&Class::TECHIE))
            .count()
    }

    pub fn vanguard(&self) -> usize {
        self.champions
            .iter()
            .filter(|c| c.classes.contains(&Class::VANGUARD))
            .count()
    }

    pub fn overlord(&self) -> usize {
        self.champions
            .iter()
            .filter(|c| c.classes.contains(&Class::OVERLORD))
            .count()
    }

    pub fn soulkiller(&self) -> usize {
        self.champions
            .iter()
            .filter(|c| c.classes.contains(&Class::SOULKILLER))
            .count()
    }

    pub fn origin(&self, origin: Origin) -> usize {
        self.champions
            .iter()
            .filter(|c| c.origion == origin)
            .count()
    }

    pub fn total_unique(&self) -> usize {
        let mut unique_classes = HashSet::new();
        for champion in &self.champions {
            for class in &champion.classes {
                unique_classes.insert(class);
            }
        }
        unique_classes.len()
    }

    pub fn total_unique_origins(&self) -> usize {
        let mut unique_origins = HashSet::new();
        for champion in &self.champions {
            unique_origins.insert(&champion.origion);
        }
        unique_origins.len()
    }

    // Helper function to calculate threshold rewards for a trait
    fn calculate_threshold_reward(&self, count: usize, thresholds: &[usize]) -> f64 {
        let mut reward = 0.0;
        for (index, &threshold) in thresholds.iter().enumerate() {
            if count >= threshold {
                // Progressive reward: higher stages (later indices) get more rewards
                // Base reward is 1.0, each stage adds 0.5 more reward
                let stage_reward = 1.0 + (index as f64 * 0.5);
                reward += stage_reward;
            }
        }
        reward
    }

    // Calculate a fitness score for the team
    pub fn fitness(&self) -> f64 {
        let mut score = 0.0;

        // Count each class and origin
        let class_counts = vec![
            self.amp(),
            self.bastion(),
            self.bruiser(),
            self.dynamo(),
            self.executioner(),
            self.marksman(),
            self.rapidfire(),
            self.slayer(),
            self.strategist(),
            self.techie(),
            self.vanguard(),
            self.overlord(),
            self.soulkiller(),
        ];

        let origin_counts = vec![
            self.origin(Origin::ANIMASQUAD),
            self.origin(Origin::BOOMBOTS),
            self.origin(Origin::CYBERBOSS),
            self.origin(Origin::CYPHER),
            self.origin(Origin::DIVINICORP),
            self.origin(Origin::EXOTECH),
            self.origin(Origin::GODOFTHENET),
            self.origin(Origin::GOLDENOX),
            self.origin(Origin::NITRO),
            self.origin(Origin::OVERLORD),
            self.origin(Origin::SOULKILLER),
            self.origin(Origin::STREETDEMON),
            self.origin(Origin::SYNCIDATE),
            self.origin(Origin::VIRUS),
        ];

        // Get class thresholds
        let class_thresholds = vec![
            amp_details().thresholds,
            bastion_details().thresholds,
            bruiser_details().thresholds,
            dynamo_details().thresholds,
            executioner_details().thresholds,
            marksman_details().thresholds,
            rapidfire_details().thresholds,
            slayer_details().thresholds,
            strategist_details().thresholds,
            techie_details().thresholds,
            vanguard_details().thresholds,
            overlord_details().thresholds,
            soulkiller_details().thresholds,
        ];

        // Get origin thresholds
        let origin_thresholds = vec![
            animasquad_details().thresholds,
            boombots_details().thresholds,
            cyberboss_details().thresholds,
            cypher_details().thresholds,
            divinicorp_details().thresholds,
            exotech_details().thresholds,
            godofthenet_details().thresholds,
            goldenox_details().thresholds,
            nitro_details().thresholds,
            overlord_details().thresholds,
            soulkiller_details().thresholds,
            streetdemon_details().thresholds,
            syndicate_details().thresholds,
            virus_details().thresholds,
        ];

        // Calculate threshold rewards for classes
        for (count, thresholds) in class_counts.iter().zip(class_thresholds.iter()) {
            score += self.calculate_threshold_reward(*count, thresholds) * CLASS_THRESHOLD_WEIGHT;
        }

        // Calculate threshold rewards for origins (1.5x multiplier)
        for (count, thresholds) in origin_counts.iter().zip(origin_thresholds.iter()) {
            score += self.calculate_threshold_reward(*count, thresholds) * ORIGIN_THRESHOLD_WEIGHT;
        }

        // Reward for having multiple of the same trait (synergy)
        // Use a more balanced approach - reward synergies but not too heavily
        for count in &class_counts {
            if *count >= 2 {
                score += (*count as f64) * CLASS_SYNERGY_WEIGHT;
            }
        }

        for count in &origin_counts {
            if *count >= 2 {
                score += (*count as f64) * ORIGIN_SYNERGY_WEIGHT;
            }
        }

        // Reward for diversity (unique traits) - more weight on diversity
        score += self.total_unique() as f64 * UNIQUE_CLASS_WEIGHT;
        score += self.total_unique_origins() as f64 * UNIQUE_ORIGIN_WEIGHT;

        // Bonus for having a good mix of different trait counts
        let active_classes = class_counts.iter().filter(|&&c| c > 0).count();
        let active_origins = origin_counts.iter().filter(|&&c| c > 0).count();
        score += (active_classes + active_origins) as f64 * ACTIVE_TRAIT_WEIGHT;

        // Penalty for having too many unique origins (encourage focus on 1-2 main origins)
        let unique_origins = self.total_unique_origins();
        if unique_origins > 2 {
            score -= (unique_origins - 2) as f64 * ORIGIN_DIVERSITY_PENALTY;
        }

        score
    }
}

fn main() {
    let all_champions = champions::champions();
    println!("Total champions available: {}", all_champions.len());

    // Initialize population with random teams
    let mut population: Vec<Team> = Vec::new();
    for _ in 0..POPULATION_SIZE {
        let mut team = Team::new();
        let mut used_indices = HashSet::new();

        while team.champions.len() < MAX_CHAMPIONS {
            let idx = rand::random::<usize>() % all_champions.len();
            if !used_indices.contains(&idx) {
                team.add_champion(all_champions[idx].clone());
                used_indices.insert(idx);
            }
        }
        population.push(team);
    }

    let mut best_team = None;
    let mut best_fitness = 0.0;

    // Evolution loop
    for generation in 0..GENERATIONS {
        // Evaluate fitness for all teams
        let mut fitness_scores: Vec<(usize, f64)> = population
            .iter()
            .enumerate()
            .map(|(i, team)| (i, team.fitness()))
            .collect();

        // Sort by fitness (descending)
        fitness_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Update best team
        if fitness_scores[0].1 > best_fitness {
            best_fitness = fitness_scores[0].1;
            best_team = Some(population[fitness_scores[0].0].champions.clone());
        }

        if generation % 100 == 0 {
            println!(
                "Generation {}: Best fitness = {:.2}",
                generation, best_fitness
            );
        }

        // Create new population
        let mut new_population: Vec<Team> = Vec::new();

        // Keep top teams (elitism)
        let elite_count = (POPULATION_SIZE as f64 * ELITE_PERCENTAGE) as usize;
        for i in 0..elite_count {
            new_population.push(Team {
                champions: population[fitness_scores[i].0].champions.clone(),
            });
        }

        // Add random individuals to maintain diversity
        let random_count = (POPULATION_SIZE as f64 * RANDOM_PERCENTAGE) as usize;
        for _ in 0..random_count {
            let mut team = Team::new();
            let mut used_indices = HashSet::new();

            while team.champions.len() < MAX_CHAMPIONS {
                let idx = rand::random::<usize>() % all_champions.len();
                if !used_indices.contains(&idx) {
                    team.add_champion(all_champions[idx].clone());
                    used_indices.insert(idx);
                }
            }
            new_population.push(team);
        }

        // Generate rest through crossover and mutation
        while new_population.len() < POPULATION_SIZE {
            // Tournament selection
            let parent1_idx = tournament_selection(&fitness_scores, TOURNAMENT_SIZE);
            let parent2_idx = tournament_selection(&fitness_scores, TOURNAMENT_SIZE);

            let mut child = crossover(
                &population[parent1_idx],
                &population[parent2_idx],
                &all_champions,
            );

            // Mutation
            if rand::random::<f64>() < MUTATION_RATE {
                mutate(&mut child, &all_champions);
            }

            new_population.push(child);
        }

        population = new_population;
    }

    // Print best team
    if let Some(best_champions) = best_team {
        let best_team = Team {
            champions: best_champions,
        };
        println!("\n=== BEST TEAM FOUND ===");
        println!("Fitness Score: {:.2}", best_team.fitness());
        println!("Champions:");
        for champion in &best_team.champions {
            println!(
                "  {} ({:?}, {:?})",
                champion.name, champion.classes, champion.origion
            );
        }

        println!("\nTrait Breakdown:");
        println!("Classes:");
        println!("  AMP: {}", best_team.amp());
        println!("  BASTION: {}", best_team.bastion());
        println!("  BRUISER: {}", best_team.bruiser());
        println!("  DYNAMO: {}", best_team.dynamo());
        println!("  EXECUTIONER: {}", best_team.executioner());
        println!("  MARKSMAN: {}", best_team.marksman());
        println!("  RAPIDFIRE: {}", best_team.rapidfire());
        println!("  SLAYER: {}", best_team.slayer());
        println!("  STRATEGIST: {}", best_team.strategist());
        println!("  TECHIE: {}", best_team.techie());
        println!("  VANGUARD: {}", best_team.vanguard());
        println!("  OVERLORD: {}", best_team.overlord());
        println!("  SOULKILLER: {}", best_team.soulkiller());

        println!("Unique Classes: {}", best_team.total_unique());
        println!("Unique Origins: {}", best_team.total_unique_origins());

        println!("\nOrigins:");
        println!("  ANIMASQUAD: {}", best_team.origin(Origin::ANIMASQUAD));
        println!("  BOOMBOTS: {}", best_team.origin(Origin::BOOMBOTS));
        println!("  CYBERBOSS: {}", best_team.origin(Origin::CYBERBOSS));
        println!("  CYPHER: {}", best_team.origin(Origin::CYPHER));
        println!("  DIVINICORP: {}", best_team.origin(Origin::DIVINICORP));
        println!("  EXOTECH: {}", best_team.origin(Origin::EXOTECH));
        println!("  GODOFTHENET: {}", best_team.origin(Origin::GODOFTHENET));
        println!("  GOLDENOX: {}", best_team.origin(Origin::GOLDENOX));
        println!("  NITRO: {}", best_team.origin(Origin::NITRO));
        println!("  OVERLORD: {}", best_team.origin(Origin::OVERLORD));
        println!("  SOULKILLER: {}", best_team.origin(Origin::SOULKILLER));
        println!("  STREETDEMON: {}", best_team.origin(Origin::STREETDEMON));
        println!("  SYNCIDATE: {}", best_team.origin(Origin::SYNCIDATE));
        println!("  VIRUS: {}", best_team.origin(Origin::VIRUS));
    }
}

fn tournament_selection(fitness_scores: &[(usize, f64)], tournament_size: usize) -> usize {
    let mut best_idx = 0;
    let mut best_fitness = 0.0;

    for _ in 0..tournament_size {
        let idx = rand::random::<usize>() % fitness_scores.len();
        if fitness_scores[idx].1 > best_fitness {
            best_fitness = fitness_scores[idx].1;
            best_idx = fitness_scores[idx].0;
        }
    }

    best_idx
}

fn crossover(parent1: &Team, parent2: &Team, all_champions: &[Champion]) -> Team {
    let mut child = Team::new();
    let mut used_names = HashSet::new();

    // Take champions from both parents randomly
    for champion in &parent1.champions {
        if rand::random::<bool>() && !used_names.contains(&champion.name) {
            child.add_champion(champion.clone());
            used_names.insert(champion.name.clone());
        }
    }

    for champion in &parent2.champions {
        if child.champions.len() < MAX_CHAMPIONS && !used_names.contains(&champion.name) {
            child.add_champion(champion.clone());
            used_names.insert(champion.name.clone());
        }
    }

    // Fill remaining slots with random champions
    while child.champions.len() < MAX_CHAMPIONS {
        let random_champion = &all_champions[rand::random::<usize>() % all_champions.len()];
        if !used_names.contains(&random_champion.name) {
            child.add_champion(random_champion.clone());
            used_names.insert(random_champion.name.clone());
        }
    }

    child
}

fn mutate(team: &mut Team, all_champions: &[Champion]) {
    if team.champions.is_empty() {
        return;
    }

    // Randomly replace one champion
    let replace_idx = rand::random::<usize>() % team.champions.len();
    let current_names: HashSet<String> = team.champions.iter().map(|c| c.name.clone()).collect();

    // Find a champion not already in the team
    let mut attempts = 0;
    while attempts < 100 {
        let random_champion = &all_champions[rand::random::<usize>() % all_champions.len()];
        if !current_names.contains(&random_champion.name) {
            team.champions[replace_idx] = random_champion.clone();
            break;
        }
        attempts += 1;
    }
}
