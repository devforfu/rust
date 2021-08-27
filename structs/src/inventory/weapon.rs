use std::fmt;

pub enum Weapon {
    Pebble,
    Sword(u32),
    Bow {
        range: u32,
        damage: u32,
    },
    SpellBook {
        school: MagicSchool,
        num_of_spells: u32,
    },
}

pub enum MagicSchool {
    Water,
    Wind,
    Fire,
    Death,
}

impl fmt::Display for Weapon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Weapon::*;
        write!(
            f,
            "{}",
            match self {
                Pebble => "● A useless pebble".to_string(),
                Sword(damage) => {
                    format!("🗡 A plain sword with damage of {}", damage)
                }
                Bow { range, damage } => {
                    format!(
                        "🏹 A wooden bow with range of {} and damage {}",
                        range, damage,
                    )
                }
                SpellBook {
                    school,
                    num_of_spells,
                } => {
                    format!(
                        "📖 A spell book ({}) with {} {}",
                        school,
                        num_of_spells,
                        if *num_of_spells > 1 {
                            "spells"
                        } else {
                            "spell"
                        }
                    )
                }
            }
        )
    }
}

impl fmt::Display for MagicSchool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use MagicSchool::*;
        write!(
            f,
            "{}",
            match self {
                Water => "💧 The School of Water",
                Wind => "🌪 The School of Wind",
                Fire => "🔥 Pyromancy",
                Death => "💀 Necromancy",
            }
        )
    }
}
