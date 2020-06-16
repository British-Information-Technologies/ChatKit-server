use std::net::TcpStream;
use std::io::Write;

pub fn transmit_data(mut stream: &TcpStream, data: &str){
    println!("Transmitting...");
    println!("data: {}",data);

    stream.write(data.to_string().as_bytes()).unwrap();
    stream.flush().unwrap();
}
