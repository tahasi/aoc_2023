use std::{
    env,
    path::{Path, PathBuf},
};

fn main() {
    copy_data_files_to_target_dir()
}

fn copy_data_files_to_target_dir() {
    let target_dir = get_output_path();
    let dir = std::env::current_dir().expect("get current dir");
    println!("current dir: {dir:?}");
    let dir = std::fs::read_dir("./src/puzzles").expect("failed to read puzzles directory");
    for data_entry in dir
        .map(|file_path| file_path.expect("failed to read puzzle file"))
        .filter(|file_path| file_path.file_name().to_string_lossy().ends_with(".data"))
    {
        let source = data_entry.path();
        let target =
            Path::new(&target_dir).join(source.file_name().expect("failed to read file name"));
        std::fs::copy(&source, &target).unwrap_or_else(|_| {
            panic!(
                "failed to copy '{}' to '{}'",
                source.display(),
                target.display()
            );
        });
    }
}

fn get_output_path() -> PathBuf {
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = env::var("PROFILE").unwrap();
    Path::new(&manifest_dir_string)
        .join("target")
        .join(build_type)
}
