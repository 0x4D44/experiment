/**
 * Ecosystem Balance Simulator
 * A nature simulation game with predator-prey relationships,
 * population dynamics, and environmental management.
 */

// ============================================================================
// TYPES AND INTERFACES
// ============================================================================

enum SpeciesType {
  Grass = 'grass',
  Rabbit = 'rabbit',
  Wolf = 'wolf',
  Bird = 'bird',
}

enum WeatherType {
  Normal = 'normal',
  Drought = 'drought',
  Abundant = 'abundant',
  Storm = 'storm',
}

interface Individual {
  id: string;
  species: SpeciesType;
  energy: number;
  age: number;
}

interface PopulationRecord {
  cycle: number;
  grass: number;
  rabbits: number;
  wolves: number;
  birds: number;
  weather: WeatherType;
  waterLevel: number;
}

interface GameState {
  cycle: number;
  isRunning: boolean;
  isPaused: boolean;
  populations: Record<SpeciesType, Individual[]>;
  populationHistory: PopulationRecord[];
  weather: WeatherType;
  waterLevel: number; // 0-100
  balanceCheckCount: number; // Cycles with good balance
}

interface SpeciesConfig {
  type: SpeciesType;
  initialPopulation: number;
  energyGain: number;
  energyCost: number;
  reproductionThreshold: number;
  reproductionRate: number;
  maxAge: number;
  color: string;
}

// ============================================================================
// SPECIES CONFIGURATION
// ============================================================================

const SPECIES_CONFIGS: Record<SpeciesType, SpeciesConfig> = {
  [SpeciesType.Grass]: {
    type: SpeciesType.Grass,
    initialPopulation: 500,
    energyGain: 0, // Plants grow automatically
    energyCost: 0,
    reproductionThreshold: 0,
    reproductionRate: 0.05, // 5% of grass grows each cycle
    maxAge: 100,
    color: '#22aa22',
  },
  [SpeciesType.Rabbit]: {
    type: SpeciesType.Rabbit,
    initialPopulation: 50,
    energyGain: 10, // Gain 10 energy from eating grass
    energyCost: 1, // Lose 1 energy per cycle
    reproductionThreshold: 15, // Need 15+ energy to reproduce
    reproductionRate: 0.3, // 30% chance to reproduce if threshold met
    maxAge: 50,
    color: '#ff8844',
  },
  [SpeciesType.Wolf]: {
    type: SpeciesType.Wolf,
    initialPopulation: 5,
    energyGain: 30, // Gain 30 energy from eating rabbit
    energyCost: 2, // Lose 2 energy per cycle
    reproductionThreshold: 40, // Need 40+ energy to reproduce
    reproductionRate: 0.2, // 20% chance to reproduce if threshold met
    maxAge: 80,
    color: '#aa2222',
  },
  [SpeciesType.Bird]: {
    type: SpeciesType.Bird,
    initialPopulation: 30,
    energyGain: 5, // Gain 5 energy from eating grass seeds or small rabbits
    energyCost: 0.8, // Lose 0.8 energy per cycle
    reproductionThreshold: 12, // Need 12+ energy to reproduce
    reproductionRate: 0.35, // 35% chance to reproduce
    maxAge: 40,
    color: '#4488ff',
  },
};

const BALANCE_REQUIREMENTS = {
  grassMin: 100,
  grassMax: 1000,
  rabbitMin: 10,
  rabbitMax: 200,
  wolfMin: 2,
  wolfMax: 30,
  birdMin: 5,
  birdMax: 100,
};

const WIN_BALANCE_CYCLES = 50; // Must maintain balance for 50 cycles to win

// ============================================================================
// ECOSYSTEM SIMULATOR CLASS
// ============================================================================

export class EcosystemSimulator {
  private gameState: GameState;
  private idCounter: number = 0;

  constructor() {
    this.gameState = this.initializeGame();
  }

