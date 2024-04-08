use std::{fmt::{self, Display, Formatter}, os::windows::ffi::OsStringExt, slice};
use prost::Message;
use windows::{core::*, Win32::Storage::FileSystem::*};

#[repr(C)]
pub struct Payload {
    data: *const u8,
    len: usize
}

impl Payload {
    fn new(data: *const u8, len: usize) -> Self {
        Self{data, len}
    }
}

impl Default for Payload {
    fn default() -> Self {
        Self { data: std::ptr::null(), len: 0 }
    }
}

#[derive(Debug)]
#[repr(C)]
pub enum DispatchError {
    FunctionNotFound(String)
}

impl Display for DispatchError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::FunctionNotFound(function) => write!(f, "Unknown function: {}", function)
        }
    }
}

impl std::error::Error for DispatchError {}

pub mod os_module {
    include!("os_module.rs");
}

pub mod dispatch {
    include!("dispatch.rs");
}

type DispatchFn = fn(Vec<u8>) -> Vec<u8>;

fn dirlist(_: Vec<u8>) -> Vec<u8> {
    //let args = os_module::DirectoryListing::decode(serialized_args.as_ref());

    //let mut listing_data: Vec<u8> = Vec::new();

    let mut find_data = WIN32_FIND_DATAW::default();

    let dir = w!("C:\\Windows\\System32\\*");
    let find_handle = unsafe { FindFirstFileW(dir,&mut find_data) };

    match find_handle {
        Ok(handle) => {
            let filename = std::ffi::OsString::from_wide(&find_data.cFileName);
            println!("Found: {}" ,filename.to_str().unwrap());

            loop {
                find_data.cFileName.fill(0u16);
                let result = unsafe { FindNextFileW(handle, &mut find_data) };
                
                match result {
                    Ok(_) => {
                        let filename = std::ffi::OsString::from_wide(&find_data.cFileName);
                        println!("Found: {}" ,filename.to_str().unwrap());
                    },
                    Err(error) => {
                        eprintln!("FindNextFileW failed: {}", error);
                        break;
                    }
                }


            }
        },
        Err(error) => panic!("FindFirstFileW failed: {}", error)
    }

    Vec::new()
}

#[no_mangle]
pub extern fn dispatch(data: *const std::ffi::c_uchar, len: usize) -> Payload {
    let bytes = unsafe{ slice::from_raw_parts(data, len) };
    let message = Vec::from(bytes);
    let message = dispatch::DispatchMessage::decode(message.as_ref());

    match message {
        Ok(message) => {
            match dispatch_internal(message) {
                Ok(response) => Payload::new(response.as_ptr(), response.len()),
                Err(error) => panic!("Dispatch failed: {}", error)
            }
        }
        Err(error) => panic!("Decode failed: {}", error)
    };

    Payload::default()
}

fn dispatch_internal(message: dispatch::DispatchMessage) -> std::result::Result<Vec<u8>, DispatchError> {
    let dispatch_fn: DispatchFn;

    match message.function_id.as_str() {
        "758d227f-27e0-4406-b27e-cf9976948109" => dispatch_fn = dirlist as DispatchFn,
        _ => return Err(DispatchError::FunctionNotFound(message.function_id))
    }

    Ok(dispatch_fn(message.payload))
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

    #[test]
    fn test_dirlist() -> std::io::Result<()> {
        dirlist(Vec::new());

        Ok(())
    }
}
