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
    /// stats (ability_score, proficiency)
    ///
    strength: (u8, bool),
    dexterity: (u8, bool),
    constitution: (u8, bool),
    intelligence: (u8, bool),
    wisdom: (u8, bool),
    charisma: (u8, bool),
    /// skills 
    ///
    proficiency_acrobatics: bool,
    proficiency_animal_handling: bool,
    proficiency_arcana: bool,
    proficiency_athletics: bool,
    proficiency_deception: bool,
    proficiency_history: bool,
    proficiency_insight: bool,
    proficiency_intimidation: bool,
    proficiency_investigation: bool,
    proficiency_medicine: bool,
    proficiency_nature: bool,
    proficiency_perception: bool,
    proficiency_performance: bool,
    proficiency_persuasion: bool,
    proficiency_religion: bool,
    proficiency_sleight_of_hand: bool,
    proficiency_stealth: bool,
    proficiency_survival: bool,

    // advantage_acrobatics: Advantage,
    // advantage_animal_handling: Advantage,
    // advantage_arcana: Advantage,
    // advantage_athletics: Advantage,
    // advantage_deception: Advantage,
    // advantage_history: Advantage,
    // advantage_insight: Advantage,
    // advantage_intimidation: Advantage,
    // advantage_investigation: Advantage,
    // advantage_medicine: Advantage,
    // advantage_nature: Advantage,
    // advantage_perception: Advantage,
    // advantage_performance: Advantage,
    // advantage_persuasion: Advantage,
    // advantage_religion: Advantage,
    // advantage_sleight_of_hand: Advantage,
    // advantage_stealth: Advantage,
    // advantage_survival: Advantage,
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
            strength: (0, false),
            dexterity: (0, false),
            constitution: (0, false),
            intelligence: (0, false),
            wisdom: (0, false),
            charisma: (0, false),
            // skills
            proficiency_acrobatics: false,
            proficiency_animal_handling: false,
            proficiency_arcana: false,
            proficiency_athletics: false,
            proficiency_deception: false,
            proficiency_history: false,
            proficiency_insight: false,
            proficiency_intimidation: false,
            proficiency_investigation: false,
            proficiency_medicine: false,
            proficiency_nature: false,
            proficiency_perception: false,
            proficiency_performance: false,
            proficiency_persuasion: false,
            proficiency_religion: false,
            proficiency_sleight_of_hand: false,
            proficiency_stealth: false,
            proficiency_survival: false,

            // advantage_acrobatics: Advantage::No,
            // advantage_animal_handling: Advantage::No,
            // advantage_arcana: Advantage::No,
            // advantage_athletics: Advantage::No,
            // advantage_deception: Advantage::No,
            // advantage_history: Advantage::No,
            // advantage_insight: Advantage::No,
            // advantage_intimidation: Advantage::No,
            // advantage_investigation: Advantage::No,
            // advantage_medicine: Advantage::No,
            // advantage_nature: Advantage::No,
            // advantage_perception: Advantage::No,
            // advantage_performance: Advantage::No,
            // advantage_persuasion: Advantage::No,
            // advantage_religion: Advantage::No,
            // advantage_sleight_of_hand: Advantage::No,
            // advantage_stealth: Advantage::No,
            // advantage_survival: Advantage::No,

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
        
        self.strength = (16, false);
        self.dexterity = (8, false);
        self.constitution = (14, false);
        self.intelligence = (8, false);
        self.wisdom = (12, true);
        self.charisma = (16, true);
        // skills
        self.proficiency_acrobatics = false;
        self.proficiency_animal_handling = true;
        self.proficiency_arcana = false;
        self.proficiency_athletics = true;
        self.proficiency_deception = false;
        self.proficiency_history = false;
        self.proficiency_insight = false;
        self.proficiency_intimidation = true;
        self.proficiency_investigation = false;
        self.proficiency_medicine = false;
        self.proficiency_nature = false;
        self.proficiency_perception = false;
        self.proficiency_performance = false;
        self.proficiency_persuasion = true;
        self.proficiency_religion = false;
        self.proficiency_sleight_of_hand = false;
        self.proficiency_stealth = false;
        self.proficiency_survival = false;

        self.proficiency = 2;
        
        self.walking_speed = 30;

        self.inspiration = false;
        // hp section
        self.current_hp = 28;
        self.max_hp = 28;
        self.temp_hp = 0;
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

    /// stats
    pub fn get_strength(&self) -> u8 {
        self.strength.0
    }
    pub fn get_dexterity(&self) -> u8 {
        self.dexterity.0
    }
    pub fn get_constitution(&self) -> u8 {
        self.constitution.0
    }
    pub fn get_intelligence(&self) -> u8 {
        self.intelligence.0
    }
    pub fn get_wisdom(&self) -> u8 {
        self.wisdom.0
    }
    pub fn get_charisma(&self) -> u8 {
        self.charisma.0
    }
    /// PROFICIENRCY STATS
    pub fn get_prof_str(&self) -> bool {
        self.strength.1
    }
    pub fn get_prof_dex(&self) -> bool {
        self.dexterity.1
    }
    pub fn get_prof_con(&self) -> bool {
        self.constitution.1
    }
    pub fn get_prof_int(&self) -> bool {
        self.intelligence.1
    }
    pub fn get_prof_wis(&self) -> bool {
        self.wisdom.1
    }
    pub fn get_prof_cha(&self) -> bool {
        self.charisma.1
    }
    // PROFICIENCY SKILLS
    pub fn get_prof_acrobatics(&self) -> bool {
        self.proficiency_acrobatics
    }
    pub fn get_prof_animal_handling(&self) -> bool {
        self.proficiency_animal_handling
    }
    pub fn get_prof_arcana(&self) -> bool {
        self.proficiency_arcana
    }
    pub fn get_prof_athletics(&self) -> bool {
        self.proficiency_athletics
    }
    pub fn get_prof_deception(&self) -> bool {
        self.proficiency_deception
    }
    pub fn get_prof_history(&self) -> bool {
        self.proficiency_history
    }
    pub fn get_prof_insight(&self) -> bool {
        self.proficiency_insight
    }
    pub fn get_prof_intimidation(&self) -> bool {
        self.proficiency_intimidation
    }
    pub fn get_prof_investigation(&self) -> bool {
        self.proficiency_investigation
    }
    pub fn get_prof_medicine(&self) -> bool {
        self.proficiency_medicine
    }
    pub fn get_prof_nature(&self) -> bool {
        self.proficiency_nature
    }
    pub fn get_prof_perception(&self) -> bool {
        self.proficiency_perception
    }
    pub fn get_prof_performance(&self) -> bool {
        self.proficiency_performance
    }
    pub fn get_prof_persuasion(&self) -> bool {
        self.proficiency_persuasion
    }
    pub fn get_prof_religion(&self) -> bool {
        self.proficiency_religion
    }
    pub fn get_prof_sleight_of_hand(&self) -> bool {
        self.proficiency_sleight_of_hand
    }
    pub fn get_prof_stealth(&self) -> bool {
        self.proficiency_stealth
    }
    pub fn get_prof_survival(&self) -> bool {
        self.proficiency_survival
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
        // armor class 
        // initiative 
        // skill advantages (i.e. when equipping heavy armour)
    }
}
