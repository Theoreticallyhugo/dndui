// FIXME: dont actually use this... we need a json solution...


use crate::character_limbs::spells::Spell;

#[derive(Debug, Clone, Default)]
struct ExistingSpells {
    pub bless: Spell,
    pub command: Spell,
}

impl ExistingSpells {
    pub fn new() -> Self {
        Self {
            bless: Spell {
                name: "bless".to_string(),
                class: "Paladin".to_string(),
                level: 1,
                concentration: true,
                ritual: false,
                time: 1,
                time_format: "A".to_string(),
                range: 30,
                hit_dc: 0,
                hit_dc_ability: "".to_string(),
                effect: "buff".to_string(),
                duration: 1,
                duration_format: "m".to_string(),
                components: "v/s/m".to_string(),
                aoe: 0,
                aoe_format: "none".to_string(),
            },
            command: Spell {
                name: "command".to_string(),
                class: "Paladin".to_string(),
                level: 1,
                concentration: false,
                ritual: false,
                time: 1,
                time_format: "A".to_string(),
                range: 60,
                hit_dc: 13,
                hit_dc_ability: "wis".to_string(),
                effect: "prone".to_string(),
                duration: 1,
                duration_format: "rnd".to_string(),
                components: "v".to_string(),
                aoe: 0,
                aoe_format: "none".to_string(),
            }
        }
        // .. Self::default()
    }

    pub fn bless(&self) -> &Spell {
        &self.bless
    }
}
