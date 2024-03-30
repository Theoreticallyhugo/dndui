// use std::error;

pub enum Advantage {
    No,
    Advantage,
    Disadvantage,
}

pub struct Character {
    name: String,
    race: String,
    class: String,
    level: u8,
    /// stats (ability_score, proficiency, modifier)
    /// the modifier is a calculated value that doesnt need to be saved to a character file.
    ///
    strength: (u8, bool, i8),
    dexterity: (u8, bool, i8),
    constitution: (u8, bool, i8),
    intelligence: (u8, bool, i8),
    wisdom: (u8, bool, i8),
    charisma: (u8, bool, i8),
    /// skills (proficiency, modifier, advantage)
    /// modifier and advantage are calculated values that dont need to be saved to a character file.
    ///
    acrobatics: (bool, i8, Advantage),
    animal_handling: (bool, i8, Advantage),
    arcana: (bool, i8, Advantage),
    athletics: (bool, i8, Advantage),
    deception: (bool, i8, Advantage),
    history: (bool, i8, Advantage),
    insight: (bool, i8, Advantage),
    intimidation: (bool, i8, Advantage),
    investigation: (bool, i8, Advantage),
    medicine: (bool, i8, Advantage),
    nature: (bool, i8, Advantage),
    perception: (bool, i8, Advantage),
    performance: (bool, i8, Advantage),
    persuasion: (bool, i8, Advantage),
    religion: (bool, i8, Advantage),
    sleight_of_hand: (bool, i8, Advantage),
    stealth: (bool, i8, Advantage),
    survival: (bool, i8, Advantage),

    /// stuff
    ///
    /// proficiency
    proficiency: u8,

    walking_speed: u8,

    inspiration: bool,
    /// HP 
    ///
    /// the characters current hp
    current_hp: i32,
    /// the characters max hp
    max_hp: u32,
    /// the characters temp hp
    temp_hp: i32,

}

impl Default for Character {
    fn default() -> Self {
        Self {
            name: "name".to_string(),
            race: "race".to_string(),
            class: "class".to_string(),
            level: 0,
            // stats
            strength: (0, false, 0),
            dexterity: (0, false, 0),
            constitution: (0, false, 0),
            intelligence: (0, false, 0),
            wisdom: (0, false, 0),
            charisma: (0, false, 0),
            // skills
            acrobatics: (false, 0, Advantage::No),
            animal_handling: (false, 0, Advantage::No),
            arcana: (false, 0, Advantage::No),
            athletics: (false, 0, Advantage::No),
            deception: (false, 0, Advantage::No),
            history: (false, 0, Advantage::No),
            insight: (false, 0, Advantage::No),
            intimidation: (false, 0, Advantage::No),
            investigation: (false, 0, Advantage::No),
            medicine: (false, 0, Advantage::No),
            nature: (false, 0, Advantage::No),
            perception: (false, 0, Advantage::No),
            performance: (false, 0, Advantage::No),
            persuasion: (false, 0, Advantage::No),
            religion: (false, 0, Advantage::No),
            sleight_of_hand: (false, 0, Advantage::No),
            stealth: (false, 0, Advantage::No),
            survival: (false, 0, Advantage::No),

            proficiency: 0,

            walking_speed: 0,

            inspiration: false,
            // hp section
            current_hp: 0,
            max_hp: 0,
            temp_hp: 0,
        }
    }
}

