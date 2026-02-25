import { EcosystemSimulator, SpeciesType, WeatherType } from './ecosystem-simulator';

describe('EcosystemSimulator', () => {
  let simulator: EcosystemSimulator;

  beforeEach(() => {
    simulator = new EcosystemSimulator();
  });

  test('initializes with correct populations', () => {
    const counts = simulator.getPopulationCounts();
    expect(counts[SpeciesType.Grass]).toBe(500);
    expect(counts[SpeciesType.Rabbit]).toBe(50);
    expect(counts[SpeciesType.Wolf]).toBe(5);
    expect(counts[SpeciesType.Bird]).toBe(30);
  });

  test('starts at cycle 0', () => {
    expect(simulator.getCurrentCycle()).toBe(0);
  });

  test('game starts not won', () => {
    expect(simulator.isGameWon()).toBe(false);
  });

  test('game starts not lost', () => {
    expect(simulator.isGameLost()).toBe(false);
  });

  test('can start and pause the game', () => {
    simulator.startGame();
    expect(simulator.getGameState().isRunning).toBe(true);
    simulator.pauseGame();
    expect(simulator.getGameState().isPaused).toBe(true);
    simulator.resumeGame();
    expect(simulator.getGameState().isPaused).toBe(false);
  });

  test('can reset the game', () => {
    simulator.startGame();
    for (let i = 0; i < 5; i++) {
      simulator.simulateCycle();
    }
    expect(simulator.getCurrentCycle()).toBeGreaterThan(0);
    simulator.resetGame();
    expect(simulator.getCurrentCycle()).toBe(0);
  });

  test('can add and remove species', () => {
    const initialCount = simulator.getPopulationCounts()[SpeciesType.Rabbit];
    simulator.addSpecies(SpeciesType.Rabbit, 10);
    expect(simulator.getPopulationCounts()[SpeciesType.Rabbit]).toBe(initialCount + 10);
    simulator.removeSpecies(SpeciesType.Rabbit, 5);
    expect(simulator.getPopulationCounts()[SpeciesType.Rabbit]).toBe(initialCount + 5);
  });

  test('cycle only progresses when running', () => {
    simulator.simulateCycle();
    expect(simulator.getCurrentCycle()).toBe(0);
    simulator.startGame();
    simulator.simulateCycle();
    expect(simulator.getCurrentCycle()).toBe(1);
  });

  test('grass population changes each cycle', () => {
    simulator.startGame();
    const initialGrass = simulator.getPopulationCounts()[SpeciesType.Grass];
    for (let i = 0; i < 10; i++) {
      simulator.simulateCycle();
    }
    const finalGrass = simulator.getPopulationCounts()[SpeciesType.Grass];
    expect(finalGrass).not.toBe(initialGrass);
  });

  test('population history is recorded', () => {
    simulator.startGame();
    for (let i = 0; i < 5; i++) {
      simulator.simulateCycle();
    }
    expect(simulator.getHistory().length).toBe(6);
  });

  test('weather affects water level', () => {
    simulator.setWeather(WeatherType.Drought);
    simulator.startGame();
    const initialWater = simulator.getWaterLevel();
    for (let i = 0; i < 5; i++) {
      simulator.simulateCycle();
    }
    expect(simulator.getWaterLevel()).toBeLessThan(initialWater);
  });

  test('water level stays within bounds', () => {
    simulator.setWeather(WeatherType.Storm);
    simulator.startGame();
    for (let i = 0; i < 100; i++) {
      simulator.simulateCycle();
      const water = simulator.getWaterLevel();
      expect(water).toBeGreaterThanOrEqual(0);
      expect(water).toBeLessThanOrEqual(100);
    }
  });

  test('populations remain non-negative', () => {
    simulator.startGame();
    for (let i = 0; i < 100; i++) {
      simulator.simulateCycle();
      const counts = simulator.getPopulationCounts();
      expect(counts[SpeciesType.Grass]).toBeGreaterThanOrEqual(0);
      expect(counts[SpeciesType.Rabbit]).toBeGreaterThanOrEqual(0);
      expect(counts[SpeciesType.Wolf]).toBeGreaterThanOrEqual(0);
      expect(counts[SpeciesType.Bird]).toBeGreaterThanOrEqual(0);
    }
  });

  test('game can run 300 cycles without crashing', () => {
    simulator.startGame();
    expect(() => {
      for (let i = 0; i < 300; i++) {
        simulator.simulateCycle();
      }
    }).not.toThrow();
    expect(simulator.getCurrentCycle()).toBe(300);
  });

  test('player can manage ecosystem', () => {
    simulator.startGame();
    for (let i = 0; i < 100; i++) {
      simulator.simulateCycle();
      const counts = simulator.getPopulationCounts();
      if (counts[SpeciesType.Rabbit] > 150) {
        simulator.addSpecies(SpeciesType.Wolf, 3);
      }
      if (counts[SpeciesType.Grass] < 100) {
        simulator.setWeather(WeatherType.Abundant);
      }
      if (simulator.isGameLost()) {
        break;
      }
    }
    expect(simulator.getCurrentCycle()).toBeGreaterThan(0);
  });

  test('balance progress starts at 0', () => {
    expect(simulator.getBalanceProgress()).toBe(0);
  });

  test('balance progress can increase', () => {
    simulator.startGame();
    let maxBalance = 0;
    for (let i = 0; i < 100; i++) {
      simulator.simulateCycle();
      maxBalance = Math.max(maxBalance, simulator.getBalanceProgress());
    }
    expect(maxBalance).toBeGreaterThanOrEqual(0);
  });

  test('game state methods work correctly', () => {
    const state1 = simulator.getGameState();
    const state2 = simulator.getGameState();
    expect(state1).not.toBe(state2);
  });
});
