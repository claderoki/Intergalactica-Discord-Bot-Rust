// pub trait PigeonUtils {
//     fn get_pigeon(&self) -> Option<Pigeon>;
//     fn has_pigeon(&self) -> bool;
//     fn create_pigeon(&self, name: &str) -> Result<(), &'static str>;
// }

// impl PigeonUtils for Human {
//     fn get_pigeon(&self) -> Option<Pigeon> {
//         PigeonRepository::get_active(self.id).ok()
//     }

//     fn has_pigeon(&self) -> bool {
//         PigeonRepository::has_active(self.id).is_ok()
//     }

//     fn create_pigeon(&self, name: &str) -> Result<(), &'static str> {
//         PigeonRepository::create(self.id, name)
//     }
// }

pub struct PigeonWinnings {
    pub gold: i32,
    pub experience: i32,
    pub cleanliness: i32,
    pub happiness: i32,
    pub food: i32,
    pub health: i32,
}
impl PigeonWinnings {
    // pub fn new() -> Self {
    //     PigeonWinnings {
    //         gold: 0,
    //         experience: 0,
    //         cleanliness: 0,
    //         happiness: 0,
    //         food: 0,
    //         health: 0,
    //     }
    // }

    // pub fn gold(&mut self, value: i32) -> &mut Self {
    //     self.gold = value;
    //     self
    // }

    // pub fn experience(&mut self, value: i32) -> &mut Self {
    //     self.experience = value;
    //     self
    // }

    // pub fn cleanliness(&mut self, value: i32) -> &mut Self {
    //     self.cleanliness = value;
    //     self
    // }

    // pub fn happiness(&mut self, value: i32) -> &mut Self {
    //     self.happiness = value;
    //     self
    // }

    // pub fn food(&mut self, value: i32) -> &mut Self {
    //     self.food = value;
    //     self
    // }

    // pub fn health(&mut self, value: i32) -> &mut Self {
    //     self.health = value;
    //     self
    // }

    // pub fn build(&self) -> &Self {
    //     self
    // }
}

impl PigeonWinnings {
    pub fn to_string(&self) -> String {
        let mut description = String::from("");
        description.push_str(&winning_to_string(self.gold, "gold"));
        description.push_str(&winning_to_string(self.cleanliness, "cleanliness"));
        description.push_str(&winning_to_string(self.health, "health"));
        description.push_str(&winning_to_string(self.experience, "experience"));
        description.push_str(&winning_to_string(self.food, "food"));
        description
    }
}

fn winning_to_emoji(winning: &'static str) -> String {
    String::from(match winning {
        "gold" => "💶",
        "experience" => "📊",
        "cleanliness" => "💩",
        "happiness" => "🌻",
        "food" => "🌾",
        "health" => "❤️",
        &_ => "",
    })
}

fn winning_to_string(winning: i32, name: &'static str) -> String {
    String::from(if winning != 0 {
        let mut emoji = winning_to_emoji(name);
        emoji.push_str(" ");
        if winning > 0 {
            emoji.push_str(" +");
        }
        emoji.push_str(winning.to_string().as_str());
        emoji.push_str(", ");
        return emoji;
    } else {
        ""
    })
}
