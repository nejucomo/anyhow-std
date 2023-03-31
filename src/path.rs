use anyhow::Context;
use std::ffi::OsStr;
use std::fs::{Metadata, ReadDir};
use std::path::{Path, PathBuf};

/// Extend [Path] with [anyhow] methods
pub trait PathAnyhow {
    /// Wrap [Path::to_str], providing the path as error context
    fn to_str_anyhow(&self) -> anyhow::Result<&str>;

    /// Wrap [Path::parent], providing the path as error context
    fn parent_anyhow(&self) -> anyhow::Result<&Path>;

    /// Wrap [Path::file_name], providing the path as error context
    fn file_name_anyhow(&self) -> anyhow::Result<&OsStr>;

    /// Wrap [Path::strip_prefix], providing the path and `base` as error context
    fn strip_prefix_anyhow<P>(&self, base: P) -> anyhow::Result<&Path>
    where
        P: AsRef<Path>;

    /// Wrap [Path::file_stem], providing the path as error context
    fn file_stem_anyhow(&self) -> anyhow::Result<&OsStr>;

    /// Wrap [Path::extension], providing the path as error context
    fn extension_anyhow(&self) -> anyhow::Result<&OsStr>;

    /// Wrap [Path::metadata], providing the path as error context
    fn metadata_anyhow(&self) -> anyhow::Result<Metadata>;

    /// Wrap [Path::symlink_metadata], providing the path as error context
    fn symlink_metadata_anyhow(&self) -> anyhow::Result<Metadata>;

    /// Wrap [Path::canonicalize], providing the path as error context
    fn canonicalize_anyhow(&self) -> anyhow::Result<PathBuf>;

    /// Wrap [Path::read_link], providing the path as error context
    fn read_link_anyhow(&self) -> anyhow::Result<PathBuf>;

    /// Wrap [Path::read_dir], providing the path as error context
    fn read_dir_anyhow(&self) -> anyhow::Result<ReadDir>;

    // Wrappers for std::fs:

    /// Wrap [std::fs::copy] from `self` to `to`, providing `self` and `to` as error context
    fn copy_anyhow<P>(&self, to: P) -> anyhow::Result<u64>
    where
        P: AsRef<Path>;

    /// Wrap [std::fs::create_dir], providing the path as error context
    fn create_dir_anyhow(&self) -> anyhow::Result<()>;

    /// Wrap [std::fs::create_dir_all], providing the path as error context
    fn create_dir_all_anyhow(&self) -> anyhow::Result<()>;

    /// Wrap [std::fs::hard_link], providing `self` and `link` as error context
    fn hard_link_anyhow<P>(&self, link: P) -> anyhow::Result<()>
    where
        P: AsRef<Path>;

    /// Wrap [std::fs::read], providing the path as error context
    fn read_anyhow(&self) -> anyhow::Result<Vec<u8>>;

    /// Wrap [std::fs::read_to_string], providing the path as error context
    fn read_to_string_anyhow(&self) -> anyhow::Result<String>;

    /// Wrap [std::fs::remove_dir], providing the path as error context
    fn remove_dir_anyhow(&self) -> anyhow::Result<()>;

    /// Wrap [std::fs::remove_dir_all], providing the path as error context
    fn remove_dir_all_anyhow(&self) -> anyhow::Result<()>;

    /// Wrap [std::fs::remove_file], providing the path as error context
    fn remove_file_anyhow(&self) -> anyhow::Result<()>;

    /// Wrap [std::fs::rename], providing `self` and `to` as error context
    fn rename_anyhow<P>(&self, to: P) -> anyhow::Result<()>
    where
        P: AsRef<Path>;

    /// Wrap [std::fs::set_permissions], providing the path as error context
    ///
    /// This method factors out the complexity of retrieving [std::fs::Permisisons], modifying
    /// them, and then setting them.
    fn set_readonly_anyhow(&self, readonly: bool) -> anyhow::Result<()>;
    /// Wrap [std::fs::rename], providing `self` and `to` as error context

    fn write_anyhow<C>(&self, contents: C) -> anyhow::Result<()>
    where
        C: AsRef<[u8]>;
}

