use crate::modules::{pigeon::{models::pigeon::Pigeon, repository::pigeon::PigeonRepository}, shared::models::human::Human};

pub trait PigeonUtils {
    fn get_pigeon(&self) -> Option<Pigeon>;
    fn has_pigeon(&self) -> bool;
    fn create_pigeon(&self, name: &str) -> Result<(), &'static str>;
}

impl PigeonUtils for Human {
    fn get_pigeon(&self) -> Option<Pigeon> {
        PigeonRepository::get_active(self.id).ok()
    }

    fn has_pigeon(&self) -> bool {
        PigeonRepository::has_active(self.id).is_ok()
    }

    fn create_pigeon(&self, name: &str) -> Result<(), &'static str> {
        PigeonRepository::create(self.id, name)
    }
}

pub struct PigeonWinnings {
    pub gold: i32,
    pub experience: i32,
    pub cleanliness: i32,
    pub happiness: i32,
    pub food: i32,
    pub health: i32
}
impl PigeonWinnings {
    pub fn new() -> Self {
        PigeonWinnings {
            gold: 0,
            experience: 0,
            cleanliness: 0,
            happiness: 0,
            food: 0,
            health: 0,
        }
    }

    pub fn gold(&mut self, value: i32) -> &Self {
        self.gold = value;
        self
    }

    pub fn experience(&mut self, value: i32) -> &Self {
        self.experience = value;
        self
    }

    pub fn cleanliness(&mut self, value: i32) -> &Self {
        self.cleanliness = value;
        self
    }

    pub fn happiness(&mut self, value: i32) -> &Self {
        self.happiness = value;
        self
    }

    pub fn food(&mut self, value: i32) -> &Self {
        self.food = value;
        self
    }

    pub fn health(&mut self, value: i32) -> &Self {
        self.health = value;
        self
    }

}
