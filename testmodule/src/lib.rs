use std::{
    collections::VecDeque, fmt::{self, Display, Formatter}, os::windows::fs::MetadataExt, slice};
use prost::Message;

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

impl From<Vec<u8>> for Payload {
    fn from(value: Vec<u8>) -> Self {
        let (data, len) = (value.as_ptr(), value.len());
        std::mem::forget(value);

        Self { data, len }
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
type CommandFn = fn(Vec<u8>) -> Option<Vec<u8>>;

fn handle_direntry(entry: std::fs::DirEntry) -> Option<os_module::DirectoryListingEntry> {
    let metadata = match entry.metadata() {
        Ok(metadata) => metadata,
        Err(_) => return None
    };

    Some(os_module::DirectoryListingEntry{
        path: match entry.path().to_str() {
            Some(string) => string.to_owned(),
            None => String::new()
        },
        accessed: metadata.last_access_time(),
        modified: metadata.last_write_time(),
        created: metadata.creation_time(),
        size: metadata.file_size(),
        attributes: metadata.file_attributes(),
        r#type: match metadata.file_type() {
            ft if ft.is_dir() => os_module::FileType::Directory.into(),
            ft if ft.is_file() => os_module::FileType::File.into(),
            _ => os_module::FileType::Symlink.into()
        }
    })
}


fn dirlist_get_entries(root: &os_module::DirectoryListingRequest, dir_queue: &mut VecDeque<String>, response: &mut os_module::DirectoryListingResponse) {
    while !dir_queue.is_empty() {
        if let Some(dir) = dir_queue.pop_back() {
            if let Ok(readdir) = std::fs::read_dir(dir) {
                let mut dl_direntry = os_module::DirectoryListingDirectoryEntry::default();

                for entry in readdir {
                    if let Ok(entry) = entry {
                        let dl_entry = handle_direntry(entry);

                        if let Some(dl_entry) = dl_entry {
                            if dl_entry.path == root.path {
                                dl_direntry.directory = Some(dl_entry);
                            } else {
                                
                                if dl_entry.r#type() == os_module::FileType::Directory &&
                                    root.recursive {
                                        dir_queue.push_back(dl_entry.path.to_string());
                                    }
                                    
                                dl_direntry.entries.push(dl_entry);
                            }
                        }
                    }
                }

                response.listing.push(dl_direntry);
            }
        }
    }
}

fn dirlist(serialized_args: Vec<u8>) -> Option<Vec<u8>> {
    let args = os_module::DirectoryListingRequest::decode(serialized_args.as_ref()).unwrap();
    
    let mut dir_queue: VecDeque<String> = VecDeque::from([args.path.to_string()]);
    let mut response = os_module::DirectoryListingResponse::default();

    dirlist_get_entries(&args, &mut dir_queue, &mut response);

    Some(response.encode_to_vec())
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
                Ok(response) => {
                    return response.into();
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

    let mut response = dispatch::DispatchResponse::default();

    (response.payload, response.error) = match command_fn(message.payload) {
        Some(payload) => (payload, 0),
        None => (Vec::new(), std::u32::MAX)
    };

    Ok(response.encode_to_vec())
}


#[cfg(test)]
mod tests {
    use std::{io::Read, path::Path};

    use super::*;

    #[test]
    fn test_deserialize() -> std::io::Result<()> {
        let mut fstream = std::fs::File::open("C:\\Users\\dweller\\Source\\repos\\workingtitle\\testmodule\\src\\serialized.bin")?;

        let mut contents = Vec::new();
        fstream.read_to_end(&mut contents)?;

        let dl : os_module::DirectoryListingRequest = prost::Message::decode(contents.as_ref())?;

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
