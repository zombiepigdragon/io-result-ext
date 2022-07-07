//! Provides [`IoResultExt`], an extension trait for shortening common patterns with [`std::io::Result`].
#![deny(missing_docs)]

use std::io;

/// A collection of helper methods for [`std::io::Result`].
pub trait IoResultExt<T>: sealed::Sealed + Sized {
    /// Map potentially [not found](`std::io::ErrorKind::NotFound`) entities to an [`Option`].
    ///
    /// ```
    /// # use std::fs::File;
    /// # use io_result_ext::IoResultExt;
    /// if let Some(config) = File::open("config.json").optional()? {
    ///     // do something...
    /// }
    /// # Ok::<(), std::io::Error>(())
    /// ```
    fn optional(self) -> io::Result<Option<T>>;

    /// Mask [`ErrorKind::AlreadyExists`](`std::io::ErrorKind::AlreadyExists`) errors.
    ///
    /// ```
    /// # use std::fs;
    /// # use io_result_ext::IoResultExt;
    /// # fn test() -> std::io::Result<()> {
    /// fs::create_dir("cache").can_exist()?;
    /// // And then on a later run...
    /// fs::create_dir("cache").can_exist()?;
    /// # Ok(())
    /// # }
    /// ```
    fn can_exist(self) -> Self
    where
        T: Default;
}

impl<T> IoResultExt<T> for io::Result<T> {
    fn optional(self) -> io::Result<Option<T>> {
        match self {
            Ok(t) => Ok(Some(t)),
            Err(ref e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e),
        }
    }

    fn can_exist(self) -> Self
    where
        T: Default,
    {
        match self {
            Ok(t) => Ok(t),
            Err(ref e) if e.kind() == io::ErrorKind::AlreadyExists => Ok(T::default()),
            Err(e) => Err(e),
        }
    }
}

mod sealed {
    /// Prevent accidental implementations of other traits on external types.
    ///
    /// For more information, see the [Rust API guidelines](https://rust-lang.github.io/api-guidelines/future-proofing.html).
    pub trait Sealed {}

    impl<T> Sealed for ::std::io::Result<T> {}
}
