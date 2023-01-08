use std::io::Write;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
         match stream {
             Ok(mut s) => {
                 println!("accepted new connection");
                 s.write(b"+PONG\r\n").unwrap();
             }
             Err(e) => {
                 println!("error: {}", e);
             }
         }
    }
}

