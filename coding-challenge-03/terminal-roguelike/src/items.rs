//! Item system for weapons, armor, and consumables

#[derive(Debug, Clone, PartialEq)]
pub enum ItemType {
    Weapon,
    Armor,
    Potion,
}

#[derive(Debug, Clone)]
pub struct Item {
    pub name: String,
    pub item_type: ItemType,
    pub attack_bonus: i32,
    pub defense_bonus: i32,
    pub heal_amount: i32,
}

impl Item {
    pub fn new_weapon(name: &str, attack_bonus: i32) -> Self {
        Self {
            name: name.to_string(),
            item_type: ItemType::Weapon,
            attack_bonus,
            defense_bonus: 0,
            heal_amount: 0,
        }
    }

    pub fn new_armor(name: &str, defense_bonus: i32) -> Self {
        Self {
            name: name.to_string(),
            item_type: ItemType::Armor,
            attack_bonus: 0,
            defense_bonus,
            heal_amount: 0,
        }
    }

    pub fn new_potion(name: &str, heal_amount: i32) -> Self {
        Self {
            name: name.to_string(),
            item_type: ItemType::Potion,
            attack_bonus: 0,
            defense_bonus: 0,
            heal_amount,
        }
    }

    pub fn attack_bonus(&self) -> i32 {
        self.attack_bonus
    }

    pub fn defense_bonus(&self) -> i32 {
        self.defense_bonus
    }

    pub fn symbol(&self) -> char {
        match self.item_type {
            ItemType::Weapon => '/',
            ItemType::Armor => '[',
            ItemType::Potion => '!',
        }
    }

    pub fn color(&self) -> (u8, u8, u8) {
        match self.item_type {
            ItemType::Weapon => (200, 200, 200),
            ItemType::Armor => (139, 69, 19),
            ItemType::Potion => (255, 0, 255),
        }
    }
}

/// Generate random items for dungeon
pub fn generate_random_item(level: i32, rng: &mut impl rand::Rng) -> Item {
    let item_type = rng.gen_range(0..10);

    match item_type {
        0..=3 => {
            // Weapon (40%)
            let weapons = [
                ("Dagger", 3),
                ("Short Sword", 5),
                ("Long Sword", 8),
                ("Battle Axe", 10),
                ("Great Sword", 15),
            ];
            let idx =
                (rng.gen_range(0..weapons.len())).min((level as usize / 2).min(weapons.len() - 1));
            Item::new_weapon(weapons[idx].0, weapons[idx].1 + level / 2)
        }
        4..=6 => {
            // Armor (30%)
            let armors = [
                ("Leather Armor", 2),
                ("Chain Mail", 4),
                ("Plate Armor", 6),
                ("Dragon Scale", 10),
            ];
            let idx =
                (rng.gen_range(0..armors.len())).min((level as usize / 2).min(armors.len() - 1));
            Item::new_armor(armors[idx].0, armors[idx].1 + level / 3)
        }
        _ => {
            // Potion (30%)
            let potions = [
                ("Minor Healing Potion", 20),
                ("Healing Potion", 40),
                ("Greater Healing Potion", 60),
            ];
            let idx = rng
                .gen_range(0..potions.len())
                .min((level as usize / 3).min(potions.len() - 1));
            Item::new_potion(potions[idx].0, potions[idx].1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weapon_creation() {
        let sword = Item::new_weapon("Sword", 10);
        assert_eq!(sword.item_type, ItemType::Weapon);
        assert_eq!(sword.attack_bonus, 10);
        assert_eq!(sword.defense_bonus, 0);
    }

    #[test]
    fn test_armor_creation() {
        let plate = Item::new_armor("Plate Armor", 5);
        assert_eq!(plate.item_type, ItemType::Armor);
        assert_eq!(plate.defense_bonus, 5);
        assert_eq!(plate.attack_bonus, 0);
    }

    #[test]
    fn test_potion_creation() {
        let potion = Item::new_potion("Health Potion", 30);
        assert_eq!(potion.item_type, ItemType::Potion);
        assert_eq!(potion.heal_amount, 30);
    }

    #[test]
    fn test_item_symbols() {
        let weapon = Item::new_weapon("Sword", 5);
        let armor = Item::new_armor("Armor", 3);
        let potion = Item::new_potion("Potion", 20);

        assert_eq!(weapon.symbol(), '/');
        assert_eq!(armor.symbol(), '[');
        assert_eq!(potion.symbol(), '!');
    }

    #[test]
    fn test_random_item_generation() {
        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;

        let mut rng = ChaCha8Rng::seed_from_u64(42);

        let item1 = generate_random_item(1, &mut rng);
        assert!(!item1.name.is_empty());

        let item2 = generate_random_item(5, &mut rng);
        assert!(!item2.name.is_empty());
    }
}
