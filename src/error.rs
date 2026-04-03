#[derive(Debug)]
pub enum Error {
    Frozen,
    NotFound(String),
    Message(String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Frozen => write!(f, "configuration is frozen"),
            Error::Message(msg) => write!(f, "{msg}"),
            Error::NotFound(key) => {
                write!(f, "missing configuration field: {key:?}")
            }
        }
    }
}

impl serde_core::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::Message(msg.to_string())
    }

    fn missing_field(field: &'static str) -> Self {
        Self::NotFound(field.into())
    }
}

impl serde_core::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::Message(msg.to_string())
    }
}
