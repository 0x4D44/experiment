use rand::Rng;

/// Different types of items
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItemType {
    HealthPotion,
    ManaPotion,
    Sword,
    Shield,
    Armor,
}

/// An item that can be picked up and used
#[derive(Debug, Clone)]
pub struct Item {
    pub name: String,
    pub symbol: char,
    pub item_type: ItemType,
    pub value: i32,
    pub consumable: bool,
}

impl Item {
    pub fn new_health_potion() -> Self {
        Item {
            name: "Health Potion".to_string(),
            symbol: '!',
            item_type: ItemType::HealthPotion,
            value: 20,
            consumable: true,
        }
    }

    pub fn new_mana_potion() -> Self {
        Item {
            name: "Mana Potion".to_string(),
            symbol: '~',
            item_type: ItemType::ManaPotion,
            value: 15,
            consumable: true,
        }
    }

    pub fn new_sword() -> Self {
        Item {
            name: "Sword".to_string(),
            symbol: '/',
            item_type: ItemType::Sword,
            value: 5,
            consumable: false,
        }
    }

    pub fn new_shield() -> Self {
        Item {
            name: "Shield".to_string(),
            symbol: '[',
            item_type: ItemType::Shield,
            value: 3,
            consumable: false,
        }
    }

    pub fn new_armor() -> Self {
        Item {
            name: "Armor".to_string(),
            symbol: ']',
            item_type: ItemType::Armor,
            value: 4,
            consumable: false,
        }
    }

    /// Generate a random item based on dungeon depth
    pub fn random_item(_depth: i32) -> Self {
        let mut rng = rand::thread_rng();

        // Higher depth = better items more likely
        let roll = rng.gen_range(0..100);

        if roll < 40 {
            Item::new_health_potion()
        } else if roll < 50 {
            Item::new_mana_potion()
        } else if roll < 70 {
            Item::new_sword()
        } else if roll < 85 {
            Item::new_shield()
        } else {
            Item::new_armor()
        }
    }
}

/// Player's inventory
#[derive(Debug)]
pub struct Inventory {
    pub items: Vec<Item>,
    pub max_size: usize,
    pub equipped_weapon: Option<Item>,
    pub equipped_armor: Option<Item>,
    pub equipped_shield: Option<Item>,
}

impl Inventory {
    pub fn new(max_size: usize) -> Self {
        Inventory {
            items: Vec::new(),
            max_size,
            equipped_weapon: None,
            equipped_armor: None,
            equipped_shield: None,
        }
    }

    /// Try to add an item to inventory
    pub fn add_item(&mut self, item: Item) -> bool {
        if self.items.len() < self.max_size {
            self.items.push(item);
            true
        } else {
            false
        }
    }

    /// Use an item from inventory
    pub fn use_item(&mut self, index: usize) -> Option<Item> {
        if index < self.items.len() {
            let item = self.items.remove(index);
            Some(item)
        } else {
            None
        }
    }

    /// Equip an item
    pub fn equip_item(&mut self, index: usize) -> Option<String> {
        if index >= self.items.len() {
            return None;
        }

        let item = self.items.remove(index);

        match item.item_type {
            ItemType::Sword => {
                if let Some(old_weapon) = self.equipped_weapon.take() {
                    self.items.push(old_weapon);
                }
                self.equipped_weapon = Some(item.clone());
                Some(format!("Equipped {}", item.name))
            }
            ItemType::Armor => {
                if let Some(old_armor) = self.equipped_armor.take() {
                    self.items.push(old_armor);
                }
                self.equipped_armor = Some(item.clone());
                Some(format!("Equipped {}", item.name))
            }
            ItemType::Shield => {
                if let Some(old_shield) = self.equipped_shield.take() {
                    self.items.push(old_shield);
                }
                self.equipped_shield = Some(item.clone());
                Some(format!("Equipped {}", item.name))
            }
            _ => {
                self.items.push(item);
                None
            }
        }
    }

    /// Get total attack bonus from equipment
    pub fn get_attack_bonus(&self) -> i32 {
        self.equipped_weapon.as_ref().map(|w| w.value).unwrap_or(0)
    }

    /// Get total defense bonus from equipment
    pub fn get_defense_bonus(&self) -> i32 {
        let armor_bonus = self.equipped_armor.as_ref().map(|a| a.value).unwrap_or(0);
        let shield_bonus = self.equipped_shield.as_ref().map(|s| s.value).unwrap_or(0);
        armor_bonus + shield_bonus
    }

    /// Check if inventory is full
    pub fn is_full(&self) -> bool {
        self.items.len() >= self.max_size
    }

    /// Get number of items
    pub fn count(&self) -> usize {
        self.items.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_creation() {
        let potion = Item::new_health_potion();
        assert_eq!(potion.item_type, ItemType::HealthPotion);
        assert!(potion.consumable);

        let sword = Item::new_sword();
        assert_eq!(sword.item_type, ItemType::Sword);
        assert!(!sword.consumable);
    }

    #[test]
    fn test_inventory_add() {
        let mut inv = Inventory::new(3);
        assert!(inv.add_item(Item::new_health_potion()));
        assert!(inv.add_item(Item::new_sword()));
        assert!(inv.add_item(Item::new_shield()));
        assert!(!inv.add_item(Item::new_armor())); // Full
        assert!(inv.is_full());
    }

    #[test]
    fn test_inventory_use() {
        let mut inv = Inventory::new(5);
        inv.add_item(Item::new_health_potion());
        inv.add_item(Item::new_sword());

        assert_eq!(inv.count(), 2);
        let item = inv.use_item(0);
        assert!(item.is_some());
        assert_eq!(inv.count(), 1);
    }

    #[test]
    fn test_equipment() {
        let mut inv = Inventory::new(5);
        inv.add_item(Item::new_sword());
        inv.add_item(Item::new_shield());

        assert_eq!(inv.get_attack_bonus(), 0);
        inv.equip_item(0); // Equip sword
        assert_eq!(inv.get_attack_bonus(), 5);

        assert_eq!(inv.get_defense_bonus(), 0);
        inv.equip_item(0); // Equip shield
        assert_eq!(inv.get_defense_bonus(), 3);
    }

    #[test]
    fn test_equipment_swap() {
        let mut inv = Inventory::new(5);
        inv.add_item(Item::new_sword());

        inv.equip_item(0);
        assert!(inv.equipped_weapon.is_some());
        assert_eq!(inv.count(), 0);

        // Add and equip another sword
        inv.add_item(Item::new_sword());
        inv.equip_item(0);

        // Old sword should be back in inventory
        assert_eq!(inv.count(), 1);
    }
}
