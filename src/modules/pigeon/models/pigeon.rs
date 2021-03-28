enum PigeonStatus {
    IDLE,
    MAILING,
    EXPLORING,
    FIGHTING,
    DATING,
}

enum PigeonCondition {
    ACTIVE,
    RAN_AWAY,
    DEAD,
}

enum Gender {
    MALE,
    FEMALE,
    OTHER,
}

struct Pigeon {
    pub id: i32,
    pub name: String,
    pub human_id: i32,
    pub condition: PigeonCondition,
    pub experience: i32,
    pub cleanliness: i32,
    pub happiness: i32,
    pub food: i32,
    pub health: i32,
    pub status: PigeonStatus,
    pub gender: Gender,
}
