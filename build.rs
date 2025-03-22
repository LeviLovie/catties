use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is not set"));
    let dest_dir = out_dir
        .ancestors()
        .nth(3) // Navigate up to target/{PROFILE}
        .expect("Failed to determine target directory")
        .join("banks");

    let src_dir = PathBuf::from("fmod/Build/Desktop/");

    if !src_dir.exists() {
        panic!("Source directory {:?} does not exist", src_dir);
    }

    fs::create_dir_all(&dest_dir).expect("Failed to create destination directory");

    for entry in fs::read_dir(&src_dir).expect("Failed to read source directory") {
        let entry = entry.expect("Failed to read directory entry");
        let src_path = entry.path();
        let dest_path = dest_dir.join(entry.file_name());

        fs::copy(&src_path, &dest_path).expect("Failed to copy file");
    }

    println!("cargo:rerun-if-changed=assets/banks");
}
