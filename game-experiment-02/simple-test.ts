import { EcosystemSimulator, SpeciesType } from './ecosystem-simulator';

// Test 1: Remove all rabbits
console.log('Test 1: Remove all rabbits');
const sim1 = new EcosystemSimulator();
const initialRabbits = sim1.getPopulationCounts()[SpeciesType.Rabbit];
console.log('Initial rabbits:', initialRabbits);
sim1.removeSpecies(SpeciesType.Rabbit, initialRabbits);
console.log('After removal:', sim1.getPopulationCounts()[SpeciesType.Rabbit]);
console.log('');

// Test 2: Try removing by count once
console.log('Test 2: Add 10 rabbits then remove 10');
const sim2 = new EcosystemSimulator();
const before = sim2.getPopulationCounts()[SpeciesType.Rabbit];
sim2.addSpecies(SpeciesType.Rabbit, 10);
const afterAdd = sim2.getPopulationCounts()[SpeciesType.Rabbit];
console.log(`Before: ${before}, After add: ${afterAdd}, Added: ${afterAdd - before}`);
sim2.removeSpecies(SpeciesType.Rabbit, 10);
const afterRemove = sim2.getPopulationCounts()[SpeciesType.Rabbit];
console.log(`After remove 10: ${afterRemove}`);
