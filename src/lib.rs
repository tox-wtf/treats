#[cfg(feature = "path_to_string")]
use std::{path::{Path, PathBuf}, ffi::{OsStr, OsString}};

#[cfg(feature = "path_to_string")]
pub trait PathToString {
    fn to_lossy_string(&self) -> String;

    fn to_string(&self) -> Option<String>;
}

#[cfg(feature = "path_to_string")]
impl PathToString for Path {
    /// Converts a [`Path`] into a lossy [`String`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    /// use treats::PathToString;
    ///
    /// fn path_len<P: AsRef<Path>>(path: P) -> usize {
    ///     path.as_ref().to_lossy_string().len()
    /// }
    /// ```
    #[inline]
    fn to_lossy_string(&self) -> String {
        self.to_string_lossy().to_string()
    }

    /// Yields a [`String`] if the [`Path`] is valid unicode.
    #[inline]
    fn to_string(&self) -> Option<String> {
        self.to_str().map(ToString::to_string)
    }
}

#[cfg(feature = "path_to_string")]
impl PathToString for PathBuf {
    /// Converts a [`PathBuf`] into a lossy [`String`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    /// use treats::PathToString;
    ///
    /// fn path_len<P: AsRef<Path>>(path: P) -> usize {
    ///     path.as_ref().to_lossy_string().len()
    /// }
    /// ```
    #[inline]
    fn to_lossy_string(&self) -> String {
        self.to_string_lossy().to_string()
    }

    /// Yields a [`String`] if the [`PathBuf`] is valid unicode.
    #[inline]
    fn to_string(&self) -> Option<String> {
        self.to_str().map(ToString::to_string)
    }
}

#[cfg(feature = "path_to_string")]
impl PathToString for OsStr {
    /// Converts an [`OsStr`] into a lossy [`String`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    /// use treats::PathToString;
    ///
    /// fn basename<P: AsRef<Path>>(path: P) -> Option<String> {
    ///     if let Some(basename) = path.as_ref().file_name() {
    ///         return Some(basename.to_lossy_string())
    ///     }
    ///
    ///     None
    /// }
    /// ```
    #[inline]
    fn to_lossy_string(&self) -> String {
        self.to_string_lossy().to_string()
    }

    /// Yields a [`String`] if the [`OsStr`] is valid unicode.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    /// use treats::PathToString;
    ///
    /// fn basename<P: AsRef<Path>>(path: P) -> Option<String> {
    ///     if let Some(basename) = path.as_ref().file_name() {
    ///         return basename.to_string()
    ///     }
    ///
    ///     None
    /// }
    /// ```
    #[inline]
    fn to_string(&self) -> Option<String> {
        self.to_str().map(ToString::to_string)
    }
}

#[cfg(feature = "path_to_string")]
impl PathToString for OsString {
    /// Converts an [`OsString`] into a lossy [`String`].
    #[inline]
    fn to_lossy_string(&self) -> String {
        self.to_string_lossy().to_string()
    }

    /// Yields a [`String`] if the [`OsString`] is valid unicode.
    #[inline]
    fn to_string(&self) -> Option<String> {
        self.to_str().map(ToString::to_string)
    }
}


#[cfg(feature = "option_inspect_none")]
pub trait OptionInspectNone<T> {
    #[must_use]
    fn inspect_none<F: FnOnce()>(self, f: F) -> Self;
}

#[cfg(feature = "option_inspect_none")]
impl<T> OptionInspectNone<T> for Option<T> {

    /// Calls a function if [`None`].
    ///
    /// Returns the original option.
    ///
    /// # Examples
    ///
    /// ```
    /// use treats::OptionInspectNone;
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

#[cfg(feature = "option_inspect_none")]
impl<T> OptionInspectNone<T> for &Option<T> {
    /// Calls a function if [`None`].
    ///
    /// Returns the original option.
    ///
    /// # Examples
    ///
    /// ```
    /// use treats::OptionInspectNone;
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
    #[allow(unused_imports)]
    use super::*;

    #[test]
    #[cfg(feature = "option_inspect_none")]
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

    #[test]
    #[cfg(feature = "path_to_string")]
    fn path_to_string() {
        use std::path::{Path, PathBuf};

        let path_str = "/path/to/whatever";
        let path = Path::new(path_str);

        assert_eq!(path.to_lossy_string(), path.to_string_lossy().to_string());
        assert_eq!(path.to_string(), path.to_str().map(ToString::to_string));

        assert_eq!(path.to_lossy_string(), path_str);
        assert_eq!(path.to_string().unwrap(), path_str);

        let path = PathBuf::from(path_str);

        assert_eq!(path.to_lossy_string(), path.to_string_lossy().to_string());
        assert_eq!(path.to_string(), path.to_str().map(ToString::to_string));

        assert_eq!(path.to_lossy_string(), path_str);
        assert_eq!(path.to_string().unwrap(), path_str);
    }
}
