use crate::modules::pigeon::repository::pigeon::PigeonRepository;

pub async fn unjail_all() {
    let _result = PigeonRepository::unjail_all();
}
