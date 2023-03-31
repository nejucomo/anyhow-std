// BUGS: Many tests use unix-specific paths, primarily by assuming "/" exists as a directory.

use crate::PathAnyhow;
use std::path::Path;

#[test]
fn to_str_utf8() -> anyhow::Result<()> {
    let path = Path::new("/foo/bar.txt");
    assert_eq!("/foo/bar.txt", path.to_str_anyhow()?);
    Ok(())
}

#[cfg(target_family = "unix")]
#[test]
fn to_str_invalid_utf8() -> anyhow::Result<()> {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;

    let path = Path::new(OsStr::from_bytes(b"\x81\xff"));
    assert_error_desc_eq(
        path.to_str_anyhow(),
        r#"while processing path "\x81\xFF": invalid UTF8"#,
    );
    Ok(())
}

#[test]
fn parent_non_root() -> anyhow::Result<()> {
    let path = Path::new("/foo/bar.txt");
    let expected = Path::new("/foo/");
    assert_eq!(expected, path.parent_anyhow()?);
    Ok(())
}

#[test]
fn parent_root() -> anyhow::Result<()> {
    let path = Path::new("/");
    assert_error_desc_eq(
        path.parent_anyhow(),
        r#"while processing path "/": expected parent directory"#,
    );
    Ok(())
}

#[test]
fn file_name_present() -> anyhow::Result<()> {
    let path = Path::new("/foo/bar.txt");
    assert_eq!("bar.txt", path.file_name_anyhow()?);
    Ok(())
}

#[test]
fn file_name_missing() -> anyhow::Result<()> {
    let path = Path::new("/foo/..");
    assert_error_desc_eq(
        path.file_name_anyhow(),
        r#"while processing path "/foo/..": missing expected filename"#,
    );
    Ok(())
}

#[test]
fn strip_prefix_ok() -> anyhow::Result<()> {
    let path = Path::new("/foo/bar/quz.txt");
    let expected = Path::new("bar/quz.txt");
    assert_eq!(expected, path.strip_prefix_anyhow("/foo")?);
    Ok(())
}

#[test]
fn strip_prefix_err() -> anyhow::Result<()> {
    let path = Path::new("/foo/bar/quz.txt");
    assert_error_desc_eq(
        path.strip_prefix_anyhow("/bananas"),
        r#"while processing path "/foo/bar/quz.txt": with prefix "/bananas": prefix not found"#,
    );
    Ok(())
}

#[test]
fn file_stem_present() -> anyhow::Result<()> {
    let path = Path::new("/foo/bar.txt");
    assert_eq!("bar", path.file_stem_anyhow()?);
    Ok(())
}

#[test]
fn file_stem_missing() -> anyhow::Result<()> {
    let path = Path::new("/foo/bar");
    assert_eq!("bar", path.file_stem_anyhow()?);
    Ok(())
}

#[test]
fn file_stem_without_name() -> anyhow::Result<()> {
    let path = Path::new("/foo/..");
    assert_error_desc_eq(
        path.file_stem_anyhow(),
        r#"while processing path "/foo/..": missing expected filename"#,
    );
    Ok(())
}

#[test]
fn extension_ok() -> anyhow::Result<()> {
    let path = Path::new("/foo/bar.txt");
    assert_eq!("txt", path.extension_anyhow()?);
    Ok(())
}

#[test]
fn extension_missing_filename() -> anyhow::Result<()> {
    let path = Path::new("/foo/..");
    assert_error_desc_eq(
        path.extension_anyhow(),
        r#"while processing path "/foo/..": missing expected extension"#,
    );
    Ok(())
}

#[test]
fn extension_missing_extension() -> anyhow::Result<()> {
    let path = Path::new("/foo/bar");
    assert_error_desc_eq(
        path.extension_anyhow(),
        r#"while processing path "/foo/bar": missing expected extension"#,
    );
    Ok(())
}

#[test]
fn extension_of_dot_file() -> anyhow::Result<()> {
    let path = Path::new("/foo/.bar");
    assert_error_desc_eq(
        path.extension_anyhow(),
        r#"while processing path "/foo/.bar": missing expected extension"#,
    );
    Ok(())
}

#[test]
fn metadata_ok() -> anyhow::Result<()> {
    let path = Path::new("/");
    assert!(path.metadata_anyhow().is_ok());
    Ok(())
}

