#[macro_export]
macro_rules! read {
    ($filename:literal) => {{
        let mut path = std::path::PathBuf::from(file!());
        path.set_file_name($filename);
        std::fs::read_to_string(path).expect("read input file")
    }};
}
