// use std::error;

#[derive(Debug, Clone, Copy)]
pub enum Advantage {
    No,
    Advantage,
    Disadvantage,
}

#[derive(Debug)]
pub struct Character {
    name: String,
    race: String,
    class: String,
    level: u8,

    // abilities (ability_score, proficiency, modifier)
    // the modifier is a calculated value that doesnt need to be saved to a character file.
    //
    /// score, proficiency, modifier, save
    strength: (u8, bool, i8, i8),
    /// score, proficiency, modifier, save
    dexterity: (u8, bool, i8, i8),
    /// score, proficiency, modifier, save
    constitution: (u8, bool, i8, i8),
    /// score, proficiency, modifier, save
    intelligence: (u8, bool, i8, i8),
    /// score, proficiency, modifier, save
    wisdom: (u8, bool, i8, i8),
    /// score, proficiency, modifier, save
    charisma: (u8, bool, i8, i8),

    // skills (proficiency, modifier, advantage)
    // modifier and advantage are calculated values that dont need to be saved to a character file.
    //
    /// proficiency, modifier, Advantage
    acrobatics: (bool, i8, Advantage),
    /// proficiency, modifier, Advantage
    animal_handling: (bool, i8, Advantage),
    /// proficiency, modifier, Advantage
    arcana: (bool, i8, Advantage),
    /// proficiency, modifier, Advantage
    athletics: (bool, i8, Advantage),
    /// proficiency, modifier, Advantage
    deception: (bool, i8, Advantage),
    /// proficiency, modifier, Advantage
    history: (bool, i8, Advantage),
    /// proficiency, modifier, Advantage
    insight: (bool, i8, Advantage),
    /// proficiency, modifier, Advantage
    intimidation: (bool, i8, Advantage),
    /// proficiency, modifier, Advantage
    investigation: (bool, i8, Advantage),
    /// proficiency, modifier, Advantage
    medicine: (bool, i8, Advantage),
    /// proficiency, modifier, Advantage
    nature: (bool, i8, Advantage),
    /// proficiency, modifier, Advantage
    perception: (bool, i8, Advantage),
    /// proficiency, modifier, Advantage
    performance: (bool, i8, Advantage),
    /// proficiency, modifier, Advantage
    persuasion: (bool, i8, Advantage),
    /// proficiency, modifier, Advantage
    religion: (bool, i8, Advantage),
    /// proficiency, modifier, Advantage
    sleight_of_hand: (bool, i8, Advantage),
    /// proficiency, modifier, Advantage
    stealth: (bool, i8, Advantage),
    /// proficiency, modifier, Advantage
    survival: (bool, i8, Advantage),

    // these are calculated values that dont need to be saved to a character file.
    passive_perception: u8,
    passive_investigation: u8,
    passive_insight: u8,

    /// stuff
    ///
    /// proficiency
    proficiency: u8,

    walking_speed: u8,

    inspiration: bool,

    // initiative is a calculated value that doesnt need to be saved to a character file.
    initiative: i8,
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
            strength: (0, false, 0, 0),
            dexterity: (0, false, 0, 0),
            constitution: (0, false, 0, 0),
            intelligence: (0, false, 0, 0),
            wisdom: (0, false, 0, 0),
            charisma: (0, false, 0, 0),
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

            passive_perception: 0,
            passive_investigation: 0,
            passive_insight: 0,

            proficiency: 0,

            walking_speed: 0,

            inspiration: false,

            initiative: 0,
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
        
        self.strength = (16, false, 0, 0);
        self.dexterity = (8, false, 0, 0);
        self.constitution = (14, false, 0, 0);
        self.intelligence = (8, false, 0, 0);
        self.wisdom = (12, true, 0, 0);
        self.charisma = (16, true, 0, 0);
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
    
