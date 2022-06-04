pub struct Creature {
    pub name: String,
}

impl Creature {
    pub fn roll(&self) -> u8 {
        rand::random()
    }
}
