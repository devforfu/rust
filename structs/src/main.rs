mod inventory;
use crate::inventory::drop::DropGenerator;

fn main() {
    let drop = DropGenerator::new();
    let loot = drop.open_loot_chest(7);

    println!(
        "You collected loot from the fallen enemies: {} item(s)",
        loot.len()
    );

    for (i, weapon) in loot.iter().enumerate() {
        println!("({}) {}", i + 1, weapon);
    }
}
