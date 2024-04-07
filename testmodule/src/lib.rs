use std::collections::HashMap;
use lazy_static::lazy_static;

pub mod os_module {
    include!("os_module.rs");
}

pub mod dispatch {
    include!("dispatch.rs");
}

type DispatchFn = fn(Vec<u8>) -> Vec<u8>;

fn dirlist(serialized_args: Vec<u8>) -> Vec<u8> {
    Vec::new()
}

#[no_mangle]
pub extern fn dispatch(message: dispatch::DispatchMessage) -> Vec<u8> {
    // let mut FUNCTION_MAP: HashMap<&str, Box<DispatchFn>> = HashMap::new();

    // FUNCTION_MAP.insert("758d227f-27e0-4406-b27e-cf9976948109", Box::new(dirlist));

    let FUNCTION_MAP: HashMap<&str, DispatchFn> = HashMap::from([
        ("758d227f-27e0-4406-b27e-cf9976948109", dirlist as DispatchFn)
    ]);

    Vec::new()
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use os_module::DirectoryListing;

    use super::*;

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
