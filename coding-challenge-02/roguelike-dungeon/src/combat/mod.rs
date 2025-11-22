use crate::entity::Entity;
use rand::Rng;

/// Result of a combat action
#[derive(Debug, Clone)]
pub struct CombatResult {
    pub attacker_name: String,
    pub defender_name: String,
    pub damage: i32,
    pub defender_died: bool,
    pub was_critical: bool,
}

/// Combat system
pub struct CombatSystem;

impl CombatSystem {
    /// Calculate damage from attacker to defender
    pub fn calculate_damage(attacker: &Entity, defender: &Entity) -> i32 {
        let mut rng = rand::thread_rng();

        // Base damage calculation: attack - defense/2, with randomness
        let base_damage = attacker.attack as f32 - (defender.defense as f32 / 2.0);
        let variance = rng.gen_range(-2..=2);

        let damage = (base_damage + variance as f32).max(1.0) as i32;
        damage
    }

    /// Perform an attack from attacker to defender
    pub fn attack(attacker: &Entity, defender: &mut Entity) -> CombatResult {
        let mut rng = rand::thread_rng();

        // 10% chance of critical hit (double damage)
        let is_critical = rng.gen_range(0..100) < 10;

        let mut damage = Self::calculate_damage(attacker, defender);

        if is_critical {
            damage *= 2;
        }

        let defender_died = defender.take_damage(damage);

        CombatResult {
            attacker_name: attacker.name.clone(),
            defender_name: defender.name.clone(),
            damage,
            defender_died,
            was_critical: is_critical,
        }
    }

    /// Check if attack hits (for future expansion)
    pub fn does_hit(_attacker: &Entity, _defender: &Entity) -> bool {
        let mut rng = rand::thread_rng();
        // 90% base hit chance
        rng.gen_range(0..100) < 90
    }
}

/// Stores combat messages for display
#[derive(Debug)]
pub struct CombatLog {
    pub messages: Vec<String>,
    max_size: usize,
}

impl CombatLog {
    pub fn new(max_size: usize) -> Self {
        CombatLog {
            messages: Vec::new(),
            max_size,
        }
    }

    pub fn add_message(&mut self, message: String) {
        self.messages.push(message);
        if self.messages.len() > self.max_size {
            self.messages.remove(0);
        }
    }

    pub fn add_combat_result(&mut self, result: &CombatResult) {
        let critical_text = if result.was_critical { " CRITICAL!" } else { "" };
        let message = format!(
            "{} attacks {} for {} damage{}",
            result.attacker_name, result.defender_name, result.damage, critical_text
        );
        self.add_message(message);

        if result.defender_died {
            self.add_message(format!("{} was defeated!", result.defender_name));
        }
    }

    pub fn get_recent(&self, count: usize) -> Vec<String> {
        let start = if self.messages.len() > count {
            self.messages.len() - count
        } else {
            0
        };
        self.messages[start..].to_vec()
    }

    pub fn clear(&mut self) {
        self.messages.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_damage_calculation() {
        let attacker = Entity::new_player(0, 0);
        let defender = Entity::new_enemy(1, 1, 1);

        let damage = CombatSystem::calculate_damage(&attacker, &defender);
        assert!(damage > 0);
    }

    #[test]
    fn test_attack() {
        let attacker = Entity::new_player(0, 0);
        let mut defender = Entity::new_enemy(1, 1, 1);
        let initial_hp = defender.hp;

        let result = CombatSystem::attack(&attacker, &mut defender);

        assert_eq!(result.attacker_name, attacker.name);
        assert_eq!(result.defender_name, defender.name);
        assert!(result.damage > 0);
        assert_eq!(defender.hp, initial_hp - result.damage);
    }

    #[test]
    fn test_combat_until_death() {
        let attacker = Entity::new_player(0, 0);
        let mut defender = Entity::new_enemy(1, 1, 1);

        // Attack until defender dies
        let mut attacks = 0;
        let max_attacks = 100; // Prevent infinite loop

        while defender.is_alive() && attacks < max_attacks {
            let result = CombatSystem::attack(&attacker, &mut defender);
            attacks += 1;

            if !defender.is_alive() {
                assert!(result.defender_died);
            }
        }

        assert!(!defender.is_alive());
        assert_eq!(defender.hp, 0);
    }

    #[test]
    fn test_combat_log() {
        let mut log = CombatLog::new(5);

        log.add_message("Message 1".to_string());
        log.add_message("Message 2".to_string());
        assert_eq!(log.messages.len(), 2);

        // Test max size
        for i in 3..=10 {
            log.add_message(format!("Message {}", i));
        }
        assert_eq!(log.messages.len(), 5);

        let recent = log.get_recent(3);
        assert_eq!(recent.len(), 3);
    }

    #[test]
    fn test_combat_log_result() {
        let mut log = CombatLog::new(10);
        let attacker = Entity::new_player(0, 0);
        let mut defender = Entity::new_enemy(1, 1, 1);

        let result = CombatSystem::attack(&attacker, &mut defender);
        log.add_combat_result(&result);

        assert!(!log.messages.is_empty());
    }
}
