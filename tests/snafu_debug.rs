extern crate snafu;

use snafu::SnafuDebug;

#[derive(SnafuDebug)]
enum Inner {
    /// Some inner message.
    Bar,
}

#[derive(SnafuDebug)]
enum Error {
    /// No user available.
    /// You may need to specify one.
    ///
    /// Here is a more detailed description.
    MissingUser,
    /// Some outer message.
    Foo { source: Inner },
}

#[test]
fn implements_error() {
    fn check<T: std::error::Error>() {}
    check::<Error>();
    assert_eq!(
        format!("{:?}", Error::MissingUser),
        "No user available. You may need to specify one."
    );
}

#[test]
fn chain_works() {
    let err = Error::Foo { source: Inner::Bar };
    assert_eq!(
        snafu::chain(&err).map(ToString::to_string).collect::<Vec<_>>(),
        ["Some outer message.", "Some inner message."],
    );
}
