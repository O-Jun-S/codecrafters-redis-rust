use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};


fn handle_stream(mut stream: TcpStream) {
    loop {
        let mut buf = [0; 512];
        let res = stream.read(&mut buf).unwrap();

        if res == 0 {
            println!("Connection closed.");
            break;
        }

        stream.write(b"+PONG\r\n").unwrap();
    }
}


fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
         match stream {
             Ok(s) => {
                 println!("accepted new connection");
                 handle_stream(s);
             }
             Err(e) => {
                 println!("error: {}", e);
             }
         }
    }
}