  private initializeGame(): GameState {
    const populations: Record<SpeciesType, Individual[]> = {
      [SpeciesType.Grass]: [],
      [SpeciesType.Rabbit]: [],
      [SpeciesType.Wolf]: [],
      [SpeciesType.Bird]: [],
    };

    // Initialize populations
    Object.values(SpeciesType).forEach((species) => {
      const config = SPECIES_CONFIGS[species];
      for (let i = 0; i < config.initialPopulation; i++) {
        populations[species].push(this.createIndividual(species));
      }
    });

    const initialRecord: PopulationRecord = {
      cycle: 0,
      grass: populations[SpeciesType.Grass].length,
      rabbits: populations[SpeciesType.Rabbit].length,
      wolves: populations[SpeciesType.Wolf].length,
      birds: populations[SpeciesType.Bird].length,
      weather: WeatherType.Normal,
      waterLevel: 80,
    };

    return {
      cycle: 0,
      isRunning: false,
      isPaused: false,
      populations,
      populationHistory: [initialRecord],
      weather: WeatherType.Normal,
      waterLevel: 80,
      balanceCheckCount: 0,
    };
  }

  private createIndividual(species: SpeciesType): Individual {
    const config = SPECIES_CONFIGS[species];
    return {
      id: `${species}-${this.idCounter++}`,
      species,
      energy: config.energyGain > 0 ? config.energyGain : 50,
      age: 0,
    };
  }

  // ========================================================================
  // SIMULATION CYCLE
  // ========================================================================

  simulateCycle(): void {
    if (!this.gameState.isRunning || this.gameState.isPaused) {
      return;
    }

    this.gameState.cycle++;

    // Update weather
    this.updateWeather();

    // Apply environmental effects
    this.applyEnvironmentalEffects();

    // Process each species
    this.processGrass();
    this.processPredators(SpeciesType.Bird);
    this.processPredators(SpeciesType.Rabbit);
    this.processPredators(SpeciesType.Wolf);

    // Update balance check
    this.updateBalanceCheck();

    // Record history
    this.recordPopulation();
  }

  private updateWeather(): void {
    // 10% chance to change weather
    const rand = Math.random();
    if (rand < 0.05) {
      const weatherOptions = Object.values(WeatherType);
      this.gameState.weather =
        weatherOptions[Math.floor(Math.random() * weatherOptions.length)];
    }

    // Weather affects water level
    switch (this.gameState.weather) {
      case WeatherType.Drought:
        this.gameState.waterLevel = Math.max(20, this.gameState.waterLevel - 5);
        break;
      case WeatherType.Abundant:
        this.gameState.waterLevel = Math.min(100, this.gameState.waterLevel + 5);
        break;
      case WeatherType.Storm:
        this.gameState.waterLevel = Math.max(0, this.gameState.waterLevel - 10);
        break;
      case WeatherType.Normal:
        this.gameState.waterLevel = Math.min(100, this.gameState.waterLevel + 1);
        break;
    }
  }

  private applyEnvironmentalEffects(): void {
    // Water affects all species
    const waterMultiplier = this.gameState.waterLevel / 100;

    // Apply stress to species based on water level
    if (this.gameState.waterLevel < 30) {
      // Severe drought - increased death rate
      this.applyDeathStress(0.3);
    } else if (this.gameState.waterLevel < 50) {
      // Moderate drought
      this.applyDeathStress(0.1);
    }
  }

  private applyDeathStress(stressRate: number): void {
    Object.values(SpeciesType).forEach((species) => {
      const individuals = this.gameState.populations[species];
      for (let i = individuals.length - 1; i >= 0; i--) {
        if (Math.random() < stressRate) {
          individuals.splice(i, 1);
        }
      }
    });
  }

