// use std::error;
use crate::character_limbs::ability::Ability;
use crate::character_limbs::skill::Skill;
use crate::character_limbs::advantage::Advantage;
use crate::character_limbs::spells::Spell;

use std::fs;

use serde_json::Result;

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
    strength: Ability,
    /// score, proficiency, modifier, save
    dexterity: Ability,
    /// score, proficiency, modifier, save
    constitution: Ability,
    /// score, proficiency, modifier, save
    intelligence: Ability,
    /// score, proficiency, modifier, save
    wisdom: Ability,
    /// score, proficiency, modifier, save
    charisma: Ability,

    // skills (proficiency, modifier, advantage)
    // modifier and advantage are calculated values that dont need to be saved to a character file.
    //
    /// proficiency, modifier, advantage
    acrobatics: Skill,
    /// proficiency, modifier, advantage
    animal_handling: Skill,
    /// proficiency, modifier, advantage
    arcana: Skill,
    /// proficiency, modifier, advantage
    athletics: Skill,
    /// proficiency, modifier, advantage
    deception: Skill,
    /// proficiency, modifier, advantage
    history: Skill,
    /// proficiency, modifier, advantage
    insight: Skill,
    /// proficiency, modifier, advantage
    intimidation: Skill,
    /// proficiency, modifier, advantage
    investigation: Skill,
    /// proficiency, modifier, advantage
    medicine: Skill,
    /// proficiency, modifier, advantage
    nature: Skill,
    /// proficiency, modifier, advantage
    perception: Skill,
    /// proficiency, modifier, advantage
    performance: Skill,
    /// proficiency, modifier, advantage
    persuasion: Skill,
    /// proficiency, modifier, advantage
    religion: Skill,
    /// proficiency, modifier, advantage
    sleight_of_hand: Skill,
    /// proficiency, modifier, advantage
    stealth: Skill,
    /// proficiency, modifier, advantage
    survival: Skill,

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

    spells_prepared: Vec<Spell>,
    spells_known: Vec<Spell>,
}

