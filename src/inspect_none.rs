//! The [`InspectNone`] convenience trait for [`Option`]s

pub trait InspectNone<T> {
    #[must_use]
    fn inspect_none<F: FnOnce()>(self, f: F) -> Self;
}

impl<T> InspectNone<T> for Option<T> {

    /// Calls a function if [`None`].
    ///
    /// Returns the original option.
    ///
    /// # Examples
    ///
    /// ```
    /// use treats::InspectNone;
    ///
    /// fn get_last_initial(name: &str) -> Option<char> {
    ///     let (_, last_name) = name.rsplit_once(' ')
    ///         .inspect_none(|| eprintln!("Name contains no spaces"))?;
    ///
    ///     last_name.chars().next()
    ///         .inspect_none(|| eprintln!("Missing first letter of last name"))
    /// }
    /// ```
    #[inline]
    fn inspect_none<F: FnOnce()>(self, f: F) -> Self {
        if self.is_none() {
            f();
        }

        self
    }
}

impl<T> InspectNone<T> for &Option<T> {
    /// Calls a function if [`None`].
    ///
    /// Returns the original option.
    ///
    /// # Examples
    ///
    /// ```
    /// use treats::InspectNone;
    ///
    /// fn get_last_initial(name: &str) -> Option<char> {
    ///     let (_, last_name) = name.rsplit_once(' ')
    ///         .inspect_none(|| eprintln!("Name contains no spaces"))?;
    ///
    ///     last_name.chars().next()
    ///         .inspect_none(|| eprintln!("Missing first letter of last name"))
    /// }
    /// ```
    #[inline]
    fn inspect_none<F: FnOnce()>(self, f: F) -> Self {
        if self.is_none() {
            f();
        }
        self
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn option_inspect_none() {
        let mut inspected = false;
        let optional_number: Option<u8> = Some(42);
        let _ = optional_number.inspect_none(|| inspected = true);
        assert!(!inspected);

        let mut inspected = false;
        let optional_number: Option<u8> = None;
        let _ = optional_number.inspect_none(|| inspected = true);
        assert!(inspected);
    }
}