    /// returns: score, proficiency, modifier, save
    pub fn get_strength(&self) -> (u8, bool, i8, i8) {
        self.strength
    }
    /// returns: score, proficiency, modifier, save
    pub fn get_dexterity(&self) -> (u8, bool, i8, i8) {
        self.dexterity
    }
    /// returns: score, proficiency, modifier, save
    pub fn get_constitution(&self) -> (u8, bool, i8, i8) {
        self.constitution
    }
    /// returns: score, proficiency, modifier, save
    pub fn get_intelligence(&self) -> (u8, bool, i8, i8) {
        self.intelligence
    }
    /// returns: score, proficiency, modifier, save
    pub fn get_wisdom(&self) -> (u8, bool, i8, i8) {
        self.wisdom
    }
    /// returns: score, proficiency, modifier, save
    pub fn get_charisma(&self) -> (u8, bool, i8, i8) {
        self.charisma
    }

    // PROFICIENCY SKILLS
    /// returns: proficiency, modifier, Advantage
    pub fn get_acrobatics(&self) -> &(bool, i8, Advantage) {
        &self.acrobatics
    }
    /// returns: proficiency, modifier, Advantage
    pub fn get_animal_handling(&self) -> &(bool, i8, Advantage) {
        &self.animal_handling
    }
    /// returns: proficiency, modifier, Advantage
    pub fn get_arcana(&self) -> &(bool, i8, Advantage) {
        &self.arcana
    }
    /// returns: proficiency, modifier, Advantage
    pub fn get_athletics(&self) -> &(bool, i8, Advantage) {
        &self.athletics
    }
    /// returns: proficiency, modifier, Advantage
    pub fn get_deception(&self) -> &(bool, i8, Advantage) {
        &self.deception
    }
    /// returns: proficiency, modifier, Advantage
    pub fn get_history(&self) -> &(bool, i8, Advantage) {
        &self.history
    }
    /// returns: proficiency, modifier, Advantage
    pub fn get_insight(&self) -> &(bool, i8, Advantage) {
        &self.insight
    }
    /// returns: proficiency, modifier, Advantage
    pub fn get_intimidation(&self) -> &(bool, i8, Advantage) {
        &self.intimidation
    }
    /// returns: proficiency, modifier, Advantage
    pub fn get_investigation(&self) -> &(bool, i8, Advantage) {
        &self.investigation
    }
    /// returns: proficiency, modifier, Advantage
    pub fn get_medicine(&self) -> &(bool, i8, Advantage) {
        &self.medicine
    }
    /// returns: proficiency, modifier, Advantage
    pub fn get_nature(&self) -> &(bool, i8, Advantage) {
        &self.nature
    }
    /// returns: proficiency, modifier, Advantage
    pub fn get_perception(&self) -> &(bool, i8, Advantage) {
        &self.perception
    }
    /// returns: proficiency, modifier, Advantage
    pub fn get_performance(&self) -> &(bool, i8, Advantage) {
        &self.performance
    }
    /// returns: proficiency, modifier, Advantage
    pub fn get_persuasion(&self) -> &(bool, i8, Advantage) {
        &self.persuasion
    }
    /// returns: proficiency, modifier, Advantage
    pub fn get_religion(&self) -> &(bool, i8, Advantage) {
        &self.religion
    }
    /// returns: proficiency, modifier, Advantage
    pub fn get_sleight_of_hand(&self) -> &(bool, i8, Advantage) {
        &self.sleight_of_hand
    }
    /// returns: proficiency, modifier, Advantage
    pub fn get_stealth(&self) -> &(bool, i8, Advantage) {
        &self.stealth
    }
    /// returns: proficiency, modifier, Advantage
    pub fn get_survival(&self) -> &(bool, i8, Advantage) {
        &self.survival
    }

