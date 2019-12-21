use std::path::PathBuf;

macro_rules! relpath {
    ($filename:literal) => {{
        let mut path = PathBuf::from(file!());
        path.set_file_name($filename);
        path
    }};
}

#[test]
fn dir_smoke_test() {
    assert_eq!(relpath!("foo.txt").to_string_lossy(), "files/foo.txt");
}