  private processGrass(): void {
    const grassPop = this.gameState.populations[SpeciesType.Grass];
    const config = SPECIES_CONFIGS[SpeciesType.Grass];
    const waterMultiplier = Math.max(0.3, this.gameState.waterLevel / 100);

    // Grass grows based on water availability
    const growthAmount = Math.floor(grassPop.length * config.reproductionRate * waterMultiplier);
    for (let i = 0; i < growthAmount; i++) {
      grassPop.push(this.createIndividual(SpeciesType.Grass));
    }

    // Grass ages
    grassPop.forEach((grass) => {
      grass.age++;
      if (grass.age > config.maxAge) {
        grass.energy = -1; // Mark for removal
      }
    });

    // Remove dead grass
    this.removeDeadIndividuals(SpeciesType.Grass);
  }

  private processPredators(species: SpeciesType): void {
    const config = SPECIES_CONFIGS[species];
    const individuals = this.gameState.populations[species];

    // Age and metabolism
    individuals.forEach((individual) => {
      individual.age++;
      individual.energy -= config.energyCost;

      // Death from age or starvation
      if (individual.age > config.maxAge || individual.energy <= 0) {
        individual.energy = -1; // Mark for removal
        return;
      }

      // Eating
      this.hunt(individual);

      // Reproduction
      if (individual.energy > config.reproductionThreshold) {
        if (Math.random() < config.reproductionRate) {
          const offspring = this.createIndividual(species);
          offspring.energy = Math.floor(config.energyGain * 0.5);
          individuals.push(offspring);
          individual.energy -= Math.floor(config.energyGain * 0.5);
        }
      }
    });

    this.removeDeadIndividuals(species);
  }

  private hunt(individual: Individual): void {
    const config = SPECIES_CONFIGS[individual.species];

    switch (individual.species) {
      case SpeciesType.Rabbit:
        this.huntGrass(individual, config);
        break;
      case SpeciesType.Bird:
        // Birds eat grass and small animals
        if (Math.random() > 0.5) {
          this.huntGrass(individual, config);
        } else {
          this.huntSmallAnimals(individual, config);
        }
        break;
      case SpeciesType.Wolf:
        this.huntRabbits(individual, config);
        break;
    }
  }

  private huntGrass(individual: Individual, config: SpeciesConfig): void {
    const grassPop = this.gameState.populations[SpeciesType.Grass];
    if (grassPop.length > 0) {
      const randomIndex = Math.floor(Math.random() * grassPop.length);
      grassPop.splice(randomIndex, 1);
      individual.energy += config.energyGain;
    }
  }

  private huntSmallAnimals(individual: Individual, config: SpeciesConfig): void {
    const rabbitPop = this.gameState.populations[SpeciesType.Rabbit];
    if (rabbitPop.length > 0) {
      // Birds have 50% success rate
      if (Math.random() < 0.5) {
        const randomIndex = Math.floor(Math.random() * rabbitPop.length);
        rabbitPop.splice(randomIndex, 1);
        individual.energy += config.energyGain * 1.5;
      }
    }
  }

  private huntRabbits(individual: Individual, config: SpeciesConfig): void {
    const rabbitPop = this.gameState.populations[SpeciesType.Rabbit];
    if (rabbitPop.length > 0 && Math.random() < 0.7) {
      // 70% hunting success for wolves
      const randomIndex = Math.floor(Math.random() * rabbitPop.length);
      rabbitPop.splice(randomIndex, 1);
      individual.energy += config.energyGain;
    }
  }

  private removeDeadIndividuals(species: SpeciesType): void {
    const individuals = this.gameState.populations[species];
    this.gameState.populations[species] = individuals.filter((ind) => ind.energy > 0);
  }

  private updateBalanceCheck(): void {
    const balance = this.isBalanced();
    if (balance) {
      this.gameState.balanceCheckCount++;
    } else {
      this.gameState.balanceCheckCount = 0;
    }
  }

