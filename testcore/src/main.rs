use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:4444")?;

    let output = "Hello, world!\n";
    let mut input = [0; 128];

    stream.write(output.as_bytes())?;
    stream.read(&mut input)?;

    match String::from_utf8(Vec::from(input)) {
        Ok(message) => println!("Received from remote: {}", message),
        Err(error) => println!("Failed to convert to string: {:?}", error)
    }


    Ok(())
}
