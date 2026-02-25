// Quick test without full framework
const testCode = `
import { EcosystemSimulator, SpeciesType } from './ecosystem-simulator';

const sim = new EcosystemSimulator();
console.log('Initial:', sim.getPopulationCounts());

// Try removing grass  
const grassCount = sim.getPopulationCounts()[SpeciesType.Grass];
console.log('Grass count:', grassCount);

sim.removeSpecies(SpeciesType.Grass, grassCount);
const after = sim.getPopulationCounts();
console.log('After removal:', after);
console.log('Grass is now:', after[SpeciesType.Grass]);
console.log('isGameLost:', sim.isGameLost());
`;

console.log('Would need to run TypeScript version...');
