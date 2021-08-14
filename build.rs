use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("muscle_exercises.bin"))
        .compile(&["proto/muscle_exercises.proto"], &["proto"])
        .unwrap();
}