impl Character {
    /// Constructs a new instance of [`Character`].
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_character(&mut self) {
        // eventually this should be replaced by a json loading script
        self.name = "name".to_string();
        self.race = "dragonborn".to_string();
        self.class = "paladin".to_string();
        self.level = 3;
        
        self.strength = (16, false, 0);
        self.dexterity = (8, false, 0);
        self.constitution = (14, false, 0);
        self.intelligence = (8, false, 0);
        self.wisdom = (12, true, 0);
        self.charisma = (16, true, 0);
        // skills
        self.acrobatics.0 = false;
        self.animal_handling.0 = true;
        self.arcana.0 = false;
        self.athletics.0 = true;
        self.deception.0 = false;
        self.history.0 = false;
        self.insight.0 = false;
        self.intimidation.0 = true;
        self.investigation.0 = false;
        self.medicine.0 = false;
        self.nature.0 = false;
        self.perception.0 = false;
        self.performance.0 = false;
        self.persuasion.0 = true;
        self.religion.0 = false;
        self.sleight_of_hand.0 = false;
        self.stealth.0 = false;
        self.survival.0 = false;

        self.proficiency = 2;
        
        self.walking_speed = 30;

        self.inspiration = false;
        // hp section
        self.current_hp = 28;
        self.max_hp = 28;
        self.temp_hp = 0;

        // make sure to calculate everything after loading
        self.recalculate();
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_race(&self) -> &String {
        &self.race
    }
    pub fn get_class(&self) -> &String {
        &self.class
    }
    pub fn get_level(&self) -> u8 {
        self.level
    }

    // stats
    
    /// returns: score, proficiency, modifier
    pub fn get_strength(&self) -> (u8, bool, i8) {
        self.strength
    }
    /// returns: score, proficiency, modifier
    pub fn get_dexterity(&self) -> (u8, bool, i8) {
        self.dexterity
    }
    /// returns: score, proficiency, modifier
    pub fn get_constitution(&self) -> (u8, bool, i8) {
        self.constitution
    }
    /// returns: score, proficiency, modifier
    pub fn get_intelligence(&self) -> (u8, bool, i8) {
        self.intelligence
    }
    /// returns: score, proficiency, modifier
    pub fn get_wisdom(&self) -> (u8, bool, i8) {
        self.wisdom
    }
    /// returns: score, proficiency, modifier
    pub fn get_charisma(&self) -> (u8, bool, i8) {
        self.charisma
    }

    // PROFICIENCY SKILLS
    pub fn get_prof_acrobatics(&self) -> &(bool, i8, Advantage) {
        &self.acrobatics
    }
    pub fn get_prof_animal_handling(&self) -> &(bool, i8, Advantage) {
        &self.animal_handling
    }
    pub fn get_prof_arcana(&self) -> &(bool, i8, Advantage) {
        &self.arcana
    }
    pub fn get_prof_athletics(&self) -> &(bool, i8, Advantage) {
        &self.athletics
    }
    pub fn get_prof_deception(&self) -> &(bool, i8, Advantage) {
        &self.deception
    }
    pub fn get_prof_history(&self) -> &(bool, i8, Advantage) {
        &self.history
    }
    pub fn get_prof_insight(&self) -> &(bool, i8, Advantage) {
        &self.insight
    }
    pub fn get_prof_intimidation(&self) -> &(bool, i8, Advantage) {
        &self.intimidation
    }
    pub fn get_prof_investigation(&self) -> &(bool, i8, Advantage) {
        &self.investigation
    }
    pub fn get_prof_medicine(&self) -> &(bool, i8, Advantage) {
        &self.medicine
    }
    pub fn get_prof_nature(&self) -> &(bool, i8, Advantage) {
        &self.nature
    }
    pub fn get_prof_perception(&self) -> &(bool, i8, Advantage) {
        &self.perception
    }
    pub fn get_prof_performance(&self) -> &(bool, i8, Advantage) {
        &self.performance
    }
    pub fn get_prof_persuasion(&self) -> &(bool, i8, Advantage) {
        &self.persuasion
    }
    pub fn get_prof_religion(&self) -> &(bool, i8, Advantage) {
        &self.religion
    }
    pub fn get_prof_sleight_of_hand(&self) -> &(bool, i8, Advantage) {
        &self.sleight_of_hand
    }
    pub fn get_prof_stealth(&self) -> &(bool, i8, Advantage) {
        &self.stealth
    }
    pub fn get_prof_survival(&self) -> &(bool, i8, Advantage) {
        &self.survival
    }

    pub fn calculate_modifier(&self, skill: u8, proficient: bool) -> i8 {
        let skill: i8 = skill.try_into().unwrap();

        if proficient {
            let prof: i8 = self.get_proficiency().try_into().unwrap();
            ((skill - 10) / 2) + prof
        } else {
            (skill - 10) / 2 
        }
    }

    pub fn get_proficiency(&self) -> u8 {
        self.proficiency
    }

    pub fn get_walking_speed(&self) -> u8 {
        self.walking_speed
    }

    pub fn get_inspiration(&self) -> bool {
        self.inspiration
    }

    pub fn set_inspiration(&mut self, new_value: bool) {
        self.inspiration = new_value;
    }

    pub fn get_current_hp(&self) -> i32 {
        self.current_hp
    }

    pub fn get_max_hp(&self) -> u32 {
        self.max_hp
    }

    pub fn get_temp_hp(&self) -> i32 {
        self.temp_hp
    }

    pub fn get_initiative(&self) -> i8 {
        // TODO: needs to be calculated properly!
        -1
    }

    pub fn get_armor_class(&self) -> u8 {
        // TODO: needs to be calculated properly!
        19
    }

    pub fn get_defenses(&self) -> String {
        // TODO: needs to be saved properly!
        "r fire\ni disease".to_string()
    }

    pub fn get_conditions(&self) -> String {
        // TODO: needs to be saved properly!
        "add active conditions".to_string()
    }

    pub fn add_healing(&mut self, healing: i32) {
        self.current_hp += healing;
    }

    pub fn add_temp_healing(&mut self, healing: i32) {
        self.temp_hp += healing;
    }

    pub fn recalculate(&mut self) {
        // ability modifiers
        self.strength.2 = self.calculate_modifier(self.strength.0, self.strength.1);
        self.dexterity.2 = self.calculate_modifier(self.dexterity.0, self.dexterity.1);
        self.constitution.2 = self.calculate_modifier(self.constitution.0, self.constitution.1);
        self.intelligence.2 = self.calculate_modifier(self.intelligence.0, self.intelligence.1);
        self.wisdom.2 = self.calculate_modifier(self.wisdom.0, self.wisdom.1);
        self.charisma.2 = self.calculate_modifier(self.charisma.0, self.charisma.1);
        // skill modifiers 
        self.acrobatics.1 = self.calculate_modifier(self.dexterity.0, self.acrobatics.0);

        // armor class 
        // initiative 
        // skill advantages (i.e. when equipping heavy armour)
    }
}
