// use std::error;

pub struct Character {
    name: String,
    race: String,
    class: String,
    level: u8,
    /// stats 
    ///
    strength: u8,
    dexterity: u8,
    constitution: u8,
    intelligence: u8,
    wisdom: u8,
    charisma: u8,
    /// proficiency stats
    proficiency_str: bool,
    proficiency_dex: bool,
    proficiency_con: bool,
    proficiency_int: bool,
    proficiency_wis: bool,
    proficiency_cha: bool,
    /// skills proficiencies
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
            strength: 0,
            dexterity: 0,
            constitution: 0,
            intelligence: 0,
            wisdom: 0,
            charisma: 0,
            // proficiency stats
            proficiency_str: false,
            proficiency_dex: false,
            proficiency_con: false,
            proficiency_int: false,
            proficiency_wis: false,
            proficiency_cha: false,
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
        
        self.strength = 16;
        self.dexterity = 8;
        self.constitution = 14;
        self.intelligence = 8;
        self.wisdom = 12;
        self.charisma = 16;
        // proficiency stats
        self.proficiency_str = false;
        self.proficiency_dex = false;
        self.proficiency_con = false;
        self.proficiency_int = false;
        self.proficiency_wis = true;
        self.proficiency_cha = true;
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
    /// PROFICIENRCY STATS
    pub fn get_prof_str(&self) -> bool {
        self.proficiency_str
    }
    pub fn get_prof_dex(&self) -> bool {
        self.proficiency_dex
    }
    pub fn get_prof_con(&self) -> bool {
        self.proficiency_con
    }
    pub fn get_prof_int(&self) -> bool {
        self.proficiency_int
    }
    pub fn get_prof_wis(&self) -> bool {
        self.proficiency_wis
    }
    pub fn get_prof_cha(&self) -> bool {
        self.proficiency_cha
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
}
