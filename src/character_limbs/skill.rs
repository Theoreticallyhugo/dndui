use crate::character_limbs::advantage::Advantage;

#[derive(Debug, Clone, Copy, Default)]
pub struct Skill {
    pub proficiency: bool,
    /// general base modifier
    pub modifier: i8,
    /// modifier for saving throws
    pub advantage: Advantage,
}

impl Skill {
    /// Constructs a new instance of [`Ability`].
    pub fn new() -> Self {
        Self::default()
    }
}
