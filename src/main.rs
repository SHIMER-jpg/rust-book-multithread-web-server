/*
Learn a bit about TCP and HTTP.
Listen for TCP connections on a socket.
Parse a small number of HTTP requests.
Create a proper HTTP response.
Improve the throughput of our server with a thread pool.
*/
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection stablished");
    }
}


