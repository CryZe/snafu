use std::error::Error;
use std::fmt;

/// TODO:
pub struct LazyMessage<F>(pub F);

impl<F: Fn(&mut fmt::Formatter<'_>) -> fmt::Result> fmt::Display for LazyMessage<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (self.0)(f)
    }
}

/// TODO:
pub struct ErrorMessage<T> {
    message: T,
    source: Option<Box<dyn std::error::Error>>,
}

impl<T: fmt::Display> ErrorMessage<T> {
    /// TODO:
    pub fn new(message: T) -> Self {
        Self {
            message,
            source: None,
        }
    }

    /// TODO:
    pub fn with_source(mut self, source: impl Error + 'static) -> Self {
        self.source = Some(Box::new(source));
        self
    }
}

impl<T: fmt::Display> fmt::Debug for ErrorMessage<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl<T: fmt::Display> fmt::Display for ErrorMessage<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.message, f)
    }
}

impl<T: fmt::Display> std::error::Error for ErrorMessage<T> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        if let Some(source) = &self.source {
            let source: &(dyn std::error::Error + 'static) = source.as_ref();
            Some(source)
        } else {
            None
        }
    }
}
