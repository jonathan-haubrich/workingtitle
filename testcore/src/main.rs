use std::io::{self, prelude::*};
use std::net::TcpListener;
use std::os::windows::ffi::OsStrExt;

use testmodule;

use windows::core::{PCWSTR, PCSTR};
use windows::Win32::System::LibraryLoader;

use env_logger;

pub mod os_module {
    include!("os_module.rs");
}

pub mod dispatch {
    include!("dispatch.rs");
}

fn load_dll_get_dispatch() -> testmodule::DispatchFn {
    // let out_dir = std::env::var("OUT_DIR").unwrap();
    // let testmodule_dll = std::path::Path::new(&out_dir).join("..\\..\\..\\testmodule.dll");
    //println!("Path to testmodule.dll: {}", testmodule_dll.to_str().unwrap());
    let testmodule_dll = r#"C:\Users\dweller\Source\repos\workingtitle\target\debug\testmodule.dll"#;
    let testmodule_dll: Vec<u16> = std::ffi::OsString::from(testmodule_dll)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    println!("testmodule_dll: {:?}", testmodule_dll.as_slice());
    let filename = PCWSTR::from_raw(testmodule_dll.as_ptr());
    let hmodule = unsafe { LibraryLoader::LoadLibraryW(filename).unwrap() };

    let ordinal = PCSTR::from_raw(42 as *const u8);
    let farproc = unsafe { LibraryLoader::GetProcAddress(hmodule, ordinal).unwrap() };

    let dispatch: testmodule::DispatchFn = unsafe { std::mem::transmute(farproc) };

    dispatch
}

fn main() -> std::io::Result<()> {
    env_logger::init();

    // let mut stream = TcpStream::connect("127.0.0.1:4444")?;

    // let output = "Hello, world!\n";
    // let mut input = [0; 128];

    // stream.write(output.as_bytes())?;
    // stream.read(&mut input)?;

    // match String::from_utf8(Vec::from(input)) {
    //     Ok(message) => println!("Received from remote: {}", message),
    //     Err(error) => println!("Failed to convert to string: {:?}", error)
    // }
    
    let dispatch = load_dll_get_dispatch();

    let server = TcpListener::bind("127.0.0.1:4444").unwrap();

    let mut client = match server.accept() {
        Ok((client, addr)) => {
            dbg!("Accepted connection from {}:{}", addr.ip(), addr.port());
            client
        },
        Err(error) => {
            dbg!("Accept failed: {}", error);
            return Err(io::ErrorKind::Other.into());
        }
    };

    loop {
        let mut data: Vec<u8> = Vec::new();
    
        let mut data_len_bytes = usize::to_be_bytes(0);
        match client.read(&mut data_len_bytes) {
            Ok(bytes_read) => println!("Read {} bytes", bytes_read),
            Err(error) => {
                dbg!("client.read failed: {}", error);
                break
            }
        }
        dbg!("data_len bytes: {:?}", data_len_bytes);
    
        let data_len = usize::from_be_bytes(data_len_bytes);
        dbg!("Reserving {} bytes", data_len);
        data.resize(data_len, 0);

        match client.read(&mut data) {
            Ok(bytes_read) => {
                println!("Read {} bytes", bytes_read);
                if 0 == bytes_read {
                    panic!("Read 0 bytes from remote, expected {}", data_len);
                }
            }
            Err(error) => {
                dbg!("client.read failed: {}", error);
                break
            }
        }
    
        let response = dispatch(data.as_mut_ptr(), data.len());
        if response.ptr().is_null() || 0 == response.len() {
            panic!("Dispatch function failed: {:?} {}", response.ptr(), response.len());
        }
        let response = unsafe { std::slice::from_raw_parts(response.ptr(), response.len()) };

        match client.write_all(&(response.len().to_be_bytes())) {
            Ok(_) => dbg!("Successfully sent response length"),
            Err(error) => {
                dbg!("Failed to send response: {}", error);
                break
            }
        };

        match client.write_all(&response) {
            Ok(_) => dbg!("Successfully sent response"),
            Err(error) => {
                dbg!("Failed to send response: {}", error);
                break
            }
        };
    }

    Ok(())
}
