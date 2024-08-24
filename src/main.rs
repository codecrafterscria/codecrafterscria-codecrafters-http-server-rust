// Uncomment this block to pass the first stage
use std::{collections::HashMap, io::{Read, Write}, net::TcpListener};

struct Request {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: String,
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
                let not_found_response = "HTTP/1.1 404 Not Found\r\n\r\n";
                let buf: &mut[u8] = &mut [0; 1024];
                s.read(buf).unwrap();
                let buf_vec: Vec<u8> = buf.to_vec();
                let str_req = String::from_utf8_lossy(&buf_vec).to_string();
                println!("str req: {}", str_req);
                let req = parse_request(str_req);
                println!("method: {}", req.method);
                println!("path: {}", req.path);
                if req.path == "/" {
                  s.write(ok_response.as_bytes()).unwrap();
                }  else if req.path.starts_with("/echo/") {
                  let echo = extract_echo(req.path);
                  let res = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", echo.len(), echo);
                  s.write(res.as_bytes()).unwrap();
                } else if req.path == "/user-agent" {
                  let value = req.headers.get("user-agent").unwrap();
                  let res = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", value.len(), value);
                  println!("res: {}", res);
                  s.write(res.as_bytes()).unwrap();
                } else {
                  s.write(not_found_response.as_bytes()).unwrap();
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn extract_echo(path:String) -> String {
  let echo = path.strip_prefix("/echo/").unwrap();
  return echo.into();
}

fn parse_request(request: String) -> Request {
  let mut req_split = request.split("\r\n\r\n");
  let path_headers = req_split.next().unwrap();
  let body = req_split.next().unwrap().into();

  let mut path_headers_split = path_headers.split("\r\n");
  let method_path = path_headers_split.next().unwrap();
  let mut method_path_split = method_path.split(" ");

  let method = method_path_split.next().unwrap().into();
  let path = method_path_split.next().unwrap().into();

  let mut headers: HashMap<String, String> = HashMap::new();

  for header in path_headers_split {
    let mut header_iter = header.split(": ");
    let key = header_iter.next().unwrap().to_lowercase();
    let value = header_iter.next().unwrap().to_lowercase();
    println!("header: {}", header);
    println!("key: {}", key);
    println!("value: {}", value);
    headers.insert(String::from(key), String::from(value));
  }

  Request{ method, path, headers, body: body }
}