    pub fn get_passive_perception(&self) -> u8 {
        self.passive_perception
    }
    pub fn get_passive_investigation(&self) -> u8 {
        self.passive_investigation
    }
    pub fn get_passive_insight(&self) -> u8 {
        self.passive_insight
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
        self.initiative
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

    fn calculate_modifier(&self, skill: u8, proficient: bool) -> i8 {
        let skill: i8 = skill.try_into().unwrap();

        if proficient {
            let prof: i8 = self.get_proficiency().try_into().unwrap();
            ((skill - 10) / 2) + prof
        } else {
            (skill - 10) / 2 
        }
    }

    /// any variable value needs to be reset here.
    pub fn recalculate(&mut self) {
        // ability modifiers
        self.strength.2 = self.calculate_modifier(self.strength.0, false);
        self.dexterity.2 = self.calculate_modifier(self.dexterity.0, false);
        self.constitution.2 = self.calculate_modifier(self.constitution.0, false);
        self.intelligence.2 = self.calculate_modifier(self.intelligence.0, false);
        self.wisdom.2 = self.calculate_modifier(self.wisdom.0, false);
        self.charisma.2 = self.calculate_modifier(self.charisma.0, false);
        // ability saves
        self.strength.3 = self.calculate_modifier(self.strength.0, self.strength.1);
        self.dexterity.3 = self.calculate_modifier(self.dexterity.0, self.dexterity.1);
        self.constitution.3 = self.calculate_modifier(self.constitution.0, self.constitution.1);
        self.intelligence.3 = self.calculate_modifier(self.intelligence.0, self.intelligence.1);
        self.wisdom.3 = self.calculate_modifier(self.wisdom.0, self.wisdom.1);
        self.charisma.3 = self.calculate_modifier(self.charisma.0, self.charisma.1);
        // skill modifiers 
        self.acrobatics.1 = self.calculate_modifier(self.dexterity.0, self.acrobatics.0);
        self.animal_handling.1 = self.calculate_modifier(self.get_wisdom().0, self.animal_handling.0);
        self.arcana.1 = self.calculate_modifier(self.get_intelligence().0, self.arcana.0);
        self.athletics.1 = self.calculate_modifier(self.get_strength().0, self.athletics.0);
        self.deception.1 = self.calculate_modifier(self.get_charisma().0, self.deception.0);
        self.history.1 = self.calculate_modifier(self.get_intelligence().0, self.history.0);
        self.insight.1 = self.calculate_modifier(self.get_wisdom().0, self.insight.0);
        self.intimidation.1 = self.calculate_modifier(self.get_charisma().0, self.intimidation.0);
        self.investigation.1 = self.calculate_modifier(self.get_intelligence().0, self.investigation.0);
        self.medicine.1 = self.calculate_modifier(self.get_wisdom().0, self.medicine.0);
        self.nature.1 = self.calculate_modifier(self.get_intelligence().0, self.nature.0);
        self.perception.1 = self.calculate_modifier(self.get_wisdom().0, self.perception.0);
        self.performance.1 = self.calculate_modifier(self.get_charisma().0, self.performance.0);
        self.persuasion.1 = self.calculate_modifier(self.get_charisma().0, self.persuasion.0);
        self.religion.1 = self.calculate_modifier(self.get_intelligence().0, self.religion.0);
        self.sleight_of_hand.1 = self.calculate_modifier(self.get_dexterity().0, self.sleight_of_hand.0);
        self.stealth.1 = self.calculate_modifier(self.get_dexterity().0, self.stealth.0);
        self.survival.1 = self.calculate_modifier(self.get_wisdom().0, self.survival.0);
        // skill advantages 
        // TODO: implement propper calculation for all skills
        self.stealth.2 = Advantage::Disadvantage;
        // passive skills 
        // Here's how to determine a character's total for a passive check:
        // 10 + all modifiers that normally apply to the check If the character has advantage on
        // the check, add 5. For disadvantage, subtract 5. The game refers to a passive check total
        // as a score.
        self.passive_perception = (10 + self.wisdom.2) as u8 + (self.proficiency * if self.perception.0 {1} else {0});
        self.passive_investigation = (10 + self.intelligence.2) as u8 + (self.proficiency * if self.investigation.0 {1} else {0});
        self.passive_insight = (10 + self.wisdom.2) as u8 + (self.proficiency * if self.insight.0 {1} else {0});
        // initiative 
        // At the beginning of every combat, you roll initiative by making a Dexterity check.
        // mby add any special modifier...?
        self.initiative = self.dexterity.2;
        // armor class 
        // Without armor or a shield, your character's AC equals 10 + his or her Dexterity modifier.
        // If your character wears armor, carries a shield, or both, calculate your AC using the
        // rules in chapter 5. Record your AC on your character sheet.
        // you have to be proficient tho!
        // 
        // Some spells and class features give you a different way to calculate your AC. If you
        // have multiple features that give you different ways to calculate your AC, you choose
        // which one to use.
        //
        // skill advantages (i.e. when equipping heavy armour)
    }
}
