pub struct PigeonWinningsBuilder {
    pub gold: i32,
    pub experience: i32,
    pub cleanliness: i32,
    pub happiness: i32,
    pub food: i32,
    pub health: i32,
}
impl PigeonWinningsBuilder {
    pub fn new() -> Self {
        Self {
            gold: 0,
            experience: 0,
            cleanliness: 0,
            happiness: 0,
            food: 0,
            health: 0,
        }
    }

    pub fn gold(&mut self, value: i32) -> &mut Self {
        self.gold = value;
        self
    }

    pub fn experience(&mut self, value: i32) -> &mut Self {
        self.experience = value;
        self
    }

    pub fn cleanliness(&mut self, value: i32) -> &mut Self {
        self.cleanliness = value;
        self
    }

    pub fn happiness(&mut self, value: i32) -> &mut Self {
        self.happiness = value;
        self
    }

    pub fn food(&mut self, value: i32) -> &mut Self {
        self.food = value;
        self
    }

    pub fn health(&mut self, value: i32) -> &mut Self {
        self.health = value;
        self
    }

    pub fn build(&self) -> PigeonWinnings {
        PigeonWinnings {
            gold: self.gold,
            experience: self.experience,
            cleanliness: self.cleanliness,
            happiness: self.happiness,
            food: self.food,
            health: self.health,
        }
    }
}

pub struct Item;

pub struct PigeonWinnings {
    pub gold: i32,
    pub experience: i32,
    pub cleanliness: i32,
    pub happiness: i32,
    pub food: i32,
    pub health: i32,
}

impl PigeonWinnings {
    pub fn to_string(&self) -> String {
        let mut messages: Vec<String> = Vec::new();

        if self.gold != 0 {
            messages.push(winning_to_string(self.gold, "gold"));
        }

        if self.cleanliness != 0 {
            messages.push(winning_to_string(self.cleanliness, "cleanliness"));
        }

        if self.health != 0 {
            messages.push(winning_to_string(self.health, "health"));
        }

        if self.experience != 0 {
            messages.push(winning_to_string(self.experience, "experience"));
        }

        if self.happiness != 0 {
            messages.push(winning_to_string(self.happiness, "happiness"));
        }

        if self.food != 0 {
            messages.push(winning_to_string(self.food, "food"));
        }
        messages.join(", ")
    }
}

pub fn winning_to_emoji(winning: &'static str) -> String {
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
    let mut emoji = winning_to_emoji(name);
    emoji.push_str(" ");
    if winning >= 0 {
        emoji.push_str(" +");
    }
    emoji.push_str(winning.to_string().as_str());
    emoji
}