#[test]
fn metadata_missing() -> anyhow::Result<()> {
    let path = Path::new("/this/path/should/not/exist");
    assert_error_desc_eq(
        path.metadata_anyhow(),
        // BUG: This error message is platform specific:
        r#"while processing path "/this/path/should/not/exist": No such file or directory (os error 2)"#,
    );
    Ok(())
}

#[test]
fn symlink_metadata_ok() -> anyhow::Result<()> {
    let path = Path::new("/");
    assert!(path.symlink_metadata_anyhow().ok().is_some());
    Ok(())
}

#[test]
fn symlink_metadata_missing() -> anyhow::Result<()> {
    let path = Path::new("/this/path/should/not/exist");
    assert_error_desc_eq(
        path.symlink_metadata_anyhow(),
        // BUG: This error message is platform specific:
        r#"while processing path "/this/path/should/not/exist": No such file or directory (os error 2)"#,
    );
    Ok(())
}

#[test]
fn canonicalize_ok() -> anyhow::Result<()> {
    // BUG: Platform specific: on some platforms "/.." may not exist.
    let path = Path::new("/..");
    assert_eq!(Path::new("/"), path.canonicalize_anyhow()?);
    Ok(())
}

#[test]
fn canonicalize_missing() -> anyhow::Result<()> {
    let path = Path::new("/this/path/should/not/exist");
    assert_error_desc_eq(
        path.canonicalize_anyhow(),
        // BUG: This error message is platform specific:
        r#"while processing path "/this/path/should/not/exist": No such file or directory (os error 2)"#,
    );
    Ok(())
}

#[ignore]
#[test]
fn read_link_ok() -> anyhow::Result<()> {
    todo!(); // We need to create a symbolic link then test the target method.
}

#[test]
fn read_link_missing() -> anyhow::Result<()> {
    let path = Path::new("/this/path/should/not/exist");
    assert_error_desc_eq(
        path.read_link_anyhow(),
        // BUG: This error message is platform specific:
        r#"while processing path "/this/path/should/not/exist": No such file or directory (os error 2)"#,
    );
    Ok(())
}

#[test]
fn read_dir_ok() -> anyhow::Result<()> {
    let path = Path::new("/");
    assert!(path.read_dir_anyhow().is_ok());
    Ok(())
}

#[test]
fn read_dir_missing() -> anyhow::Result<()> {
    let path = Path::new("/this/path/should/not/exist");
    assert_error_desc_eq(
        path.read_dir_anyhow(),
        // BUG: This error message is platform specific:
        r#"while processing path "/this/path/should/not/exist": No such file or directory (os error 2)"#,
    );
    Ok(())
}

#[test]
fn copy_from_missing() -> anyhow::Result<()> {
    let from = Path::new("/this/path/should/not/exist");
    let to = Path::new("/this/path/also/should/not/exist");
    assert_error_desc_eq(
        from.copy_anyhow(to),
        // BUG: This error message is platform specific:
        &format!(
            "while copying {:?} to {:?}: No such file or directory (os error 2)",
            from.display(),
            to.display()
        ),
    );
    Ok(())
}

#[test]
fn copy_to_non_existent_directory() -> anyhow::Result<()> {
    let from = tempfile::NamedTempFile::new()?;
    let to = Path::new("/this/path/also/should/not/exist");
    assert_error_desc_eq(
        from.path().copy_anyhow(to),
        // BUG: This error message is platform specific:
        &format!(
            "while copying {:?} to {:?}: No such file or directory (os error 2)",
            from.path().display(),
            to.display(),
        ),
    );
    Ok(())
}

#[test]
fn create_dir_within_non_existent_directory() -> anyhow::Result<()> {
    let path = Path::new("/this/path/also/should/not/exist");
    assert_error_desc_eq(
        path.create_dir_anyhow(),
        // BUG: This error message is platform specific:
        &format!(
            "while processing path {:?}: No such file or directory (os error 2)",
            path.display(),
        ),
    );
    Ok(())
}

#[test]
fn create_dir_all_permission_denied() -> anyhow::Result<()> {
    let dir = tempfile::TempDir::new()?;
    dir.path().set_readonly_anyhow(true)?;

    let path = dir.path().join("foo").join("bar");
    assert_error_desc_eq(
        path.create_dir_all_anyhow(),
        // BUG: This error message is platform specific:
        &format!(
            "while processing path {:?}: Permission denied (os error 13)",
            path.display(),
        ),
    );
    Ok(())
}

