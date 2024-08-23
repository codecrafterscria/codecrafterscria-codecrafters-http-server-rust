// Uncomment this block to pass the first stage
use std::{io::{Read, Write}, net::TcpListener};

struct Request {
    pub method: String,
    pub path: String,
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => {
                println!("accepted new connection");
                let ok_response = "HTTP/1.1 200 OK\r\n\r\n";
                let not_found_response = "HTTP/1.1 404 Not Foudn\r\n\r\n";
                let buf: &mut[u8] = &mut [0; 1024];
                s.read(buf).unwrap();
                let buf_vec: Vec<u8> = buf.to_vec();
                let str_req = String::from_utf8_lossy(&buf_vec).to_string();
                println!("str req: {}", str_req);
                let req = parse_request(str_req);
                println!("method: {}", req.method);
                println!("path: {}", req.path);
                match req.path.as_str() {
                    "/" => {
                      s.write(ok_response.as_bytes()).unwrap();
                    }
                    _ => {
                      s.write(not_found_response.as_bytes()).unwrap();
                    }
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn parse_request(request: String) -> Request {
  println!("rr: {}", request);
  let method_path_version = request.split("\r\n").next().unwrap();
  let mut method_path_iter = method_path_version.split(" ");
  let method = method_path_iter.next().unwrap().into();
  println!("m: {}", method);
  let path = method_path_iter.next().unwrap().into();
  Request{ method, path }
}
