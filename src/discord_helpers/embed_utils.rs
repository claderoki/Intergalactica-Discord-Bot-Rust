use serenity::builder::CreateEmbed;

pub trait EmbedExtension {
    fn normal_embed<D>(&mut self, text: D) -> &mut Self where D: ToString;
    fn error_embed<D>(&mut self, text: D) -> &mut Self where D: ToString;
}

impl EmbedExtension for CreateEmbed {

    fn normal_embed<D: ToString>(&mut self, text: D) -> &mut Self {
        self.color(serenity::utils::Color::from_rgb(242, 181, 37))
            .description(text)
    }

    fn error_embed<D: ToString>(&mut self, text: D) -> &mut Self {
        self.color(serenity::utils::Color::RED).description(text)
    }
}
