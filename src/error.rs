use std::fmt;

/// An error type for the unrecoverable situations this application bails out on.
///
/// An [`Error`] carries only a human readable message: there is no payload to
/// inspect programmatically.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    message: String,
}

impl Error {
    /// Makes a new [`Error`] with the given message.
    pub fn new<T: Into<String>>(message: T) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::new(e.to_string())
    }
}

impl From<fmt::Error> for Error {
    fn from(e: fmt::Error) -> Self {
        Self::new(e.to_string())
    }
}

impl From<nojson::JsonParseError> for Error {
    fn from(e: nojson::JsonParseError) -> Self {
        Self::new(e.to_string())
    }
}

impl From<nopng::Error> for Error {
    fn from(e: nopng::Error) -> Self {
        Self::new(e.to_string())
    }
}

/// This crate specific [`Result`](std::result::Result) type.
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_displays_message() {
        let error = Error::new("something went wrong");
        assert_eq!(error.to_string(), "something went wrong");
    }

    #[test]
    fn io_error_converts() {
        let io_error = std::io::Error::other("io boom");
        let error: Error = io_error.into();
        assert_eq!(error.to_string(), "io boom");
    }
}
