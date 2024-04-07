use std::io::Result;
fn main() -> Result<()> {
    let def_file_path = r#"exports.def"#;
    if std::env::var("CARGO_CFG_TARGET_ENV").unwrap() == "msvc" {
        println!("cargo::rustc-link-arg=/DEF:{}", def_file_path);
    }

    std::env::set_var("PROTOC", r#"C:\Users\dweller\Source\repos\workingtitle\proto\protoc\bin\protoc.exe"#);
    prost_build::compile_protos(&["C:\\Users\\dweller\\Source\\repos\\workingtitle\\proto\\os_module.proto"], &["C:\\Users\\dweller\\Source\\repos\\workingtitle\\proto"])?;
    Ok(())
}