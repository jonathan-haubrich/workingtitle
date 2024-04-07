
pub mod os_module {
    include!(concat!(env!("OUT_DIR"), "/os_module.rs"));
}

#[no_mangle]
pub extern fn dispatch(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use os_module::DirectoryListing;

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_deserialize() -> std::io::Result<()> {
        let mut fstream = std::fs::File::open("C:\\Users\\dweller\\Source\\repos\\workingtitle\\testmodule\\src\\serialized.bin")?;

        let mut contents = Vec::new();
        fstream.read_to_end(&mut contents)?;

        let dl : DirectoryListing = prost::Message::decode(contents.as_ref())?;

        assert_eq!(dl.path, r#"C:\Users\dweller\testpath"#);
        assert_eq!(dl.recursive, true);
        Ok(())
    }
}
