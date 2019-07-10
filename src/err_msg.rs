use std::error::Error;
use std::fmt;

/// TODO:
pub struct Fail(pub Box<dyn Error>);

impl Fail {
    /// TODO:
    pub fn new(err: impl Error + 'static) -> Self {
        Self(err.into())
    }
}

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
pub struct LazyDisplay<F>(pub F);

impl<F: Fn(&mut fmt::Formatter<'_>) -> fmt::Result> fmt::Display for LazyDisplay<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (self.0)(f)
    }
}

/// TODO:
pub struct ErrorFromDisplay<T>(pub T);

impl<T: fmt::Display> fmt::Debug for ErrorFromDisplay<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl<T: fmt::Display> fmt::Display for ErrorFromDisplay<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl<T: fmt::Display> Error for ErrorFromDisplay<T> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

/// TODO:
pub struct WithSource<T, E> {
    message: T,
    source: E,
}

impl<T, E> WithSource<T, E> {
    /// TODO:
    pub fn new(message: T, source: E) -> Self {
        Self { message, source }
    }
}

impl<T: fmt::Display, E> fmt::Debug for WithSource<T, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl<T: fmt::Display, E> fmt::Display for WithSource<T, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.message, f)
    }
}

impl<T: fmt::Display, E: Error + 'static> Error for WithSource<T, E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}
