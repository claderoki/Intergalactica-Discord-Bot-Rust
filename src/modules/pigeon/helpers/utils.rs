pub struct PigeonWinningsBuilder {
    pub gold: i32,
    pub experience: i32,
    pub cleanliness: i32,
    pub happiness: i32,
    pub food: i32,
    pub health: i32,
    pub item_ids: Vec<i32>,
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
            item_ids: Vec::new(),
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

    pub fn add_item_id(&mut self, value: i32) -> &mut Self {
        self.item_ids.push(value);
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
            item_ids: (*self.item_ids).to_vec(),
        }
    }
}

pub struct PigeonWinnings {
    pub gold: i32,
    pub experience: i32,
    pub cleanliness: i32,
    pub happiness: i32,
    pub food: i32,
    pub health: i32,
    pub item_ids: Vec<i32>,
}

pub trait PigeonWinnable {
    fn get_gold(&self) -> i32;

    fn get_cleanliness(&self) -> i32;

    fn get_happiness(&self) -> i32;

    fn get_health(&self) -> i32;

    fn get_experience(&self) -> i32;

    fn get_food(&self) -> i32;

    fn get_seperator(&self) -> String {
        ", ".into()
    }

    fn is_gained(&self) -> bool {
        true
    }

    fn to_string(&self) -> String {
        let mut messages: Vec<String> = Vec::new();

        if self.get_gold() != 0 {
            messages.push(winning_to_string(self.get_gold(), "gold", self.is_gained()));
        }

        if self.get_cleanliness() != 0 {
            messages.push(winning_to_string(
                self.get_cleanliness(),
                "cleanliness",
                self.is_gained(),
            ));
        }

        if self.get_health() != 0 {
            messages.push(winning_to_string(
                self.get_health(),
                "health",
                self.is_gained(),
            ));
        }

        if self.get_experience() != 0 {
            messages.push(winning_to_string(
                self.get_experience(),
                "experience",
                self.is_gained(),
            ));
        }

        if self.get_happiness() != 0 {
            messages.push(winning_to_string(
                self.get_happiness(),
                "happiness",
                self.is_gained(),
            ));
        }

        if self.get_food() != 0 {
            messages.push(winning_to_string(self.get_food(), "food", self.is_gained()));
        }

        messages.join(&self.get_seperator())
    }
}

impl PigeonWinnable for PigeonWinnings {
    fn get_gold(&self) -> i32 {
        self.gold
    }

    fn get_health(&self) -> i32 {
        self.health
    }

    fn get_cleanliness(&self) -> i32 {
        self.cleanliness
    }

    fn get_happiness(&self) -> i32 {
        self.happiness
    }

    fn get_experience(&self) -> i32 {
        self.experience
    }

    fn get_food(&self) -> i32 {
        self.food
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

fn winning_to_string(winning: i32, name: &'static str, show_plus: bool) -> String {
    let mut emoji = winning_to_emoji(name);
    emoji.push_str(" ");
    if winning >= 0 && show_plus {
        emoji.push_str("+");
    }
    emoji.push_str(winning.to_string().as_str());
    emoji
}
