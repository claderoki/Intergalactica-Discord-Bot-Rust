use crate::modules::{pigeon::{models::pigeon::Pigeon, repository::pigeon::PigeonRepository}, shared::models::human::Human};

pub trait PigeonUtils {
    fn get_pigeon(&self) -> Option<Pigeon>;
    fn has_pigeon(&self) -> bool;
    fn create_pigeon(&self, name: &str) -> Result<Pigeon, &'static str>;
}

impl PigeonUtils for Human {
    fn get_pigeon(&self) -> Option<Pigeon> {
        PigeonRepository::get_active(self.id).ok()
    }

    fn has_pigeon(&self) -> bool {
        //TODO: create a PigeonRepository method for this (to avoid needlessly selecting the entire pigeon.)
        self.get_pigeon().is_some()
    }

    fn create_pigeon(&self, name: &str) -> Result<Pigeon, &'static str> {
        PigeonRepository::create(self.id, name)
    }
}