impl Default for Character {
    fn default() -> Self {
        Self {
            name: "name".to_string(),
            race: "race".to_string(),
            class: "class".to_string(),
            level: 0,
            // stats
            strength: Ability::new(),
            dexterity: Ability::new(),
            constitution: Ability::new(),
            intelligence: Ability::new(),
            wisdom: Ability::new(),
            charisma: Ability::new(),
            // skills
            acrobatics: Skill::new(),
            animal_handling: Skill::new(),
            arcana: Skill::new(),
            athletics: Skill::new(),
            deception: Skill::new(),
            history: Skill::new(),
            insight: Skill::new(),
            intimidation: Skill::new(),
            investigation: Skill::new(),
            medicine: Skill::new(),
            nature: Skill::new(),
            perception: Skill::new(),
            performance: Skill::new(),
            persuasion: Skill::new(),
            religion: Skill::new(),
            sleight_of_hand: Skill::new(),
            stealth: Skill::new(),
            survival: Skill::new(),

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

            spells_prepared: vec![Spell::new()],
            spells_known: vec![Spell::new()],
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
        
        // self.strength = (16, false, 0, 0);
        self.strength.score = 16;
        self.strength.proficiency = false;
        self.dexterity.score = 8;
        self.dexterity.proficiency = false;
        self.constitution.score = 14;
        self.constitution.proficiency = false;
        self.intelligence.score = 8;
        self.intelligence.proficiency = false;
        self.wisdom.score = 12;
        self.wisdom.proficiency =  true;
        self.charisma.score = 16;
        self.charisma.proficiency = true;
        // skills
        self.acrobatics.proficiency = false;
        self.animal_handling.proficiency = true;
        self.arcana.proficiency = false;
        self.athletics.proficiency = true;
        self.deception.proficiency = false;
        self.history.proficiency = false;
        self.insight.proficiency = false;
        self.intimidation.proficiency = true;
        self.investigation.proficiency = false;
        self.medicine.proficiency = false;
        self.nature.proficiency = false;
        self.perception.proficiency = false;
        self.performance.proficiency = false;
        self.persuasion.proficiency = true;
        self.religion.proficiency = false;
        self.sleight_of_hand.proficiency = false;
        self.stealth.proficiency = false;
        self.survival.proficiency = false;

        self.proficiency = 2;
        
        self.walking_speed = 30;

        self.inspiration = false;
        // hp section
        self.current_hp = 28;
        self.max_hp = 28;
        self.temp_hp = 0;


        self.spells_prepared = self.load_spells();

        // make sure to calculate everything after loading
        self.recalculate();
    }

    pub fn load_spells(&self) -> Vec<Spell> {
        let spell_files = fs::read_dir("spells").unwrap();
        let mut spells: Vec<Spell> = vec![];
        for file_path in spell_files {

            let contents = fs::read_to_string(file_path.unwrap().path())
                .expect("Should have been able to read the file");

            if let Ok(spell) = self.load_spells_file(&contents) { spells.extend(vec![spell]); }
        }
        spells
    }

    pub fn load_spells_file(&self, data: &str) -> Result<Spell> {
        serde_json::from_str(data)
    }

    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn race(&self) -> &String {
        &self.race
    }
    pub fn class(&self) -> &String {
        &self.class
    }
    pub fn level(&self) -> u8 {
        self.level
    }

    // stats
    
    /// returns: score, proficiency, modifier, save
    pub fn strength(&self) -> Ability {
        self.strength
    }
    /// returns: score, proficiency, modifier, save
    pub fn dexterity(&self) -> Ability {
        self.dexterity
    }
    /// returns: score, proficiency, modifier, save
    pub fn constitution(&self) -> Ability {
        self.constitution
    }
    /// returns: score, proficiency, modifier, save
    pub fn intelligence(&self) -> Ability {
        self.intelligence
    }
    /// returns: score, proficiency, modifier, save
    pub fn wisdom(&self) -> Ability {
        self.wisdom
    }
    /// returns: score, proficiency, modifier, save
    pub fn charisma(&self) -> Ability {
        self.charisma
    }

    // PROFICIENCY SKILLS
    /// returns: proficiency, modifier, Advantage
    pub fn acrobatics(&self) -> &Skill {
        &self.acrobatics
    }
    /// returns: proficiency, modifier, Advantage
    pub fn animal_handling(&self) -> &Skill {
        &self.animal_handling
    }
    /// returns: proficiency, modifier, Advantage
    pub fn arcana(&self) -> &Skill {
        &self.arcana
    }
    /// returns: proficiency, modifier, Advantage
    pub fn athletics(&self) -> &Skill {
        &self.athletics
    }
    /// returns: proficiency, modifier, Advantage
    pub fn deception(&self) -> &Skill {
        &self.deception
    }
    /// returns: proficiency, modifier, Advantage
    pub fn history(&self) -> &Skill {
        &self.history
    }
    /// returns: proficiency, modifier, Advantage
    pub fn insight(&self) -> &Skill {
        &self.insight
    }
    /// returns: proficiency, modifier, Advantage
    pub fn intimidation(&self) -> &Skill {
        &self.intimidation
    }
    /// returns: proficiency, modifier, Advantage
    pub fn investigation(&self) -> &Skill {
        &self.investigation
    }
    /// returns: proficiency, modifier, Advantage
    pub fn medicine(&self) -> &Skill {
        &self.medicine
    }
    /// returns: proficiency, modifier, Advantage
    pub fn nature(&self) -> &Skill {
        &self.nature
    }
    /// returns: proficiency, modifier, Advantage
    pub fn perception(&self) -> &Skill {
        &self.perception
    }
    /// returns: proficiency, modifier, Advantage
    pub fn performance(&self) -> &Skill {
        &self.performance
    }
    /// returns: proficiency, modifier, Advantage
    pub fn persuasion(&self) -> &Skill {
        &self.persuasion
    }
    /// returns: proficiency, modifier, Advantage
    pub fn religion(&self) -> &Skill {
        &self.religion
    }
    /// returns: proficiency, modifier, Advantage
    pub fn sleight_of_hand(&self) -> &Skill {
        &self.sleight_of_hand
    }
    /// returns: proficiency, modifier, Advantage
    pub fn stealth(&self) -> &Skill {
        &self.stealth
    }
    /// returns: proficiency, modifier, Advantage
    pub fn survival(&self) -> &Skill {
        &self.survival
    }

    pub fn passive_perception(&self) -> u8 {
        self.passive_perception
    }
    pub fn passive_investigation(&self) -> u8 {
        self.passive_investigation
    }
    pub fn passive_insight(&self) -> u8 {
        self.passive_insight
    }


    pub fn proficiency(&self) -> u8 {
        self.proficiency
    }

    pub fn walking_speed(&self) -> u8 {
        self.walking_speed
    }

    pub fn inspiration(&self) -> bool {
        self.inspiration
    }

    pub fn set_inspiration(&mut self, new_value: bool) {
        self.inspiration = new_value;
    }

    pub fn current_hp(&self) -> i32 {
        self.current_hp
    }

    pub fn max_hp(&self) -> u32 {
        self.max_hp
    }

    pub fn temp_hp(&self) -> i32 {
        self.temp_hp
    }

    pub fn initiative(&self) -> i8 {
        self.initiative
    }

    pub fn armor_class(&self) -> u8 {
        // TODO: needs to be calculated properly!
        19
    }

    pub fn defenses(&self) -> String {
        // TODO: needs to be saved properly!
        "r fire\ni disease".to_string()
    }

    pub fn conditions(&self) -> String {
        // TODO: needs to be saved properly!
        "add active conditions".to_string()
    }

    pub fn spells_prepared(&self) -> &Vec<Spell> {
        &self.spells_prepared
    }

    pub fn spells_known(&self) -> &Vec<Spell> {
        &self.spells_known
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
            let prof: i8 = self.proficiency().try_into().unwrap();
            ((skill - 10) / 2) + prof
        } else {
            (skill - 10) / 2 
        }
    }

