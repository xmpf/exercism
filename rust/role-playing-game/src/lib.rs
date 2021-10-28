// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

// Note: `cargo test -- --ignored`

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health <= 0 {
            let mana: Option<u32> = if self.level >= 10 {Some(100)} else {None};
            return Some(Player { health: 100, mana: mana, level: self.level })
        }
        None
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            // Sufficient mana
            Some(n) if n >= mana_cost => {
                self.mana = Some(n - mana_cost);
                return 2 * mana_cost;
            },
            // Insufficient mana
            Some(_) => 0,
            // No mana-pool
            None => {
                self.health = match mana_cost {
                    n if n <= self.health => self.health - n,
                    _ => 0,
                };
                return 0;
            },
        }
    }
}
