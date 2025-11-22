//! Combat system for turn-based battles

use crate::entity::Entity;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct CombatResult {
    pub attacker_name: String,
    pub defender_name: String,
    pub damage: i32,
    pub defender_killed: bool,
}

/// Execute an attack from attacker to defender
pub fn attack(
    attacker: &Entity,
    defender: &mut Entity,
    rng: &mut impl Rng,
) -> Option<CombatResult> {
    if !attacker.is_alive() || !defender.is_alive() {
        return None;
    }

    let _attacker_stats = attacker.stats.as_ref()?;

    // Get values we need before borrowing defender mutably
    let defender_name = defender.name();
    let damage_reduction = defender.total_defense();

    let defender_stats = defender.stats.as_mut()?;

    // Calculate damage: attack + random(0-5) - defense
    let base_damage = attacker.total_attack() + rng.gen_range(0..6);
    let final_damage = (base_damage - damage_reduction).max(1); // Always at least 1 damage

    defender_stats.take_damage(final_damage);
    let defender_killed = !defender_stats.is_alive();

    Some(CombatResult {
        attacker_name: attacker.name(),
        defender_name,
        damage: final_damage,
        defender_killed,
    })
}

/// Calculate if an attack would hit (for display purposes)
#[allow(dead_code)]
pub fn would_hit(attacker: &Entity, defender: &Entity, rng: &mut impl Rng) -> bool {
    let attack_roll = rng.gen_range(1..21); // D20
    let hit_chance = 10 + (attacker.total_attack() - defender.total_defense());

    attack_roll >= hit_chance.clamp(2, 19) // Always 10% miss, 10% hit
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::{EnemyType, Entity};
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn test_attack_deals_damage() {
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let attacker = Entity::new_player(0, 0);
        let mut defender = Entity::new_enemy(1, 1, EnemyType::Goblin);

        let initial_hp = defender.stats.as_ref().unwrap().hp;

        let result = attack(&attacker, &mut defender, &mut rng);
        assert!(result.is_some());

        let result = result.unwrap();
        assert!(result.damage > 0);

        let final_hp = defender.stats.as_ref().unwrap().hp;
        assert!(final_hp < initial_hp);
    }

    #[test]
    fn test_attack_can_kill() {
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let attacker = Entity::new_player(0, 0);
        let mut defender = Entity::new_enemy(1, 1, EnemyType::Goblin);

        // Attack until dead
        for _ in 0..100 {
            if let Some(result) = attack(&attacker, &mut defender, &mut rng) {
                if result.defender_killed {
                    assert!(!defender.is_alive());
                    return;
                }
            } else {
                break;
            }
        }
    }

    #[test]
    fn test_dead_entity_cannot_attack() {
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let mut attacker = Entity::new_player(0, 0);
        attacker.stats.as_mut().unwrap().hp = 0;

        let mut defender = Entity::new_enemy(1, 1, EnemyType::Goblin);

        let result = attack(&attacker, &mut defender, &mut rng);
        assert!(result.is_none());
    }

    #[test]
    fn test_attack_result_contains_names() {
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let attacker = Entity::new_player(0, 0);
        let mut defender = Entity::new_enemy(1, 1, EnemyType::Goblin);

        let result = attack(&attacker, &mut defender, &mut rng).unwrap();

        assert!(!result.attacker_name.is_empty());
        assert!(!result.defender_name.is_empty());
        assert!(result.damage > 0);
    }

    #[test]
    fn test_minimum_damage() {
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let attacker = Entity::new_enemy(0, 0, EnemyType::Goblin);
        let mut defender = Entity::new_enemy(1, 1, EnemyType::Dragon);

        // Even weak attacker vs strong defender should deal at least 1 damage
        let result = attack(&attacker, &mut defender, &mut rng).unwrap();
        assert!(result.damage >= 1);
    }

    #[test]
    fn test_equipment_affects_combat() {
        use crate::items::Item;
        let mut rng = ChaCha8Rng::seed_from_u64(42);

        let mut attacker = Entity::new_player(0, 0);
        attacker.equipped_weapon = Some(Item::new_weapon("Sword", 10));

        let mut defender = Entity::new_enemy(1, 1, EnemyType::Goblin);

        let damage_with_weapon = {
            let result = attack(&attacker, &mut defender, &mut rng).unwrap();
            result.damage
        };

        // Reset defender
        defender = Entity::new_enemy(1, 1, EnemyType::Goblin);
        attacker.equipped_weapon = None;

        let damage_without_weapon = {
            let result = attack(&attacker, &mut defender, &mut rng).unwrap();
            result.damage
        };

        // Weapon should increase damage
        assert!(damage_with_weapon > damage_without_weapon);
    }
}
