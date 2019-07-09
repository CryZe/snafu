use std::error::Error;
use std::fmt;
use std::borrow::Cow;

/// TODO:
pub struct ErrorMessage {
    message: Cow<'static, str>,
    source: Option<Box<dyn std::error::Error>>,
}

impl ErrorMessage {
    /// TODO:
    pub fn new(message: impl Into<Cow<'static, str>>) -> Self {
        Self { message: message.into(), source: None }
    }

    /// TODO:
    pub fn with_source(mut self, source: impl Error + 'static) -> Self {
        self.source = Some(Box::new(source));
        self
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
