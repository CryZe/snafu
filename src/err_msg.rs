use std::error::Error;
use std::fmt;

/// TODO:
pub struct Fail(pub Box<dyn Error>);

impl fmt::Debug for Fail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for Fail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl Error for Fail {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.0.source()
    }
}

/// TODO:
pub struct LazyMessage<F>(pub F);

impl<F: Fn(&mut fmt::Formatter<'_>) -> fmt::Result> fmt::Display for LazyMessage<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (self.0)(f)
    }
}

/// TODO:
pub struct ErrorMessage<T, E> {
    message: T,
    source: Option<E>,
}

impl<T, E> ErrorMessage<T, E> {
    /// TODO:
    pub fn without_source(message: T) -> Self {
        Self {
            message,
            source: None,
        }
    }

    /// TODO:
    pub fn with_source(message: T, source: E) -> Self {
        Self {
            message,
            source: Some(source),
        }
    }
}

impl<T: fmt::Display, E> fmt::Debug for ErrorMessage<T, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl<T: fmt::Display, E> fmt::Display for ErrorMessage<T, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.message, f)
    }
}

impl<T: fmt::Display, E: Error + 'static> Error for ErrorMessage<T, E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        if let Some(source) = &self.source {
            Some(source)
        } else {
            None
        }
    }
}
