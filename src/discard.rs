//! The [`Discard`] convenience trait

pub trait Discard {
    fn discard(self);
}

impl<T> Discard for T {
    /// Discard any `T`.
    ///
    /// This is functionally equivalent to `let _ = ...` but is occasionally more convenient.
    ///
    /// # Examples
    /// ```
    /// use treats::Discard;
    /// use std::sync::OnceLock;
    ///
    /// static FLAG_FORCE:   OnceLock<bool> = OnceLock::new();
    /// static FLAG_VERBOSE: OnceLock<bool> = OnceLock::new();
    /// static FLAG_QUIET:   OnceLock<bool> = OnceLock::new();
    ///
    /// fn parse_args() {
    ///     for arg in std::env::args().skip(1) {
    ///         if arg.starts_with("--") {
    ///             match arg.as_str() {
    ///                 "--force"   => FLAG_FORCE.set(true).discard(),
    ///                 "--verbose" => FLAG_VERBOSE.set(true).discard(),
    ///                 "--quiet"   => FLAG_QUIET.set(true).discard(),
    ///                 _ => eprintln!("unknown flag: {arg}"),
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    #[inline]
    fn discard(self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discard() {
        let x: Result<i32, &str> = Ok(-3);

        assert_eq! {
            x.discard(),
            ()
        };

        let x: Result<i32, &str> = Err("some error");

        assert_eq! {
            x.discard(),
            ()
        };
    }
}
