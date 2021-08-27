use super::weapon::{MagicSchool, Weapon};
use rand::{rngs::ThreadRng, thread_rng, Rng};
use std::cell::RefCell;

pub struct DropGenerator {
    rng: RefCell<ThreadRng>,
}

impl DropGenerator {
    pub fn new() -> Self {
        DropGenerator {
            rng: RefCell::new(thread_rng()),
        }
    }

    /// Roll a dice with a given number of faces.
    pub fn roll_dice(&self, n_faces: u32) -> u32 {
        self.rng.borrow_mut().gen_range(1..=n_faces)
    }

    /// Returns a randomly chosen weapon with random characteristics.
    ///
    /// Rolls a dice with a number of faces equal to the number of
    /// available weapons and picks one. Then generates random
    /// characteristics for the weapon.
    pub fn drop_weapon(&self) -> Weapon {
        match self.roll_dice(4) {
            1 => Weapon::Pebble,
            2 => Weapon::Sword(self.roll_dice(7)),
            3 => Weapon::Bow {
                range: self.roll_dice(5),
                damage: self.roll_dice(4),
            },
            _ => Weapon::SpellBook {
                school: self.random_magic_school(),
                num_of_spells: 1 + self.roll_dice(5),
            },
        }
    }

    /// Returns a list of weapons from a looted chest.
    ///
    /// The chest's contents and its characteristics generated randomly.
    pub fn open_loot_chest(&self, max_items: u32) -> Vec<Weapon> {
        (1..=self.roll_dice(max_items))
            .map(|_| self.drop_weapon())
            .collect()
    }

    /// Returns a random magic school.
    ///
    /// Rolls a dice with a number of faces equal to the
    /// number of schools and returns one.
    fn random_magic_school(&self) -> MagicSchool {
        match self.roll_dice(4) {
            1 => MagicSchool::Water,
            2 => MagicSchool::Wind,
            3 => MagicSchool::Fire,
            _ => MagicSchool::Death,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rolling_dice() {
        let gen = DropGenerator::new();

        let result = gen.roll_dice(6);

        assert!((1..=6).contains(&result));
    }

    #[test]
    fn test_dropping_random_weapon() {
        let gen = DropGenerator::new();

        let weapon_desc = gen.drop_weapon().to_string();

        assert!(vec!["pebble", "bow", "sword", "book"]
            .iter()
            .any(|name| weapon_desc.contains(name)))
    }

    #[test]
    fn test_opening_loot_chest() {
        let gen = DropGenerator::new();

        let items = gen.open_loot_chest(5);

        assert!(items.len() <= 5)
    }
}
