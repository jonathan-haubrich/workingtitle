use std::{
    fmt::{self, Display, Formatter},
    ffi::OsString,
    os::windows::ffi::OsStringExt,
    slice};
use prost::Message;
use windows::{core::*, Win32::{Foundation::ERROR_NO_MORE_FILES, Storage::FileSystem::*}};

#[repr(C)]
pub struct Payload {
    data: *const u8,
    len: usize
}

impl Payload {
    fn new(data: *const u8, len: usize) -> Self {
        Self{data, len}
    }

    pub fn ptr(&self) -> *const u8 {
        self.data
    }

    pub fn len(&self) -> usize {
        self.len
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

pub type DispatchFn = extern "C" fn(data: *const u8, len: usize) -> Payload;
type CommandFn = fn(Vec<u8>) -> Vec<u8>;

fn dirlist(serialized_args: Vec<u8>) -> Vec<u8> {
    let args = os_module::DirectoryListing::decode(serialized_args.as_ref()).unwrap();
    println!("args.path: {}", args.path);
    let mut listing_data = String::new();

    let mut find_data = WIN32_FIND_DATAW::default();

    let dir = w!("C:\\Windows\\System32\\*");
    let find_handle = unsafe { FindFirstFileW(dir,&mut find_data) };

    match find_handle {
        Ok(handle) => {
            let filename = OsString::from_wide(&find_data.cFileName);

            listing_data.push_str(filename.to_str().unwrap());
            listing_data.push('\n');

            loop {
                
                find_data = WIN32_FIND_DATAW::default();
                let result = unsafe { FindNextFileW(handle, &mut find_data) };
                
                match result {
                    Ok(_) => {
                        let filename = OsString::from_wide(&find_data.cFileName);
                        listing_data.push_str(filename.to_str().unwrap());
                        listing_data.push('\n');
                    },
                    Err(error) => {
                        if error == ERROR_NO_MORE_FILES.into() {
                            eprintln!("FindNextFile finished");
                        } else {
                            eprintln!("FindNextFileW failed: {}", error);
                        }
                        break;
                    }
                }


            }
        },
        Err(error) => panic!("FindFirstFileW failed: {}", error)
    }

    listing_data.into_bytes()
}

#[no_mangle]
pub extern "C" fn dispatch(data: *const u8, len: usize) -> Payload {
    println!("data: {:?} len: {}", data, len);
    let bytes = unsafe{ slice::from_raw_parts(data, len) };
    let message = Vec::from(bytes);
    let message = dispatch::DispatchMessage::decode(message.as_ref());

    match message {
        Ok(message) => {
            match dispatch_internal(message) {
                Ok(mut response) => {
                    let ptr = response.as_mut_ptr();
                    let len = response.len();
                    std::mem::forget(response);
                    return Payload::new(ptr, len);
                },
                Err(error) => println!("Dispatch failed: {}", error)
            }
        }
        Err(error) => println!("Decode failed: {}", error)
    };

    Payload::default()
}

fn dispatch_internal(message: dispatch::DispatchMessage) -> std::result::Result<Vec<u8>, DispatchError> {
    let command_fn: CommandFn;

    match message.function_id.as_str() {
        "758d227f-27e0-4406-b27e-cf9976948109" => command_fn = dirlist as CommandFn,
        _ => return Err(DispatchError::FunctionNotFound(message.function_id))
    }

    Ok(command_fn(message.payload))
}


#[cfg(test)]
mod tests {
    use std::{io::Read, path::Path};

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
    fn test_dispatch_internal() -> std::io::Result<()> {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let tests_dir = Path::new(&manifest_dir).join("tests");

        let mut fs = std::fs::File::open(tests_dir.join("dispatch.bin"))?;

        
        let mut serialized = Vec::new();
        fs.read_to_end(&mut serialized).unwrap();
    
        let message = dispatch::DispatchMessage::decode(serialized.as_ref()).unwrap();

        if let Ok(data) = dispatch_internal(message) {
            let listing = String::from_utf8(data).unwrap();
            let listing_parts: Vec<&str> = listing.split('\n').collect();

            println!("{}", listing);
            println!("Found {} items", listing_parts.len());
        }

        Ok(())
    }

}
