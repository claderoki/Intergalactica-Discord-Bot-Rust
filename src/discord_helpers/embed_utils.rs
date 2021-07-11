use serenity::builder::CreateEmbed;

pub trait EmbedExtension {
    fn normal_embed<D>(&mut self, text: D) -> &mut Self
    where
        D: ToString;
    fn error_embed<D>(&mut self, text: D) -> &mut Self
    where
        D: ToString;

    fn default_color(&mut self) -> &mut Self;

    fn warning_color(&mut self) -> &mut Self;

    fn success_color(&mut self) -> &mut Self;

    fn danger_color(&mut self) -> &mut Self;
}

impl EmbedExtension for CreateEmbed {
    fn default_color(&mut self) -> &mut Self {
        self.color(serenity::utils::Color::from_rgb(242, 181, 37))
    }

    fn warning_color(&mut self) -> &mut Self {
        self.color(serenity::utils::Color::ORANGE)
    }

    fn success_color(&mut self) -> &mut Self {
        self.color(serenity::utils::Color::DARK_GREEN)
    }

    fn danger_color(&mut self) -> &mut Self {
        self.color(serenity::utils::Color::RED)
    }

    fn normal_embed<D: ToString>(&mut self, text: D) -> &mut Self {
        self.default_color()
            .description(text)
    }

    fn error_embed<D: ToString>(&mut self, text: D) -> &mut Self {
        self.color(serenity::utils::Color::RED).description(text)
    }
}