#[test]
fn hard_link_permission_error() -> anyhow::Result<()> {
    let dir = tempfile::TempDir::new()?;
    let path = dir.path().join("original");
    std::fs::write(&path, b"hello world")?;
    dir.path().set_readonly_anyhow(true)?;
    let link = dir.path().join("link");
    assert_error_desc_eq(
        path.hard_link_anyhow(&link),
        // BUG: This error message is platform specific:
        &format!(
            "while hard-linking {:?} to {:?}: Permission denied (os error 13)",
            path.display(),
            link.display(),
        ),
    );
    Ok(())
}

#[test]
fn read_missing() -> anyhow::Result<()> {
    let path = Path::new("/this/path/should/not/exist");
    assert_error_desc_eq(
        path.read_anyhow(),
        // BUG: This error message is platform specific:
        r#"while processing path "/this/path/should/not/exist": No such file or directory (os error 2)"#,
    );
    Ok(())
}

#[test]
fn read_to_string_missing() -> anyhow::Result<()> {
    let path = Path::new("/this/path/should/not/exist");
    assert_error_desc_eq(
        path.read_to_string_anyhow(),
        // BUG: This error message is platform specific:
        r#"while processing path "/this/path/should/not/exist": No such file or directory (os error 2)"#,
    );
    Ok(())
}

#[test]
fn read_to_string_invalid_utf8() -> anyhow::Result<()> {
    use std::io::Write;

    let mut f = tempfile::NamedTempFile::new()?;
    f.write_all(b"not utf8: \xf3")?;
    f.flush()?;

    assert_error_desc_eq(
        f.path().read_to_string_anyhow(),
        &format!(
            "while processing path {:?}: stream did not contain valid UTF-8",
            f.path().display()
        ),
    );
    Ok(())
}

#[test]
fn remove_dir_nonexistent() -> anyhow::Result<()> {
    let path = Path::new("/this/path/should/not/exist");
    assert_error_desc_eq(
        path.remove_dir_anyhow(),
        // BUG: This error message is platform specific:
        &format!(
            "while processing path {:?}: No such file or directory (os error 2)",
            path.display(),
        ),
    );
    Ok(())
}

#[test]
fn remove_dir_all_permission_error() -> anyhow::Result<()> {
    let dir = tempfile::TempDir::new()?;
    let a = dir.path().join("a");
    let b = a.join("b");
    let c = b.join("c");
    c.create_dir_all_anyhow()?;
    b.set_readonly_anyhow(true)?;

    assert_error_desc_eq(
        a.remove_dir_all_anyhow(),
        // BUG: This error message is platform specific:
        &format!(
            "while processing path {:?}: Permission denied (os error 13)",
            a.display(),
        ),
    );
    Ok(())
}

#[test]
fn remove_file_nonexistent() -> anyhow::Result<()> {
    let path = Path::new("/this/path/should/not/exist");
    assert_error_desc_eq(
        path.remove_file_anyhow(),
        // BUG: This error message is platform specific:
        &format!(
            "while processing path {:?}: No such file or directory (os error 2)",
            path.display(),
        ),
    );
    Ok(())
}

#[test]
fn rename_permission_error() -> anyhow::Result<()> {
    let dir = tempfile::TempDir::new()?;
    let a = dir.path().join("a");
    let b = dir.path().join("b");
    a.create_dir_anyhow()?;
    dir.path().set_readonly_anyhow(true)?;

    assert_error_desc_eq(
        a.rename_anyhow(&b),
        // BUG: This error message is platform specific:
        &format!(
            "while renaming {:?} to {:?}: Permission denied (os error 13)",
            a.display(),
            b.display(),
        ),
    );
    Ok(())
}

#[test]
fn write_permission_error() -> anyhow::Result<()> {
    let dir = tempfile::TempDir::new()?;
    dir.path().set_readonly_anyhow(true)?;
    let path = dir.path().join("file");

    assert_error_desc_eq(
        path.write_anyhow("Hello World!"),
        // BUG: This error message is platform specific:
        &format!(
            "while writing to {:?}: Permission denied (os error 13)",
            path.display(),
        ),
    );
    Ok(())
}

fn assert_error_desc_eq<T>(res: anyhow::Result<T>, expected: &str) {
    let error = format!("{:#}", res.err().unwrap());
    assert_eq!(error, expected.trim_end());
}