#[derive(Debug, Clone, Default)]
pub struct Spell {
    pub name: String,
    pub class: String,
    pub level: u8,
    pub concentration: bool,
    pub ritual: bool,
    /// like 1 bonus action, should be a struct
    pub time: i16,
    pub time_format: String,

    pub range: i16,
    /// should be a struct
    pub hit_dc: u8,
    pub hit_dc_ability: String,
    /// like buff, prone, or 1d4, should be a struct
    pub effect: String,
    /// like 10 minutes, should be a struct
    pub duration: i16,
    pub duration_format: String,

    /// like v/m (one piece of string), should be a struct
    pub components: String,
    /// area of effect, like 5 ft cube, should be a struct
    pub aoe: i16,
    pub aoe_format: String,
}

impl Spell {
    /// Constructs a new instance of [`Ability`].
    pub fn new() -> Self {
        Self::default()
    }
}
