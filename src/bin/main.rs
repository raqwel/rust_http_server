extern crate HttpServer;

use HttpServer::ThreadPool;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs::File;
use std::thread;
use std::time::Duration;
use std::thread::Thread;

static STATUS_LINE_200: &str = "HTTP/1.1 200 OK";
static STATUS_LINE_404: &str = "HTTP/1.1 404 Not found";

fn main() {
    let addr = "127.0.0.1:7878";
    let listener = TcpListener::bind(addr).unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection Established!");

        pool.execute(||
            handle_connection(stream));

    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get_request = b"GET / HTTP/1.1\r\n";
    let sleep_request = b"GET /sleep HTTP/1.1\r\n";


    let (file_path, status_line) = if buffer.starts_with(get_request) {
        ("index.html", STATUS_LINE_200)
    } else if buffer.starts_with(sleep_request) {
        thread::sleep(Duration::from_secs(5));
        ("index.html", STATUS_LINE_200)
    }
    else {
        ("404.html", STATUS_LINE_404)
    };

    write_response(&mut stream, file_path, status_line);
}

fn write_response(stream: &mut TcpStream, file_path: &str, status_line: &str) {
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}\r\n\r\n{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}