use std::{env, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("auth_descriptor.bin"))
        .compile_protos(&["src/auth.proto"], &["src"])
        .unwrap();

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("users_descriptor.bin"))
        .compile_protos(&["src/users.proto"], &["src"])
        .unwrap();
}
