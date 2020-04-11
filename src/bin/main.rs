use rust_chat_server::ThreadPool;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

fn main(){
    let listener = TcpListener::bind("127.0.0.1:6001").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));



    //stream.write(response.as_bytes()).unwrap();
    //stream.flush().unwrap();
}
