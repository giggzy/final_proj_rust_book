use http_server_example::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

const PORT: u16 = 8080;
const DATA_DIR: &str = "data/";
fn main() {
    let listener = TcpListener::bind(format!("localhost:{}", PORT)).unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(3) {
        let stream = stream.unwrap();
        eprintln!("Connection established");

        pool.execute(|| {
            eprintln!("Thread started");
            handle_connection(stream);
            eprintln!("Thread finished");
        });
    }
    eprintln!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_requests: Vec<_> = buf_reader
        .lines()
        .map(|l| l.unwrap())
        .take_while(|l| !l.is_empty())
        .collect();

    //println!("Request: {:#?}", http_requests);

    // Get the status line of the request, the first line
    //status_line = http_requests[0].split_whitespace().collect()[0];
    //println!("Status line: {:#?}", status_line);

    // parse string like "GET / HTTP/1.1" into Method, Path, Protocol variables
    let mut request = http_requests[0].split_whitespace();
    let request_method = request.next().unwrap();
    let request_path = request.next().unwrap();
    let _request_protocol = request.next().unwrap();

    // print out method, path, protocol
    eprintln!("Method: {}, File: {}", request_method, request_path);

    //let request_method = http_requests[0].split_whitespace().next().unwrap();
    //println!("Method: {:#?}", request_method);

    let (status_line, filename) = if request_method == "GET" {
        match request_path {
            "/" => ("HTTP/1.1 200 OK", "hello.html"),
            "/sleep" => {
                thread::sleep(Duration::from_secs(5));
                ("HTTP/1.1 200 OK", "hello.html")
            }
            _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
        }
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(format!("{}{}", DATA_DIR, filename)).unwrap();
    let len = contents.len();
    //let path = DATA_DIR.to_string() + "/hello.html";

    //let status_line = "HTTP/1.1 200 OK";
    //let contents = fs::read_to_string(path).unwrap();

    let response = format!(
        "{status_line}\r\n\
        Content-Length: {len}\r\n\r\n\
        {contents}"
    );

    stream.write(response.as_bytes()).unwrap();
}
