import { EcosystemSimulator, SpeciesType } from './ecosystem-simulator';

const simulator = new EcosystemSimulator();
console.log('Initial counts:', simulator.getPopulationCounts());

// Try removing grass
const grassCount = simulator.getPopulationCounts()[SpeciesType.Grass];
console.log('Grass count before removal:', grassCount);
simulator.removeSpecies(SpeciesType.Grass, grassCount);
console.log('Grass count after removal:', simulator.getPopulationCounts()[SpeciesType.Grass]);
console.log('Is game lost?', simulator.isGameLost());
