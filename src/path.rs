//! Convenience traits for [`Path`] and [`PathBuf`]s

use std::{path::{Path, PathBuf}, ffi::{OsStr, OsString}};

pub trait PathToString {
    fn to_lossy_string(&self) -> String;

    fn to_string(&self) -> Option<String>;
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
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
