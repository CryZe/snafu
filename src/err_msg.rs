use std::error::Error;
use std::fmt;
use super::IntoError;

/// TODO:
pub struct ErrorMessage {
    message: String,
    source: Option<Box<dyn std::error::Error>>,
}

impl ErrorMessage {
    /// TODO:
    pub fn without_source(message: String) -> Self {
        Self { message, source: None }
    }

    /// TODO:
    pub fn with_source(message: String, source: Box<dyn Error>) -> Self {
        Self { message, source: Some(source) }
    }
}

impl fmt::Debug for ErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.message, f)
    }
}

impl std::error::Error for ErrorMessage {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        if let Some(source) = &self.source {
            let source: &(dyn std::error::Error + 'static) = source.as_ref();
            Some(source)
        } else {
            None
        }
    }
}
