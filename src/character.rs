// use std::error;

pub struct Character {
    /// stats 
    ///
    strength: u8,
    dexterity: u8,
    constitution: u8,
    intelligence: u8,
    wisdom: u8,
    charisma: u8,
    /// skills 
    ///

    /// stuff
    ///
    /// proficiency
    proficiency: u8,

    /// HP 
    ///
    /// the characters current hp
    current_hp: i8,
    /// the characters max hp
    max_hp: u8,
    /// the characters temp hp
    temp_hp: i8,
}

impl Default for Character {
    fn default() -> Self {
        Self {
            // stats
            strength: 16,
            dexterity: 8,
            constitution: 14,
            intelligence: 8,
            wisdom: 12,
            charisma: 16,
            // skills

            proficiency: 2,
            // hp section
            current_hp: 28,
            max_hp: 28,
            temp_hp: 0,
        }
    }
}

impl Character {
    /// Constructs a new instance of [`Character`].
    pub fn new() -> Self {
        Self::default()
    }

    /// stats
    pub fn get_strength(&self) -> u8 {
        self.strength
    }
    pub fn get_dexterity(&self) -> u8 {
        self.dexterity
    }
    pub fn get_constitution(&self) -> u8 {
        self.constitution
    }
    pub fn get_intelligence(&self) -> u8 {
        self.intelligence
    }
    pub fn get_wisdom(&self) -> u8 {
        self.wisdom
    }
    pub fn get_charisma(&self) -> u8 {
        self.charisma
    }

    pub fn get_proficiency(&self) -> u8 {
        self.proficiency
    }

    pub fn get_current_hp(&self) -> i8 {
        self.current_hp
    }

    pub fn get_max_hp(&self) -> u8 {
        self.max_hp
    }

    pub fn get_temp_hp(&self) -> i8 {
        self.temp_hp
    }

    pub fn add_damage(&mut self, damage: i8) {
        self.current_hp += damage;
    }
}
