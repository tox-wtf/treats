//! Permit errors for [`Result`]s

#![allow(clippy::missing_errors_doc)]

pub trait Permit<E> {
    #[must_use]
    fn permit<F>(self, f: F) -> Self
    where
        F: FnOnce(&E) -> bool;

    #[must_use]
    fn permit_if(self, condition: bool) -> Self;

    #[must_use]
    fn permit_all(self) -> Self;
}

impl<E> Permit<E> for Result<(), E> {
    /// Lazy error handling
    /// Lets you permit a specific error for `Result<(), E>`
    /// Note that this does *not* work for `T`
    ///
    /// **Example:**
    /// ```rust
    /// // Attempt to create a directory, but permit the case where it already exists
    /// use treats::Permit;
    ///
    /// if let Err(e) =
    ///     std::fs::create_dir("/tmp/dir").permit(|e| e.kind() == std::io::ErrorKind::AlreadyExists)
    /// {
    ///     // If a different error exists, handle it as usual
    ///     eprintln!("Failed to create /tmp/dir: {e}")
    /// }
    /// ```
    ///
    /// You can chain this
    #[inline]
    fn permit<F>(self, f: F) -> Self
    where
        F: FnOnce(&E) -> bool,
    {
        match self {
            Ok(()) => Ok(()),             // if result is ok, return Ok(())
            Err(ref e) if f(e) => Ok(()), // permit the error and return Ok(())
            Err(e) => Err(e),             // return the original error if not permitted
        }
    }

    #[inline]
    fn permit_if(self, condition: bool) -> Self {
        match self {
            Ok(()) => Ok(()),              // if result is ok, return Ok(())
            Err(_) if condition => Ok(()), // permit the error and return Ok(())
            Err(e) => Err(e),              // return the original error if not permitted
        }
    }

    #[inline]
    fn permit_all(self) -> Self { Ok(()) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, ErrorKind, Write};
    use std::string::ToString;
    use std::fs;

    fn ls(path: &str) -> anyhow::Result<()> {
        fs::read_dir(path)?;
        Ok(())
    }

    #[test]
    fn permit_an_error_and_succeed() {
        assert! {
            io::stdout()
                .flush()
                .permit(|e| e.kind() == ErrorKind::FileTooLarge)
                .is_ok()
        }
    }

    #[test]
    fn permit_an_error_and_fail() {
        assert! {
            fs::create_dir("/path/to/nonexistent/directory")
                .permit(|e| e.kind() == io::ErrorKind::PermissionDenied)
                .is_err()
        }
    }

    #[test]
    fn anyhow_permit_all() {
        assert! {
            ls("/etc/sudoers.d")
                .permit_all()
                .is_ok()
        }
    }

    #[test]
    fn anyhow_permit_if_false() {
        // NOTE: This is not how you wanna use permit
        assert! {
            ls("/etc/sudoers.d")
                .permit_if(false)
                .is_err()
        }
    }

    #[test]
    fn anyhow_succeed_after_permitting() {
        assert! {
            ls("/root")
                .permit(|e| e.to_string().contains("Permission"))
                .is_ok()
        }
    }

    #[test]
    fn anyhow_fail_after_permitting() {
        assert! {
            ls("/etc/sudoers.d")
                .permit(|e| e.to_string().contains("stuff it doesn't contain"))
                .is_err()
        }
    }

    #[test]
    fn anyhow_succeed() {
        assert! {
            ls(".")
                .permit(|e| e.to_string().contains("there shouldn't be an error"))
                .is_ok()
        }
    }

    #[test]
    fn chain_and_succeed() {
        assert! {
            fs::create_dir("/test")
                .permit(|e| e.kind() == io::ErrorKind::AlreadyExists)
                .permit(|e| e.kind() == io::ErrorKind::PermissionDenied)
                .is_ok()
        }
    }
}
