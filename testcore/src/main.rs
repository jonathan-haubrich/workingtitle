use std::io::prelude::*;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::os::windows::ffi::OsStrExt;
use std::str::FromStr;

use testmodule;

use windows::core::{PCWSTR, PWSTR, PCSTR};
use windows::Win32::System::LibraryLoader;

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

    let mut client: TcpStream;
    let addr: SocketAddr;
    match server.accept() {
        Ok((mut _client, _addr)) => {
            (client, addr) = (_client, _addr);
            println!("Accepted connection from {}:{}", addr.ip(), addr.port());
        },
        Err(error) => {
            println!("Accept failed: {}", error);
            return Err(error);
        }
    }

    let mut data: Vec<u8> = Vec::new();
    let _ = client.read_to_end(&mut data).unwrap();

    let (ptr, size) = (data.as_mut_ptr(), data.len());
    let response = dispatch(ptr, size);

    let response = unsafe { std::slice::from_raw_parts(response.ptr(), response.len()) };
    
    match client.write_all(response) {
        Ok(_) => println!("Successfully sent response"),
        Err(error) => println!("Failed to send response: {}", error)
    };

    Ok(())
}
