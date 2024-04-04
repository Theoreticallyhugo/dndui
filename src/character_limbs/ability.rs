#[derive(Debug, Clone, Copy, Default)]
pub struct Ability {
    pub score: u8,
    pub proficiency: bool,
    /// general base modifier
    pub modifier: i8,
    /// modifier for saving throws
    pub save: i8,
}

impl Ability {
    /// Constructs a new instance of [`Ability`].
    pub fn new() -> Self {
        Self::default()
    }
}
