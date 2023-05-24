use std::io::prelude::*;
use std::net::TcpStream;

fn get_page(host: &String, path: &String) -> Result<Vec<char>, std::io::Error> {
    println!("{}, {}", host, path);
    let mut read_buffer: Vec<u8> = Vec::new();

    let mut stream = TcpStream::connect(host)?;

    println!("Connected");
    // Send magic word to server
    let magic_string = path.clone() + "\r\n";
    stream.write(magic_string.as_bytes())?;
    println!("Written to host");

    // Read everything and load to buffer
    stream.read_to_end(&mut read_buffer)?;
    println!("Read from host");
    return Ok(read_buffer.iter().map(|&x| x as char).collect::<Vec<char>>());
}
