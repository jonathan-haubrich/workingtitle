use std::env;
use std::io::Result;
use std::path::Path;

fn main() -> Result<()> {
    // setup exports
    let def_file_path = "exports.def";
    if env::var("CARGO_CFG_TARGET_ENV").unwrap() == "msvc" {
        println!("cargo::rustc-link-arg=/DEF:{}", def_file_path);
    }

    // compile proto files
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let proto_dir = Path::new(&(manifest_dir.clone())).join("..\\proto").canonicalize().unwrap();
    let protoc_path = proto_dir.join("protoc\\bin\\protoc.exe").canonicalize().unwrap();

    env::set_var("PROTOC", protoc_path);

    let proto_files: Vec<String> = proto_dir.read_dir().unwrap()
        .into_iter()
        .filter(|entry| {
            match entry {
                Ok(entry) => entry.path().is_file(),
                _ => false
            }
        }).map(|entry| {
            entry.unwrap()
                .path()
                .canonicalize().unwrap()
                .to_str().unwrap()
                .to_owned()
                .replace("\\\\?\\", "")
        }).collect();

    prost_build::Config::new()
        .out_dir(Path::new(&manifest_dir).join("src"))
        .compile_protos(proto_files.as_slice(),
        &[proto_dir.to_str().unwrap().replace("\\\\?\\","")])?;

    Ok(())
}