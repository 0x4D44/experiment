import { EcosystemSimulator, SpeciesType } from './ecosystem-simulator';

const sim = new EcosystemSimulator();
console.log('Initial grass:', sim.getPopulationCounts()[SpeciesType.Grass]);
const count = sim.getPopulationCounts()[SpeciesType.Grass];
console.log('Count to remove:', count);

// Try to remove all grass
for (let i = 0; i < count; i++) {
  sim.removeSpecies(SpeciesType.Grass, 1);
  if (i % 100 === 0) {
    console.log(`After removing ${i+1}, grass count:`, sim.getPopulationCounts()[SpeciesType.Grass]);
  }
}
console.log('Final grass:', sim.getPopulationCounts()[SpeciesType.Grass]);
