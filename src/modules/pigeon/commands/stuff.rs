

struct MessageWaiter<'fut> {
    pub ctx: &'fut Context,
    pub msg: &'fut Message,
    // pub start_prompt: Option<String>,
    // pub end_prompt: Option<String>
}



impl MessageWaiter<'_> {

    pub async fn wait<T: TryFrom<String>>(&self, timeout: u64, prompt: &str) -> Result<T, &'static str> {
        let reply = &self
            .msg
            .author
            .await_reply(&self.ctx)
            .timeout(Duration::from_secs(timeout))
            .await;

        match reply {
            Some(message) => {
                let converted: std::result::Result<T, _> =
                    String::from(message.content.as_str()).try_into();
                return match converted {
                    Ok(value) => Ok(value),
                    Err(_) => Err("Failed to convert"),
                };
            }
            None => {
                return Err("Timed out".into());
            }
        };
    }
}