macro_rules! wrap_nullary_option_method {
    ( $method:ident, $cb:expr, $ret:ty, $errordesc:expr ) => {
        fn $method(&self) -> anyhow::Result<$ret> {
            let p = self.as_ref();
            $cb(p)
                .ok_or_else(|| anyhow::Error::msg($errordesc))
                .with_context(|| format!("while processing path {:?}", p.display()))
        }
    };
}

macro_rules! wrap_nullary_result_method {
    ( $method:ident, $cb:expr, $ret:ty ) => {
        fn $method(&self) -> anyhow::Result<$ret> {
            $cb(self).with_context(|| format!("while processing path {:?}", self.display()))
        }
    };
}

impl PathAnyhow for Path {
    wrap_nullary_option_method!(to_str_anyhow, Path::to_str, &str, "invalid UTF8");

    wrap_nullary_option_method!(
        parent_anyhow,
        Path::parent,
        &Path,
        "expected parent directory"
    );

    wrap_nullary_option_method!(
        file_name_anyhow,
        Path::file_name,
        &OsStr,
        "missing expected filename"
    );

    fn strip_prefix_anyhow<Q>(&self, base: Q) -> anyhow::Result<&Path>
    where
        Q: AsRef<Path>,
    {
        let bref = base.as_ref();
        self.strip_prefix(bref)
            .with_context(|| format!("with prefix {:?}", bref.display()))
            .with_context(|| format!("while processing path {:?}", self.display()))
    }

    wrap_nullary_option_method!(
        file_stem_anyhow,
        Path::file_stem,
        &OsStr,
        "missing expected filename"
    );

    wrap_nullary_option_method!(
        extension_anyhow,
        Path::extension,
        &OsStr,
        "missing expected extension"
    );

    wrap_nullary_result_method!(metadata_anyhow, Path::metadata, Metadata);
    wrap_nullary_result_method!(symlink_metadata_anyhow, Path::symlink_metadata, Metadata);
    wrap_nullary_result_method!(canonicalize_anyhow, Path::canonicalize, PathBuf);
    wrap_nullary_result_method!(read_link_anyhow, Path::read_link, PathBuf);
    wrap_nullary_result_method!(read_dir_anyhow, Path::read_dir, ReadDir);

    fn copy_anyhow<P>(&self, to: P) -> anyhow::Result<u64>
    where
        P: AsRef<Path>,
    {
        let to = to.as_ref();
        std::fs::copy(self, to)
            .with_context(|| format!("while copying {:?} to {:?}", self.display(), to.display()))
    }

    wrap_nullary_result_method!(create_dir_anyhow, std::fs::create_dir, ());
    wrap_nullary_result_method!(create_dir_all_anyhow, std::fs::create_dir_all, ());

    fn hard_link_anyhow<P>(&self, link: P) -> anyhow::Result<()>
    where
        P: AsRef<Path>,
    {
        let link = link.as_ref();
        std::fs::hard_link(self, link).with_context(|| {
            format!(
                "while hard-linking {:?} to {:?}",
                self.display(),
                link.display()
            )
        })
    }

    wrap_nullary_result_method!(read_anyhow, std::fs::read, Vec<u8>);
    wrap_nullary_result_method!(read_to_string_anyhow, std::fs::read_to_string, String);
    wrap_nullary_result_method!(remove_dir_anyhow, std::fs::remove_dir, ());
    wrap_nullary_result_method!(remove_dir_all_anyhow, std::fs::remove_dir_all, ());
    wrap_nullary_result_method!(remove_file_anyhow, std::fs::remove_file, ());

    fn rename_anyhow<P>(&self, to: P) -> anyhow::Result<()>
    where
        P: AsRef<Path>,
    {
        let to = to.as_ref();
        std::fs::rename(self, to)
            .with_context(|| format!("while renaming {:?} to {:?}", self.display(), to.display()))
    }

    fn set_readonly_anyhow(&self, readonly: bool) -> anyhow::Result<()> {
        let mut perms = self.metadata_anyhow()?.permissions();
        perms.set_readonly(true);
        std::fs::set_permissions(self, perms)
            .with_context(|| format!("with readonly permission {:?}", readonly))
            .with_context(|| format!("while processing path {:?}", self.display()))
    }

    fn write_anyhow<C>(&self, contents: C) -> anyhow::Result<()>
    where
        C: AsRef<[u8]>,
    {
        std::fs::write(self, contents)
            .with_context(|| format!("while writing to {:?}", self.display()))
    }
}

#[cfg(test)]
mod tests;