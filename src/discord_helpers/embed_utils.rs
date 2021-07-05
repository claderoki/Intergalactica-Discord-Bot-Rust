use serenity::builder::CreateEmbed;

pub trait EmbedExtension {
    fn priced_embed(&mut self, text: &str, cost: i32) -> &mut Self;
    fn normal_embed(&mut self, text: &str) -> &mut Self;
    fn error_embed(&mut self, text: &str) -> &mut Self;
}

impl EmbedExtension for CreateEmbed {
    fn priced_embed(&mut self, text: &str, _cost: i32) -> &mut Self {
        self.normal_embed(text)
    }

    fn normal_embed(&mut self, text: &str) -> &mut Self {
        self.color(serenity::utils::Color::from_rgb(242, 181, 37))
            .description(text)
    }

    fn error_embed(&mut self, text: &str) -> &mut Self {
        self.color(serenity::utils::Color::RED).description(text)
    }
}
