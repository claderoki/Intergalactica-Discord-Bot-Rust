
pub trait UserInput {}
impl UserInput for Interaction {}
impl UserInput for String {}

pub trait WaitableValidator {
    fn validate(&self);
}

#[async_trait]
pub trait Waitable<U>
where
    U: UserInput,
{
    async fn wait<T>(&self) -> T
    where
        T: TryFrom<String> + TryFrom<Interaction>;

    fn parse<T>(&self, input: U) -> Result<T, &'static str>
    where
        T: TryFrom<String> + TryFrom<Interaction>;

    fn validate<T, V>(&self, input: U, _validator: V) -> Result<T, &'static str>
    where
        T: TryFrom<String> + TryFrom<Interaction>,
        V: WaitableValidator,
    {
        let parsed = self.parse::<T>(input)?;
        Ok(parsed)
    }
}

struct MessageWaiter;
#[async_trait]
impl Waitable<String> for MessageWaiter {
    async fn wait<T>(&self) -> T {
        todo!()
    }

    fn parse<T>(&self, input: String) -> Result<T, &'static str>
    where
        T: TryFrom<String>,
    {
        let parsed: Result<T, _> = input.try_into();
        parsed.map_err(|_| "Could not parse value")
    }
}

struct InteractionWaiter;
#[async_trait]
impl Waitable<Interaction> for InteractionWaiter {
    async fn wait<T>(&self) -> T {
        todo!()
    }

    fn parse<T>(&self, input: Interaction) -> Result<T, &'static str>
    where
        T: TryFrom<Interaction>,
    {
        let parsed: Result<T, _> = input.try_into();
        parsed.map_err(|_| "Could not parse value")
    }
}

// trait MessageWaitable {
//     fn validate<T>(&self, input: U) -> Result<T, &'static str> where T: From<String> + From<Interaction> {
//         self.parse::<T>(input)
//     }
//     fn wait<T>(&self) -> T {
//         todo!()
//     }

//     fn parse<T>(&self, input: String) -> Result<T, &'static str> where T: TryFrom<String> {
//         let parsed: Result<T, _> = input.try_into();
//         parsed.map_err(|_|"Could not parse value")
//     }
// }

// struct MessageWaiter;
// impl MessageWaitable for
