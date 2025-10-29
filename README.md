# Treats

**A collection of utility traits for Rust's standard library**

## Traits
- `PathToString` adds methods `to_string()` and `to_lossy_string()` for the
  following traits:
    - `Path`
    - `PathBuf`
    - `OsStr`
    - `OsString`
- `OptionInspectNone` adds an `inspect_none()` method to `Option` that works
  similarly to the `inspect_err()` method on `Result`