    /// any variable value needs to be reset here.
    pub fn recalculate(&mut self) {
        // ability modifiers
        self.strength.modifier = self.calculate_modifier(self.strength.score, false);
        self.dexterity.modifier = self.calculate_modifier(self.dexterity.score, false);
        self.constitution.modifier = self.calculate_modifier(self.constitution.score, false);
        self.intelligence.modifier = self.calculate_modifier(self.intelligence.score, false);
        self.wisdom.modifier = self.calculate_modifier(self.wisdom.score, false);
        self.charisma.modifier = self.calculate_modifier(self.charisma.score, false);
        // ability saves
        self.strength.save = self.calculate_modifier(self.strength.score, self.strength.proficiency);
        self.dexterity.save = self.calculate_modifier(self.dexterity.score, self.dexterity.proficiency);
        self.constitution.save = self.calculate_modifier(self.constitution.score, self.constitution.proficiency);
        self.intelligence.save = self.calculate_modifier(self.intelligence.score, self.intelligence.proficiency);
        self.wisdom.save = self.calculate_modifier(self.wisdom.score, self.wisdom.proficiency);
        self.charisma.save = self.calculate_modifier(self.charisma.score, self.charisma.proficiency);
        // skill modifiers 
        self.acrobatics.modifier = self.calculate_modifier(self.dexterity.score, self.acrobatics.proficiency);
        self.animal_handling.modifier = self.calculate_modifier(self.wisdom.score, self.animal_handling.proficiency);
        self.arcana.modifier = self.calculate_modifier(self.intelligence.score, self.arcana.proficiency);
        self.athletics.modifier = self.calculate_modifier(self.strength.score, self.athletics.proficiency);
        self.deception.modifier = self.calculate_modifier(self.charisma.score, self.deception.proficiency);
        self.history.modifier = self.calculate_modifier(self.intelligence.score, self.history.proficiency);
        self.insight.modifier = self.calculate_modifier(self.wisdom.score, self.insight.proficiency);
        self.intimidation.modifier = self.calculate_modifier(self.charisma.score, self.intimidation.proficiency);
        self.investigation.modifier = self.calculate_modifier(self.intelligence.score, self.investigation.proficiency);
        self.medicine.modifier = self.calculate_modifier(self.wisdom.score, self.medicine.proficiency);
        self.nature.modifier = self.calculate_modifier(self.intelligence.score, self.nature.proficiency);
        self.perception.modifier = self.calculate_modifier(self.wisdom.score, self.perception.proficiency);
        self.performance.modifier = self.calculate_modifier(self.charisma.score, self.performance.proficiency);
        self.persuasion.modifier = self.calculate_modifier(self.charisma.score, self.persuasion.proficiency);
        self.religion.modifier = self.calculate_modifier(self.intelligence.score, self.religion.proficiency);
        self.sleight_of_hand.modifier = self.calculate_modifier(self.dexterity.score, self.sleight_of_hand.proficiency);
        self.stealth.modifier = self.calculate_modifier(self.dexterity.score, self.stealth.proficiency);
        self.survival.modifier = self.calculate_modifier(self.wisdom.score, self.survival.proficiency);
        // skill advantages 
        // TODO: implement propper calculation for all skills
        self.stealth.advantage = Advantage::Disadvantage;
        // passive skills 
        // Here's how to determine a character's total for a passive check:
        // 10 + all modifiers that normally apply to the check If the character has advantage on
        // the check, add 5. For disadvantage, subtract 5. The game refers to a passive check total
        // as a score.
        self.passive_perception = (10 + self.wisdom.modifier) as u8 + (self.proficiency * if self.perception.proficiency {1} else {0});
        self.passive_investigation = (10 + self.intelligence.modifier) as u8 + (self.proficiency * if self.investigation.proficiency {1} else {0});
        self.passive_insight = (10 + self.wisdom.modifier) as u8 + (self.proficiency * if self.insight.proficiency {1} else {0});
        // initiative 
        // At the beginning of every combat, you roll initiative by making a Dexterity check.
        // mby add any special modifier...?
        self.initiative = self.dexterity.modifier;
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