  private isBalanced(): boolean {
    const grassCount = this.gameState.populations[SpeciesType.Grass].length;
    const rabbitCount = this.gameState.populations[SpeciesType.Rabbit].length;
    const wolfCount = this.gameState.populations[SpeciesType.Wolf].length;
    const birdCount = this.gameState.populations[SpeciesType.Bird].length;

    return (
      grassCount >= BALANCE_REQUIREMENTS.grassMin &&
      grassCount <= BALANCE_REQUIREMENTS.grassMax &&
      rabbitCount >= BALANCE_REQUIREMENTS.rabbitMin &&
      rabbitCount <= BALANCE_REQUIREMENTS.rabbitMax &&
      wolfCount >= BALANCE_REQUIREMENTS.wolfMin &&
      wolfCount <= BALANCE_REQUIREMENTS.wolfMax &&
      birdCount >= BALANCE_REQUIREMENTS.birdMin &&
      birdCount <= BALANCE_REQUIREMENTS.birdMax
    );
  }

  private recordPopulation(): void {
    const record: PopulationRecord = {
      cycle: this.gameState.cycle,
      grass: this.gameState.populations[SpeciesType.Grass].length,
      rabbits: this.gameState.populations[SpeciesType.Rabbit].length,
      wolves: this.gameState.populations[SpeciesType.Wolf].length,
      birds: this.gameState.populations[SpeciesType.Bird].length,
      weather: this.gameState.weather,
      waterLevel: this.gameState.waterLevel,
    };
    this.gameState.populationHistory.push(record);
  }

  // ========================================================================
  // PLAYER ACTIONS
  // ========================================================================

  addSpecies(species: SpeciesType, count: number): void {
    const population = this.gameState.populations[species];
    for (let i = 0; i < count; i++) {
      population.push(this.createIndividual(species));
    }
  }

  removeSpecies(species: SpeciesType, count: number): void {
    const population = this.gameState.populations[species];
    for (let i = 0; i < Math.min(count, population.length); i++) {
      population.pop();
    }
  }

  setWeather(weather: WeatherType): void {
    this.gameState.weather = weather;
  }

  startGame(): void {
    this.gameState.isRunning = true;
    this.gameState.isPaused = false;
  }

  pauseGame(): void {
    this.gameState.isPaused = true;
  }

  resumeGame(): void {
    this.gameState.isPaused = false;
  }

  resetGame(): void {
    this.gameState = this.initializeGame();
    this.idCounter = 0;
  }

  // ========================================================================
  // GETTERS
  // ========================================================================

  getGameState(): GameState {
    return { ...this.gameState };
  }

  getPopulationCounts(): Record<SpeciesType, number> {
    return {
      [SpeciesType.Grass]: this.gameState.populations[SpeciesType.Grass].length,
      [SpeciesType.Rabbit]: this.gameState.populations[SpeciesType.Rabbit].length,
      [SpeciesType.Wolf]: this.gameState.populations[SpeciesType.Wolf].length,
      [SpeciesType.Bird]: this.gameState.populations[SpeciesType.Bird].length,
    };
  }

  isGameWon(): boolean {
    return this.gameState.balanceCheckCount >= WIN_BALANCE_CYCLES;
  }

  isGameLost(): boolean {
    const counts = this.getPopulationCounts();
    // Game is lost if any species goes extinct
    return counts[SpeciesType.Grass] === 0 ||
           counts[SpeciesType.Rabbit] === 0 ||
           counts[SpeciesType.Wolf] === 0 ||
           counts[SpeciesType.Bird] === 0;
  }

  getHistory(): PopulationRecord[] {
    return [...this.gameState.populationHistory];
  }

  getCurrentCycle(): number {
    return this.gameState.cycle;
  }

  getCurrentWeather(): WeatherType {
    return this.gameState.weather;
  }

  getWaterLevel(): number {
    return this.gameState.waterLevel;
  }

  getBalanceProgress(): number {
    return (this.gameState.balanceCheckCount / WIN_BALANCE_CYCLES) * 100;
  }
}

export { SpeciesType, WeatherType, PopulationRecord, GameState };
